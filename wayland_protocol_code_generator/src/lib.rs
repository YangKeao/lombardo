extern crate proc_macro;
extern crate wayland_protocol_scanner;
extern crate heck;

use proc_macro::TokenStream;
use wayland_protocol_scanner::ProtocolChild;
use wayland_protocol_scanner::InterfaceChild;
use wayland_protocol_scanner::EventOrRequestEvent;
use heck::CamelCase;

#[proc_macro]
pub fn generate_wayland_protocol_code(_item: TokenStream) -> TokenStream {
    let protocol = wayland_protocol_scanner::parse_wayland_protocol();

    let mut codes = String::from(r#"
    type NewId=u32;
    type Uint=u32;
    type Int=i32;
    type Fd=i32;
    type Object=u32;
    "#);
    for item in protocol.items {
        match item {
            ProtocolChild::Interface(interface) => {
                codes = format!(r#"
                {}
                trait {} {{
                "#, codes, interface.name);

                for msg in interface.items {
                    match msg {
                        InterfaceChild::Request(req) => {
                            codes = format!(r#"
                            {}
                            fn {}(
                            "#, codes, if req.name == "move" {"mv"} else {&req.name});
                            for child in req.items {
                                match child {
                                    EventOrRequestEvent::Arg(arg) => {
                                        codes = format!("{}{}: {},",codes, arg.name, arg.typ.to_camel_case());
                                    }
                                    _ => {}
                                }
                            }
                            codes = format!(r#"
                            {}
                            ) {{}}
                            "#, codes);
                            // println!("{}", codes);
                        }
                        InterfaceChild::Event(ev) => {
                            codes = format!(r#"
                            {}
                            fn {}() {{}}
                            "#, codes, if ev.name == "move" {"mv"} else {&ev.name});
                            // println!("{}", codes);
                        }
                        InterfaceChild::Enum(en) => {
                        }
                        _ => {}
                    }
                }

                codes = format!("{} }}", codes);
            }
            _ => {}
        }
    }

    println!("{}", codes);
    return codes.parse().unwrap();
}
