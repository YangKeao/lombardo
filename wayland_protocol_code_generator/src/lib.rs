extern crate proc_macro;
extern crate wayland_protocol_scanner;
extern crate heck;
extern crate quote;

use wayland_protocol_scanner::ProtocolChild;
use wayland_protocol_scanner::InterfaceChild;
use wayland_protocol_scanner::EventOrRequestEvent;
use heck::CamelCase;

pub fn generate_wayland_protocol_code() -> String {
    let protocol = wayland_protocol_scanner::parse_wayland_protocol();

    let mut codes = String::from(r#"
    use super::socket::*;
    use std::sync::Arc;
    use std::sync::RwLock;
    use std::mem::transmute;
    use std::mem::size_of;

    type NewId=u32;
    type Uint=u32;
    type Int=i32;
    type Fd=i32;
    type Object=u32;
    "#);
    for item in &protocol.items {
        match item {
            ProtocolChild::Interface(interface) => {
                codes = format!(r#"
                {}
                trait {} {{
                "#, codes, interface.name);

                for msg in &interface.items {
                    match msg {
                        InterfaceChild::Request(req) => {
                            codes = format!(r#"
                            {}
                            fn {}(&self,
                            "#, codes, if req.name == "move" { "mv" } else { &req.name });
                            for child in &req.items {
                                match child {
                                    EventOrRequestEvent::Arg(arg) => {
                                        codes = format!("{}{}: {},", codes, arg.name, arg.typ.to_camel_case());
                                    }
                                    _ => {}
                                }
                            }
                            codes = format!(r#"
                            {}
                            );
                            "#, codes);
                        }
                        InterfaceChild::Event(_ev) => {
                            // Todo: Set Event
                        }
                        InterfaceChild::Enum(_en) => {}
                        _ => {}
                    }
                }

                codes = format!("{} }}", codes);
            }
            _ => {}
        }
    }

    for item in &protocol.items {
        match item {
            ProtocolChild::Interface(interface) => {
                codes = format!(r#"
                {}
                pub struct {} {{
                    object_id: u32,
                    socket: Arc<WaylandSocket>,
                }}
                impl {} for {} {{
                "#, codes, interface.name.to_camel_case(), interface.name, interface.name.to_camel_case());

                for msg in &interface.items {
                    match msg {
                        InterfaceChild::Request(req) => {
                            {
                                let mut structs = String::from(format!(r#"
                                #[repr(packed)]
                                struct {}{}Request {{
                                "#, interface.name.to_camel_case(), req.name.to_camel_case()));
                                for child in &req.items {
                                    match child {
                                        EventOrRequestEvent::Arg(arg) => {
                                            structs = format!("{}{}: {},", structs, arg.name, arg.typ.to_camel_case());
                                        }
                                        _ => {}
                                    }
                                }
                                structs = format!("{} }}", structs);
                                codes = format!("{}{}", structs, codes);
                            }

                            {
                                codes = format!(r#"
                                {}
                                fn {}(&self,
                                "#, codes, if req.name == "move" { "mv" } else { &req.name });
                                for child in &req.items {
                                    match child {
                                        EventOrRequestEvent::Arg(arg) => {
                                            codes = format!("{}{}: {},", codes, arg.name, arg.typ.to_camel_case());
                                        }
                                        _ => {}
                                    }
                                }
                            }

                            {
                                let mut implementations = String::from(format!(r#"
                                    let buffer = {}{}Request {{
                                "#, interface.name.to_camel_case(), req.name.to_camel_case()));
                                for child in &req.items {
                                    match child {
                                        EventOrRequestEvent::Arg(arg) => {
                                            implementations = format!("{} {}:{}, ", implementations, arg.name, arg.name);
                                        }
                                        _ => {}
                                    }
                                }
                                implementations = format!("{} }};", implementations);
                                codes = format!(r#"
                                {}
                                ){{
                                    {}
                                    self.socket.send(unsafe {{ &transmute::<{}{}Request, [u8; size_of::<{}{}Request>()]>(buffer) }});
                                }}
                                "#, codes, implementations, interface.name.to_camel_case(), req.name.to_camel_case(), interface.name.to_camel_case(), req.name.to_camel_case());
                            }
                        }
                        InterfaceChild::Event(_ev) => {
                            // Todo: Set Event
                        }
                        InterfaceChild::Enum(_en) => {}
                        _ => {}
                    }
                }

                codes = format!("{} }}", codes);
            }
            _ => {}
        }
    }

    return codes;
}
