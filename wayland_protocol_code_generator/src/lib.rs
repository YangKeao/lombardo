#![recursion_limit = "256"]

extern crate heck;
extern crate proc_macro2;
extern crate wayland_protocol_scanner;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;

use wayland_protocol_scanner::{Protocol, ProtocolChild, Interface, InterfaceChild, EventOrRequestField};
use proc_macro2::{Ident, Span, TokenStream};
use heck::{CamelCase, SnakeCase};

lazy_static! {
    static ref PROTOCOL: Protocol = wayland_protocol_scanner::parse_wayland_protocol();
}

fn escape_name(name: &String) -> String {
    if name == "move" {
        String::from("mv")
    } else {
        name.clone()
    }
}

fn construct_indent_from_string(str: &String) -> Ident {
    Ident::new(str, Span::call_site())
}

enum Case {
    CamelCase,
    SnakeCase,
}

fn construct_ident_from_str_and_case(str: &String, case: Option<Case>) -> Ident {
    match case {
        Some(case) => match case {
            Case::CamelCase => construct_indent_from_string(&str.to_camel_case()),
            Case::SnakeCase => construct_indent_from_string(&str.to_snake_case()),
        },
        None => construct_indent_from_string(&str),
    }
}

macro_rules! ident {
    ($t:expr, $( $s:expr ),*; $c: expr) => (construct_ident_from_str_and_case(&format!($t, $(escape_name(&$s)),*), $c))
}

