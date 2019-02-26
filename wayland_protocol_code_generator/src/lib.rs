extern crate proc_macro;
extern crate wayland_protocol_scanner;

use proc_macro::TokenStream;
use wayland_protocol_scanner::ProtocolChild;
use wayland_protocol_scanner::InterfaceChild;

#[proc_macro]
pub fn generate_wayland_protocol_code(_item: TokenStream) -> TokenStream {
    let protocol = wayland_protocol_scanner::parse_wayland_protocol();

    let mut codes = String::new();
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
                            fn {}_() {{}}
                            "#, codes, req.name);
                            // println!("{}", codes);
                        }
                        InterfaceChild::Event(ev) => {
                            codes = format!(r#"
                            {}
                            fn {}_() {{}}
                            "#, codes, ev.name);
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

    return codes.parse().unwrap();
}
