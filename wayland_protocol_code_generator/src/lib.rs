#![recursion_limit = "256"]

extern crate heck;
extern crate proc_macro;
extern crate wayland_protocol_scanner;
#[macro_use]
extern crate quote;

use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use wayland_protocol_scanner::{EventOrRequestField, Protocol};
use wayland_protocol_scanner::InterfaceChild;
use wayland_protocol_scanner::ProtocolChild;
use syn::export::TokenStream2;

const protocol: Protocol = wayland_protocol_scanner::parse_wayland_protocol();

fn generate_traits(mut code: TokenStream) -> TokenStream {
    for item in &protocol.items {
        match item {
            ProtocolChild::Interface(interface) => {
                let functions = interface.items.iter().map(|msg| match msg {
                    InterfaceChild::Request(req) => {
                        let args = req.items.iter().filter_map(|child| match child {
                            EventOrRequestField::Arg(arg) => {
                                let arg_name = Ident::new(&arg.name, Span::call_site());
                                let arg_typ =
                                    Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                Some(quote! {#arg_name: #arg_typ})
                            }
                            _ => None,
                        });
                        let function_name = Ident::new(
                            if req.name == "move" { "mv" } else { &req.name },
                            Span::call_site(),
                        );
                        return quote! {fn #function_name(&self, #(#args),*);};
                    }
                    InterfaceChild::Event(_ev) => {
                        quote! {}
                    }
                    InterfaceChild::Enum(_en) => {
                        quote! {}
                    }
                    _ => {
                        quote! {}
                    }
                });
                let interface_name = Ident::new(
                    &format!("I{}", interface.name.to_camel_case()),
                    Span::call_site(),
                );
                code = quote! {
                    #code
                    pub trait #interface_name {
                        #(#functions)*
                    }
                };
            }
            _ => {}
        }
    };
    return code;
}

fn generate_req_interface_structs_and_functions(mut code: TokenStream) -> TokenStream {
    for item in &protocol.items {
        match item {
            ProtocolChild::Interface(interface) => {
                let struct_name = Ident::new(&interface.name.to_camel_case(), Span::call_site());
                let interface_name = Ident::new(
                    &format!("I{}", interface.name.to_camel_case()),
                    Span::call_site(),
                );
                let mut req_op_code_count: u16 = 0;
                let functions = interface.items.iter().filter_map(|msg| {
                    match msg {
                        InterfaceChild::Request(req) => {
                            let args = req.items.iter().filter_map(|child| {
                                match child {
                                    EventOrRequestField::Arg(arg) => {
                                        let arg_name = Ident::new(&arg.name, Span::call_site());
                                        let arg_typ = Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                        Some(quote! {#arg_name: #arg_typ})
                                    }
                                    _ => { None }
                                }
                            });
                            let function_name = Ident::new(if req.name == "move" { "mv" } else { &req.name }, Span::call_site());

                            req_op_code_count += 1;
                            let op_code = req_op_code_count - 1;
                            let add_raw_size = req.items.iter().filter_map(|child| {
                                match child {
                                    EventOrRequestField::Arg(arg) => {
                                        let arg_name = Ident::new(&arg.name, Span::call_site());
                                        let arg_typ = Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                        match &arg.typ.to_camel_case()[..] {
                                            "String" => {
                                                Some(quote! {
                                                    raw_size += ((#arg_name.len() + 1) as f64 / 4.0).ceil() as usize * 4 + 4;
                                                })
                                            }
                                            "Fd" => {
                                                None
                                            }
                                            // TODO: Array and other types
                                            _ => {
                                                Some(quote! {raw_size += size_of::<#arg_typ>();})
                                            }
                                        }
                                    }
                                    _ => { None }
                                }
                            });

                            let send_args = req.items.iter().filter_map(|child| {
                                match child {
                                    EventOrRequestField::Arg(arg) => {
                                        let arg_name = Ident::new(&arg.name, Span::call_site());
                                        let arg_typ = Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                        match &arg.typ.to_camel_case()[..] {
                                            "String" => {
                                                Some(quote! {
                                                    let str_len = #arg_name.len();
                                                    let buf_len = ((#arg_name.len() + 1) as f64 / 4.0).ceil() as usize * 4;
                                                    unsafe {
                                                        std::ptr::copy(&buf_len as *const usize as *const u8, &mut send_buffer[written_len] as *mut u8, str_len + 1);
                                                        std::ptr::copy(&#arg_name.into_bytes()[0] as *const u8, &mut send_buffer[written_len + 4] as *mut u8, str_len);
                                                    }
                                                    written_len += buf_len + 4;
                                                })
                                            }
                                            "Fd" => {
                                                Some(quote! {
                                                    info!("Send FD: {}", #arg_name);
                                                    send_fd[send_fd_num] = #arg_name;
                                                    send_fd_num += 1;
                                                })
                                            }
                                            // TODO: Array and other types
                                            _ => {
                                                Some(quote! {
                                                    unsafe {
                                                        std::ptr::copy(&#arg_name as *const #arg_typ, &mut send_buffer[written_len] as *mut u8 as *mut #arg_typ, 1);
                                                    }
                                                    written_len += size_of::<u32>();
                                                })
                                            }
                                        }
                                    }
                                    _ => { None }
                                }
                            });

                            Some(quote! {
                                fn #function_name(&self, #(#args),*) {
                                    let mut raw_size = 8;
                                    #(#add_raw_size)*
                                    let mut send_buffer: Vec<u8> = vec![0; raw_size];
                                    let mut send_fd = vec![0; 16];
                                    let mut send_fd_num = 0;
                                    unsafe {
                                        std::ptr::copy(&self.object_id as *const u32, &mut send_buffer[0] as *mut u8 as *mut u32, 1);
                                        let op_code_and_length: u32 = ((raw_size as u32) << 16) + (#op_code as u32);
                                        std::ptr::copy(&op_code_and_length as *const u32, &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32, 1);
                                    }
                                    let mut written_len: usize = 8;
                                    #(#send_args)*
                                    unsafe {
                                        send_fd.set_len(send_fd_num);
                                    }
                                    self.socket.send(&send_buffer, &send_fd);
                                }
                            })
                        }
                        InterfaceChild::Event(_ev) => { None }
                        InterfaceChild::Enum(_en) => { None }
                        _ => { None }
                    }
                });
                code = quote! {
                    #code
                    #[derive(Clone)]
                    pub struct #struct_name {
                        #[allow(dead_code)]
                        pub object_id: u32,
                        #[allow(dead_code)]
                        pub socket: Arc<WaylandSocket>,
                    }
                    impl #interface_name for #struct_name {
                        #(#functions)*
                    }
                };
            }
            _ => {}
        }
    }
    return code;
}

fn generate_get_wayland_object_functions(mut code: TokenStream) -> TokenStream {
    let interface_names = protocol.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let interface_name = Ident::new(
                &format!("{}", interface.name.to_camel_case()),
                Span::call_site(),
            );
            Some(quote! {#interface_name(#interface_name)})
        }
        _ => None,
    });
    let impl_wl_raw_obj = protocol.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let interface_name = Ident::new(
                &format!("{}", interface.name.to_camel_case()),
                Span::call_site(),
            );
            Some(quote! {
                impl WlRawObject for #interface_name {
                    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> #interface_name {
                        #interface_name {
                            object_id,
                            socket,
                        }
                    }
                    fn to_enum(self) -> WlObject {
                        WlObject::#interface_name(self)
                    }
                }
            })
        }
        _ => None,
    });
    let impl_wl_get_obj = protocol.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let interface_name = Ident::new(
                &format!("{}", interface.name.to_camel_case()),
                Span::call_site(),
            );
            let get_function_name = Ident::new(
                &format!("try_get_{}", interface.name.to_snake_case()),
                Span::call_site(),
            );
            Some(quote! {
                #[allow(dead_code)]
                pub fn #get_function_name(&self) -> Option<#interface_name> {
                    match self {
                        WlObject::#interface_name(item) => Some(item.clone()),
                        _ => None,
                    }
                }
            })
        }
        _ => None,
    });
    code = quote! {
        #code
        pub trait WlRawObject {
            fn new(object_id: u32, socket: Arc<WaylandSocket>) -> Self;
            fn to_enum(self) -> WlObject ;
        }
        pub enum WlObject {
            #(#interface_names),*
        }
        impl WlObject {
            #(#impl_wl_get_obj)*
        }
        #(#impl_wl_raw_obj)*
    };
    return code;
}

