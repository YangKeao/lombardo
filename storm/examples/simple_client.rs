extern crate storm;
#[macro_use]
extern crate log;
extern crate env_logger;

use log::Level;
use std::thread;
use storm::wayland;
use storm::wayland::*;

fn main() {
    env_logger::init();

    let mut client = storm::client::Client::connect(None);
    println!("Connected to display");
    client.add_event_listener(Box::new(|event| match event {
        wayland::Event::WlRegistryEvent(reg_ev) => match reg_ev {
            wayland::WlRegistryEvent::WlRegistryGlobalEvent(rm_ev) => {
                info!(
                    "WlRegistryGlobalEvent: Name: {}, Interface: {}",
                    rm_ev.name, rm_ev.interface
                );
            }
            _ => {}
        },
        _ => {}
    }));
    client.bind_obj::<WlRegistry>(2);
    client.get_display().get_registry(2);
    println!("Get Registry at id 2");

    loop {}
    client.disconnect();

    println!("Disconnected from display");
}