macro_rules! generate_arguments {
    ($re:expr) => {
        $re.items.iter().filter_map(|child| match child {
            EventOrRequestField::Arg(arg) => {
                let arg_name = ident!("{}", arg.name; None);
                let arg_typ = ident!("{}", arg.typ; Some(Case::CamelCase));
                Some(quote! {#arg_name: #arg_typ})
            }
            _ => None,
        })
    }
}

fn add_arg_size(arg: &wayland_protocol_scanner::Arg) -> Option<TokenStream> {
    let arg_name = ident!("{}",arg.name; None);
    let arg_typ = ident!("{}", &arg.typ; Some(Case::CamelCase));
    match &arg.typ[..] {
        "String" => Some(quote! {
            raw_size += ((#arg_name.len() + 1) as f64 / 4.0).ceil() as usize * 4 + 4;
        }),
        "Fd" => None,
        // TODO: Array and other types
        _ => Some(quote! {raw_size += size_of::<#arg_typ>();}),
    }
}

fn send_arg(arg: &wayland_protocol_scanner::Arg) -> Option<TokenStream> {
    let arg_name = ident!("{}",arg.name; None);
    let arg_typ = ident!("{}", &arg.typ; Some(Case::CamelCase));
    match &arg.typ.to_camel_case()[..] {
        "String" => Some(quote! {
            let str_len = #arg_name.len();
            let buf_len = ((#arg_name.len() + 1) as f64 / 4.0).ceil() as usize * 4;
            unsafe {
                std::ptr::copy(&buf_len as *const usize as *const u8, &mut send_buffer[written_len] as *mut u8, str_len + 1);
                std::ptr::copy(&#arg_name.into_bytes()[0] as *const u8, &mut send_buffer[written_len + 4] as *mut u8, str_len);
            }
            #[allow(unused)]
            written_len += buf_len + 4;
        }),
        "Fd" => Some(quote! {
            info!("Send FD: {}", #arg_name);
            send_fd[send_fd_num] = #arg_name;
            send_fd_num += 1;
        }),
        // TODO: Array and other types
        _ => Some(quote! {
            unsafe {
                std::ptr::copy(&#arg_name as *const #arg_typ, &mut send_buffer[written_len] as *mut u8 as *mut #arg_typ, 1);
            }
            #[allow(unused)]
            written_len += size_of::<u32>();
        }),
    }
}

fn parse_args(arg: &wayland_protocol_scanner::Arg) -> Option<TokenStream> {
    let arg_name = ident!("{}", arg.name; Some(Case::SnakeCase));
    let arg_typ = ident!("{}", arg.typ; Some(Case::CamelCase));
    match &arg.typ[..] {
        "fixed" => Some(quote! {
            let #arg_name: f32 = 0.0;
            warn!("Fixed value has not been implemented");
        }),
        "string" => Some(quote! {
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
        }),
        "array" => Some(quote! {
            let #arg_name: Vec<u32> = Vec::new();
            warn!("Array value has not been implemented");
        }),
        _ => Some(quote! {
            parsed_len += size_of::<#arg_typ>();
            let start = parsed_len - size_of::<#arg_typ>();

            let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const #arg_typ;
            let #arg_name = unsafe{
                *raw_ptr
            };
        }),
    }
}

fn generate_code_for_interface(interface: &Interface) -> TokenStream {
    let struct_name = ident!("{}", interface.name; Some(Case::CamelCase));

    let mut req_op_code: i32 = -1;
    let send_req_functions = interface.items.iter().filter_map(|msg| match msg {
        InterfaceChild::Request(req) => {
            req_op_code += 1;
            let args = generate_arguments!(req);
            let function_name = ident!("{}", &req.name; None);

            let add_raw_size = req.items.iter().filter_map(|child| {
                match child {
                    EventOrRequestField::Arg(arg) => {
                        add_arg_size(arg)
                    }
                    _ => { None }
                }
            });
            let send_args = req.items.iter().filter_map(|child| {
                match child {
                    EventOrRequestField::Arg(arg) => {
                        send_arg(arg)
                    }
                    _ => { None }
                }
            });

            Some(quote! {
                pub fn #function_name(&self, #(#args),*) {
                    #[allow(unused)]
                    let mut raw_size = 8;
                    #(#add_raw_size)*
                    let mut send_buffer: Vec<u8> = vec![0; raw_size];
                    let mut send_fd = vec![0; 16];

                    #[allow(unused)]
                    let mut send_fd_num = 0;
                    unsafe {
                        std::ptr::copy(&self.object_id as *const u32, &mut send_buffer[0] as *mut u8 as *mut u32, 1);
                        let op_code_and_length: u32 = ((raw_size as u32) << 16) + (#req_op_code as u32);
                        std::ptr::copy(&op_code_and_length as *const u32, &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32, 1);
                    }

                    #[allow(unused)]
                    let mut written_len: usize = 8;
                    #(#send_args)*
                    unsafe {
                        send_fd.set_len(send_fd_num);
                    }
                    self.socket.send(&send_buffer, &send_fd);
                }
            })
        }
        _ => None
    });

    let mut ev_op_code: i32 = -1;
    let parse_ev = interface.items.iter().filter_map(|msg| match msg {
        InterfaceChild::Event(ev) => {
            ev_op_code += 1;
            let op_code = ev_op_code as u16;

            let ev_name_str = format!(
                "{}{}Event",
                interface.name.to_camel_case(),
                ev.name.to_camel_case()
            );
            let ev_interface_name = ident!("{}Event", interface.name; Some(Case::CamelCase));
            let ev_name = ident!("{}{}Event", interface.name, ev.name; Some(Case::CamelCase));

            let parse_args = ev.items.iter().filter_map(|field| match field {
                EventOrRequestField::Arg(arg) => parse_args(arg),
                _ => None,
            });
            let arg_names = ev.items.iter().filter_map(|field| match field {
                EventOrRequestField::Arg(arg) => {
                    let arg_name = ident!("{}", arg.name; Some(Case::SnakeCase));
                    Some(quote! {#arg_name})
                }
                _ => None,
            });
            Some(quote! {
                #op_code => {
                    info!("Receive event {}", #ev_name_str);

                    #[allow(unused)]
                    let mut parsed_len: usize = 0;
                    #(#parse_args)*
                    Event::#ev_interface_name(#ev_interface_name::#ev_name(#ev_name {
                        sender_id,
                        #(#arg_names),*
                    }))
                }
            })
        }
        _ => None
    });
    quote! {
        #[derive(Clone)]
        pub struct #struct_name {
            #[allow(dead_code)]
            pub object_id: u32,
            #[allow(dead_code)]
            pub socket: Arc<WaylandSocket>,
        }
        impl WlRawObject for #struct_name {
            fn new(object_id: u32, socket: Arc<WaylandSocket>) -> #struct_name {
                #struct_name { object_id, socket }
            }
            fn to_enum(self) -> WlObject {
                WlObject::#struct_name(self)
            }
        }
        impl #struct_name {
            fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
                match op_code {
                    #(#parse_ev)*
                    _ => panic!("Unknown event")
                }
            }
            #(#send_req_functions)*
        }
    }
}

fn generate_code_for_wayland_enums() -> TokenStream {
    let enum_interface_names = PROTOCOL.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let interface_name = ident!("{}", interface.name; Some(Case::CamelCase));
            Some(quote! {#interface_name(#interface_name)})
        }
        _ => None,
    });
    let enum_event_names = PROTOCOL.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let event_name = ident!("{}Event", interface.name; Some(Case::CamelCase));
            Some(quote! {#event_name(#event_name)})
        }
        _ => None,
    });
    let wl_event_enums = PROTOCOL.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let event_enum_name = ident!("{}Event", interface.name; Some(Case::CamelCase));
            let event_structs = interface.items.iter().filter_map(|ev| match ev {
                InterfaceChild::Event(ev) => {
                    let ev_struct_enum_name = ident!("{}{}Event", interface.name, ev.name; Some(Case::CamelCase));
                    let event_fields = generate_arguments!(ev);
                    Some(quote! {
                        pub struct #ev_struct_enum_name {
                            #[allow(dead_code)]
                            pub sender_id: u32,
                            #(#[allow(dead_code)]pub #event_fields),*
                        }
                    })
                }
                _ => None
            });
            let event_struct_enum_names = interface.items.iter().filter_map(|ev| match ev {
                InterfaceChild::Event(ev) => {
                    let ev_struct_enum_name = ident!("{}{}Event", interface.name, ev.name; Some(Case::CamelCase));
                    Some(quote! {
                        #ev_struct_enum_name(#ev_struct_enum_name)
                    })
                }
                _ => None
            });
            Some(quote! {
                #(#event_structs)*
                pub enum #event_enum_name {
                    #(#event_struct_enum_names),*
                }
            })
        }
        _ => None,
    });
    let impl_wl_get_obj = PROTOCOL.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let interface_name = ident!("{}", interface.name; Some(Case::CamelCase));
            let get_function_name = ident!("try_get_{}", interface.name; Some(Case::SnakeCase));
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
    quote! {
        pub enum WlObject {
            #(#enum_interface_names),*
        }
        pub enum Event {
            #(#enum_event_names),*
        }
        #(#wl_event_enums)*
        impl WlObject {
            #(#impl_wl_get_obj)*
        }
    }
}

