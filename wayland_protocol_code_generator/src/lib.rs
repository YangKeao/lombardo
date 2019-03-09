#![recursion_limit = "256"]

extern crate heck;
extern crate proc_macro;
extern crate wayland_protocol_scanner;
#[macro_use]
extern crate quote;

use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, Span};
use wayland_protocol_scanner::EventOrRequestField;
use wayland_protocol_scanner::InterfaceChild;
use wayland_protocol_scanner::ProtocolChild;

pub fn generate_wayland_protocol_code() -> String {
    let protocol = wayland_protocol_scanner::parse_wayland_protocol();

    // Required USE
    let mut code = quote! {
        use super::socket::*;
        use std::sync::Arc;
        use std::mem::transmute;
        use std::mem::size_of;
        use std::os::unix::net::UnixStream;
        use std::io::Read;

        type NewId=u32;
        type Uint=u32;
        type Int=i32;
        type Fd=i32;
        type Object=u32;
        type Fixed=f32; // TODO: handle fixed value
        type Array=Vec<u32>;
    };

    // Generate Traits
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
    }

    // Generate Request interface structs and functions.
    for item in &protocol.items {
        match item {
            ProtocolChild::Interface(interface) => {
                let struct_name = Ident::new(&interface.name.to_camel_case(), Span::call_site());
                let interface_name = Ident::new(
                    &format!("I{}", interface.name.to_camel_case()),
                    Span::call_site(),
                );
                let pre_structs = interface.items.iter().filter_map(|msg| match msg {
                    InterfaceChild::Request(req) => {
                        let fields = req.items.iter().filter_map(|child| match child {
                            EventOrRequestField::Arg(arg) => {
                                let arg_name = Ident::new(&arg.name, Span::call_site());
                                let arg_type =
                                    Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                Some(quote! {#arg_name: #arg_type})
                            }
                            _ => None,
                        });
                        let pre_struct_name = Ident::new(
                            &format!("{}{}Request", struct_name, req.name.to_camel_case()),
                            Span::call_site(),
                        );

                        Some(quote! {
                            #[repr(packed)]
                            struct #pre_struct_name {
                                #[allow(dead_code)]
                                sender_id: u32,
                                #[allow(dead_code)]
                                op_code: u16,
                                #[allow(dead_code)]
                                msg_size: u16,
                                #(#[allow(dead_code)]#fields),*
                            }
                        })
                    }
                    _ => None,
                });
                let mut req_op_code_count: u16 = 0;
                let functions = interface.items.iter().filter_map(|msg| {
                    match msg {
                        InterfaceChild::Request(req) => {
                            let pre_struct_name = Ident::new(&format!("{}{}Request", struct_name, req.name.to_camel_case()), Span::call_site());

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
                            let set_fields = req.items.iter().filter_map(|child| {
                                match child {
                                    EventOrRequestField::Arg(arg) => {
                                        let arg_name = Ident::new(&arg.name, Span::call_site());
                                        Some(quote! {#arg_name})
                                    }
                                    _ => { None }
                                }
                            });

                            req_op_code_count += 1;
                            let op_code = req_op_code_count - 1;
                            Some(quote! {
                                fn #function_name(&self, #(#args),*) {
                                    let buffer = #pre_struct_name {
                                        sender_id: self.object_id,
                                        msg_size: size_of::<#pre_struct_name>() as u16,
                                        op_code: #op_code,
                                        #(#set_fields),*
                                    };
                                    self.socket.send(unsafe { &transmute::<#pre_struct_name, [u8; size_of::<#pre_struct_name>()]>(buffer) });
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
                    #(#pre_structs)*
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

    // Generate Event interface structs and functions.
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
            let interface_event_events = interface.items.iter().filter_map(|child| match child {
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
            });
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
            fn read_event(&mut self) -> (EventHeader, Vec<u8>) ;
        }
        impl ReadEvent for UnixStream {
            fn read_event(&mut self) -> (EventHeader, Vec<u8>) {
                let mut event_header: [u8; size_of::<EventHeaderPre>()] = [0; size_of::<EventHeaderPre>()];
                self.read(&mut event_header).unwrap(); // TODO: Handle Error

                let event_header = unsafe {
                    transmute::<[u8; size_of::<EventHeaderPre>()], EventHeaderPre>(event_header).convert_to_event_header()
                };

                let mut msg_body = vec![0; event_header.msg_size as usize];
                self.read(&mut msg_body).unwrap(); // TODO: Handle Error

                return (event_header, msg_body);
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
//                                        Some(quote! {
//                                            let #arg_name = String::new();
//                                            unimplemented!();
//                                        })
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
                                            let #arg_name = std::str::from_utf8(&tmp_ptr).unwrap().to_string();
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
                                            let mut #arg_name = unsafe{
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
                            info!("Receive event {}", #ev_name_str); // TODO: parse Event
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
                        _ => panic!("No such op_code")
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
    return code.to_string();
}
