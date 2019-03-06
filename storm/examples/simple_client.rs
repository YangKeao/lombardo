extern crate storm;
#[macro_use]
extern crate log;
extern crate env_logger;

use log::Level;
use storm::wayland::*;

fn main() {
    env_logger::init();

    let mut client = storm::client::Client::connect(None);
    println!("Connected to display");

    client.display.get_registry(2);
    loop {}
    client.disconnect();

    println!("Disconnected from display");
}
