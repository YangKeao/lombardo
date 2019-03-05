extern crate wayland_protocol_code_generator;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let dest_path = Path::new("./src/").join("wayland.rs");
    let mut file = File::create(&dest_path).unwrap();

    file.write_all(wayland_protocol_code_generator::generate_wayland_protocol_code().as_bytes())
        .unwrap();

    Command::new("cargo").arg("fmt").spawn().unwrap();
}
