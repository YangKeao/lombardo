#![recursion_limit="128"]

extern crate proc_macro;
extern crate wayland_protocol_scanner;
extern crate heck;
#[macro_use]
extern crate quote;

use wayland_protocol_scanner::ProtocolChild;
use wayland_protocol_scanner::InterfaceChild;
use wayland_protocol_scanner::EventOrRequestEvent;
use heck::CamelCase;
use proc_macro2::{Ident, Span};

pub fn generate_wayland_protocol_code() -> String {
    let protocol = wayland_protocol_scanner::parse_wayland_protocol();

    // Required USE
    let mut code = quote! {
        use super::socket::*;
        use std::sync::Arc;
        use std::mem::transmute;
        use std::mem::size_of;
        use std::os::unix::net::UnixStream;

        type NewId=u32;
        type Uint=u32;
        type Int=i32;
        type Fd=i32;
        type Object=u32;
        type Fixed=i32; // TODO: handle fixed value
        type Array=Vec<u32>;
    };

    // Generate Traits
    for item in &protocol.items {
        match item {
            ProtocolChild::Interface(interface) => {
                let functions = interface.items.iter().map(|msg| {
                    match msg {
                        InterfaceChild::Request(req) => {
                            let args = req.items.iter().filter_map(|child| {
                                match child {
                                    EventOrRequestEvent::Arg(arg) => {
                                        let arg_name = Ident::new(&arg.name, Span::call_site());
                                        let arg_typ = Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                        Some(quote! {#arg_name: #arg_typ})
                                    }
                                    _ => { None }
                                }
                            });
                            let function_name = Ident::new(if req.name == "move" { "mv" } else { &req.name }, Span::call_site());
                            return quote! {fn #function_name(&self, #(#args),*);};
                        }
                        InterfaceChild::Event(_ev) => { quote! {} }
                        InterfaceChild::Enum(_en) => { quote! {} }
                        _ => { quote! {} }
                    }
                });
                let interface_name = Ident::new(&format!("I{}", interface.name.to_camel_case()), Span::call_site());
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
                let interface_name = Ident::new(&format!("I{}", interface.name.to_camel_case()), Span::call_site());
                let pre_structs = interface.items.iter().filter_map(|msg| {
                    match msg {
                        InterfaceChild::Request(req) => {
                            let fields = req.items.iter().filter_map(|child| {
                                match child {
                                    EventOrRequestEvent::Arg(arg) => {
                                        let arg_name = Ident::new(&arg.name, Span::call_site());
                                        let arg_type = Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                        Some(quote! {#arg_name: #arg_type})
                                    }
                                    _ => { None }
                                }
                            });
                            let pre_struct_name = Ident::new(&format!("{}{}Request", struct_name, req.name.to_camel_case()), Span::call_site());

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
                        _ => { None }
                    }
                });
                let mut req_op_code_count: u16 = 0;
                let functions = interface.items.iter().filter_map(|msg| {
                    match msg {
                        InterfaceChild::Request(req) => {
                            let pre_struct_name = Ident::new(&format!("{}{}Request", struct_name, req.name.to_camel_case()), Span::call_site());

                            let args = req.items.iter().filter_map(|child| {
                                match child {
                                    EventOrRequestEvent::Arg(arg) => {
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
                                    EventOrRequestEvent::Arg(arg) => {
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

    let mut predefine_event_structs = quote!{};
    for item in protocol.items.iter() {
        match item {
            ProtocolChild::Interface(interface) => {
                for child in interface.items.iter() {
                    match child {
                        InterfaceChild::Event(ev) => {
                            let interface_event_name = Ident::new(&format!("{}{}Event", interface.name.to_camel_case(), ev.name.to_camel_case()), Span::call_site());
                            let event_fields = ev.items.iter().filter_map(|field| {
                                match field {
                                    EventOrRequestEvent::Arg(arg) => {
                                        let arg_name = Ident::new(&arg.name, Span::call_site());
                                        let arg_typ = Ident::new(&arg.typ.to_camel_case(), Span::call_site());
                                        Some(quote! {#arg_name: #arg_typ})
                                    }
                                    _ => {None}
                                }
                            });
                            predefine_event_structs = quote! {
                                #predefine_event_structs
                                struct #interface_event_name {
                                    #[allow(dead_code)]
                                    pub sender_id: u32,
                                    #(#[allow(dead_code)]#event_fields),*
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
    let interface_event_enums = protocol.items.iter().filter_map(|item| {
        match item {
            ProtocolChild::Interface(interface) => {
                let interface_event_name = Ident::new(&format!("{}Event", interface.name.to_camel_case()), Span::call_site());
                let interface_event_events = interface.items.iter().filter_map(|child| {
                    match child {
                        InterfaceChild::Event(ev) => {
                            let interface_event_name = Ident::new(&format!("{}{}Event", interface.name.to_camel_case(), ev.name.to_camel_case()), Span::call_site());
                            Some(quote! {#interface_event_name(#interface_event_name)})
                        }
                        _ => {None}
                    }
                });
                Some(quote! {
                    enum #interface_event_name {
                        #(#interface_event_events),*
                    }
                })
            }
            _ => {None}
        }
    });
    let interface_event_names = protocol.items.iter().filter_map(|item| {
        match item {
            ProtocolChild::Interface(interface) => {
                let interface_event_name = Ident::new(&format!("{}Event", interface.name.to_camel_case()), Span::call_site());
                Some(quote! {
                    #interface_event_name(#interface_event_name)
                })
            }
            _ => {None}
        }
    });
    code = quote! {
        #code
        #predefine_event_structs
        #(#interface_event_enums)*
        enum Event {
            #(#interface_event_names),*
        }
        trait ReadEvent {
            fn read_event(&mut self) ;
        }
        impl ReadEvent for UnixStream {
            fn read_event(&mut self) {
                
            }
        }
    };
    return code.to_string();
}
