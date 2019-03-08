extern crate storm;
#[macro_use]
extern crate log;
extern crate env_logger;

use log::Level;
use std::thread;
use storm::wayland::*;

fn main() {
    env_logger::init();

    let mut client = storm::client::Client::connect(None);
    println!("Connected to display");
    client.bind_obj::<WlRegistry>(2);
    client.get_display().get_registry(2);
    println!("Get Registry at id 2");

    loop {}
    client.disconnect();

    println!("Disconnected from display");
}
