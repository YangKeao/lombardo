extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_wayland_protocol_code(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