fn generate_event_interface_structs_and_functions(mut code: TokenStream) -> TokenStream {
    let mut predefine_event_structs = quote! {};
    for item in protocol.items.iter() {
        match item {
            ProtocolChild::Interface(interface) => {
                for child in interface.items.iter() {
                    match child {
                        InterfaceChild::Event(ev) => {
                            let interface_event_name = Ident::new(
                                &format!(
                                    "{}{}Event",
                                    interface.name.to_camel_case(),
                                    ev.name.to_camel_case()
                                ),
                                Span::call_site(),
                            );
                            let event_fields = ev.items.iter().filter_map(|field| match field {
                                EventOrRequestField::Arg(arg) => {
                                    let arg_name = Ident::new(&arg.name, Span::call_site());
                                    let arg_typ =
                                        Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                    Some(quote! {#arg_name: #arg_typ})
                                }
                                _ => None,
                            });
                            predefine_event_structs = quote! {
                                #predefine_event_structs
                                pub struct #interface_event_name {
                                    #[allow(dead_code)]
                                    pub sender_id: u32,
                                    #(#[allow(dead_code)]pub #event_fields),*
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    let interface_event_enums = protocol.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let interface_event_name = Ident::new(
                &format!("{}Event", interface.name.to_camel_case()),
                Span::call_site(),
            );
            let mut interface_event_events: Vec<TokenStream> = interface.items.iter().filter_map(|child| match child {
                InterfaceChild::Event(ev) => {
                    let interface_event_name = Ident::new(
                        &format!(
                            "{}{}Event",
                            interface.name.to_camel_case(),
                            ev.name.to_camel_case()
                        ),
                        Span::call_site(),
                    );
                    Some(quote! {#interface_event_name(#interface_event_name)})
                }
                _ => None,
            }).collect();
            interface_event_events.push(quote! {None});
            Some(quote! {
                enum #interface_event_name {
                    #(#interface_event_events),*
                }
            })
        }
        _ => None,
    });
    let interface_event_names = protocol.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let interface_event_name = Ident::new(
                &format!("{}Event", interface.name.to_camel_case()),
                Span::call_site(),
            );
            Some(quote! {
                #interface_event_name(#interface_event_name)
            })
        }
        _ => None,
    });
    code = quote! {
        #code
        #predefine_event_structs
        #(#[allow(dead_code)]pub #interface_event_enums)*
        pub enum Event {
            #(#interface_event_names),*
        }
        #[repr(packed)]
        struct EventHeaderPre {
            pub sender_id: u32,
            pub msg_size_and_op_code: u32,
        }
        #[repr(packed)]
        pub struct EventHeader {
            pub sender_id: u32,
            pub msg_size: u16,
            pub op_code: u16,
        }
        impl EventHeaderPre {
            fn convert_to_event_header(self) -> EventHeader {
                let msg_size = {
                    let size = self.msg_size_and_op_code >> 16;
                    if size % 4 == 0 {
                        size as u16
                    } else {
                        (size as f64 / 4.0).ceil() as u16 * 4
                    }
                } - 8;
                EventHeader {
                    sender_id: self.sender_id,
                    msg_size,
                    op_code: self.msg_size_and_op_code as u16,
                }
            }
        }
        pub trait ReadEvent {
            fn read_event(&mut self) -> Vec<(EventHeader, Vec<u8>)> ;
        }
        impl ReadEvent for UnixSocket {
            fn read_event(&mut self) -> Vec<(EventHeader, Vec<u8>)> {
                // TODO: don't reallocate buffer and fds every time
                let mut buffer: [u8; 1024] = [0; 1024];
                let mut fds: [u8; 24] = [0; 24];

                let (size, num_fds) = self.read(&mut buffer, &mut fds);
                if size == 1024 {
                    warn!("Buffer is full");
                }

                let mut ret_value = Vec::new();
                let mut read_size: usize = 0;
                while read_size < size {
                    let mut event_header: [u8; size_of::<EventHeaderPre>()] = [0; size_of::<EventHeaderPre>()];
                    unsafe {
                        std::ptr::copy(&buffer[read_size] as *const u8, event_header.as_mut_ptr(), size_of::<EventHeaderPre>());
                    }

                    let event_header = unsafe {
                        transmute::<[u8; size_of::<EventHeaderPre>()], EventHeaderPre>(event_header).convert_to_event_header()
                    };

                    let msg_size = event_header.msg_size as usize;
                    let mut msg_body = vec![0; event_header.msg_size as usize];
                    unsafe {
                        std::ptr::copy(&buffer[read_size + size_of::<EventHeaderPre>()] as *const u8, msg_body.as_mut_ptr(), msg_size);
                    }

                    ret_value.push((event_header, msg_body));
                    read_size += size_of::<EventHeaderPre>() + msg_size;
                }

                return ret_value;
            }
        }
    };

    let parse_event_for_every_interface = protocol.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let interface_name_str = format!("{}", interface.name.to_camel_case());
            let event_interface_name_str = format!("{}Event", interface.name.to_camel_case());
            let interface_name = Ident::new(
                &interface_name_str,
                Span::call_site(),
            );
            let event_interface_name = Ident::new(
                &event_interface_name_str,
                Span::call_site(),
            );
            let mut op_code: u16 = 0;
            let parse_event_for_every_op_code = interface.items.iter().filter_map(|child| match child {
                InterfaceChild::Event(ev) => {
                    op_code += 1;
                    let true_op_code =op_code - 1;

                    let ev_name_str = format!("{}{}Event", interface.name.to_camel_case(), ev.name.to_camel_case());
                    let ev_name = Ident::new(
                        &ev_name_str,
                        Span::call_site(),
                    );
                    let parse_args = ev.items.iter().filter_map(|field| {
                        match field {
                            EventOrRequestField::Arg(arg) => {
                                let arg_name_str = arg.name.to_snake_case();
                                let arg_name = Ident::new(
                                    &arg_name_str,
                                    Span::call_site(),
                                );
                                let arg_typ =
                                    Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                match &arg.typ[..] {
                                    "fixed" => {
                                        Some(quote! {
                                            let #arg_name: f32 = 0.0;
                                            unimplemented!();
                                        })
                                    }
                                    "string" => {
                                        Some(quote! {
                                            parsed_len += size_of::<u32>();
                                            let start = parsed_len - size_of::<u32>();

                                            let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const u32;
                                            let str_len = unsafe{
                                                *raw_ptr
                                            };
                                            let str_len = (str_len as f64 / 4.0).ceil() as usize * 4;
                                            parsed_len += str_len;

                                            let src_ptr = msg_body[(start + size_of::<u32>())..parsed_len].as_ptr();
                                            let mut tmp_ptr = Vec::with_capacity(str_len);
                                            unsafe {
                                                tmp_ptr.set_len(str_len);
                                                std::ptr::copy(src_ptr, tmp_ptr.as_mut_ptr(), str_len);
                                            };
                                            let #arg_name = std::str::from_utf8(&tmp_ptr).unwrap().trim_matches('\0').to_string();
                                        })
                                    }
                                    "array" => {
                                        Some(quote! {
                                            let #arg_name: Vec<u32> = Vec::new();
                                            unimplemented!();
                                        })
                                    }
                                    _ => {
                                        Some(quote! {
                                            parsed_len += size_of::<#arg_typ>();
                                            let start = parsed_len - size_of::<#arg_typ>();

                                            let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const #arg_typ;
                                            let #arg_name = unsafe{
                                                *raw_ptr
                                            };
                                        })
                                    }
                                }
                            }
                            _ => None
                        }
                    });
                    let arg_names = ev.items.iter().filter_map(|field| {
                        match field {
                            EventOrRequestField::Arg(arg) => {
                                let arg_name_str = arg.name.to_snake_case();
                                let arg_name = Ident::new(
                                    &arg_name_str,
                                    Span::call_site(),
                                );
                                Some(quote! {#arg_name})
                            }
                            _ => None
                        }
                    });
                    Some(quote! {
                        #true_op_code => {
                            info!("Receive event {}", #ev_name_str);
                            let mut parsed_len: usize = 0;
                            #(#parse_args)*
                            #event_interface_name::#ev_name(#ev_name {
                                sender_id,
                                #(#arg_names),*
                            })
                        }
                    })
                }
                _ => None
            });
            Some(quote! {
                WlObject::#interface_name(_obj) => {
                    Event::#event_interface_name(match op_code {
                        #(#parse_event_for_every_op_code)*
                        _ => {
                            warn!("No such op_code");
                            #event_interface_name::None
                        }
                    })
                }
            })
        }
        _ => None,
    });
    code = quote! {
        #code
        impl WlObject {
            pub fn parse_event(&self, sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
                match self {
                    #(#parse_event_for_every_interface)*
                }
            }
        }
    };
    return code;
}

pub fn generate_wayland_protocol_code() -> String {
    let protocol = wayland_protocol_scanner::parse_wayland_protocol();

    // Required USE
    let mut code = quote! {
        use super::socket::*;
        use crate::unix_socket::UnixSocket;
        use std::sync::Arc;
        use std::mem::transmute;
        use std::mem::size_of;
        use std::io::Read;

        type NewId=u32;
        type Uint=u32;
        type Int=i32;
        type Fd=i32;
        type Object=u32;
        type Fixed=f32; // TODO: handle fixed value
        type Array=Vec<u32>;
    };

    code = generate_traits(code);
    code = generate_req_interface_structs_and_functions(code);
    code = generate_get_wayland_object_functions(code);
    code = generate_event_interface_structs_and_functions(code);

    return code.to_string();
}