pub fn generate_wayland_protocol_code() -> String {
    let codes_for_every_interface = PROTOCOL.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            Some(generate_code_for_interface(interface))
        }
        _ => None
    });
    let code_for_wayland_enums = generate_code_for_wayland_enums();
    let parse_event_for_interface = PROTOCOL.items.iter().filter_map(|item| match item {
        ProtocolChild::Interface(interface) => {
            let interface_name = ident!("{}", interface.name; Some(Case::CamelCase));
            Some(quote! {
                WlObject::#interface_name(_obj) => #interface_name::parse_event(sender_id, op_code, msg_body)
            })
        }
        _ => None
    });

    let code = quote! {
        use super::socket::*;
        use crate::unix_socket::UnixSocket;
        use std::sync::Arc;
        use std::mem::transmute;
        use std::mem::size_of;

        type NewId=u32;
        type Uint=u32;
        type Int=i32;
        type Fd=i32;
        type Object=u32;
        type Fixed=f32; // TODO: handle fixed value
        type Array=Vec<u32>;


        #(#codes_for_every_interface)*
        #code_for_wayland_enums

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
            fn read_event(&mut self) -> Vec<(EventHeader, Vec<u8>)>;
        }
        impl ReadEvent for UnixSocket {
            fn read_event(&mut self) -> Vec<(EventHeader, Vec<u8>)> {
                let mut buffer: [u8; 1024] = [0; 1024];
                let mut fds: [u8; 24] = [0; 24];
                let (size, _) = self.read(&mut buffer, &mut fds);
                if size == 1024 {
                    warn!("Buffer is full");
                }
                let mut ret_value = Vec::new();
                let mut read_size: usize = 0;
                while read_size < size {
                    let mut event_header: [u8; size_of::<EventHeaderPre>()] =
                        [0; size_of::<EventHeaderPre>()];
                    unsafe {
                        std::ptr::copy(
                            &buffer[read_size] as *const u8,
                            event_header.as_mut_ptr(),
                            size_of::<EventHeaderPre>(),
                        );
                    }
                    let event_header = unsafe {
                        transmute::<[u8; size_of::<EventHeaderPre>()], EventHeaderPre>(event_header)
                            .convert_to_event_header()
                    };
                    let msg_size = event_header.msg_size as usize;
                    let mut msg_body = vec![0; event_header.msg_size as usize];
                    unsafe {
                        std::ptr::copy(
                            &buffer[read_size + size_of::<EventHeaderPre>()] as *const u8,
                            msg_body.as_mut_ptr(),
                            msg_size,
                        );
                    }
                    ret_value.push((event_header, msg_body));
                    read_size += size_of::<EventHeaderPre>() + msg_size;
                }
                return ret_value;
            }
        }
        pub trait WlRawObject {
            fn new(object_id: u32, socket: Arc<WaylandSocket>) -> Self;
            fn to_enum(self) -> WlObject;
        }
        impl WlObject {
            pub fn parse_event(&self, sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
                match self {
                    #(#parse_event_for_interface),*
                }
            }
        }
    };

    return code.to_string();
}