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

    let mut c_client = client.clone();
    client.add_event_listener(Box::new(move |event| match event {
        wayland::Event::WlRegistryEvent(reg_ev) => match reg_ev {
            wayland::WlRegistryEvent::WlRegistryGlobalEvent(gl_ev) => {
                info!(
                    "WlRegistryGlobalEvent: Name: {}, Interface: {}",
                    gl_ev.name, gl_ev.interface
                );

                if gl_ev.interface == "wl_compositor" {
                    c_client.bind_obj::<WlCompositor>(3);
                    c_client
                        .get_obj(gl_ev.sender_id)
                        .unwrap()
                        .try_get_wl_registry()
                        .unwrap()
                        .bind(gl_ev.name, String::from("wl_compositor"), gl_ev.version, 3);
                } else if gl_ev.interface == "wl_shell" {
                    c_client.bind_obj::<WlShell>(4);
                    c_client
                        .get_obj(gl_ev.sender_id)
                        .unwrap()
                        .try_get_wl_registry()
                        .unwrap()
                        .bind(gl_ev.name, String::from("wl_shell"), gl_ev.version, 4);
                }

                if c_client.get_obj(4).is_some() && c_client.get_obj(3).is_some() {
                    c_client.bind_obj::<WlSurface>(5);
                    c_client
                        .get_obj(3)
                        .unwrap()
                        .try_get_wl_compositor()
                        .unwrap()
                        .create_surface(5);

                    c_client.bind_obj::<WlShellSurface>(6);
                    c_client
                        .get_obj(4)
                        .unwrap()
                        .try_get_wl_shell()
                        .unwrap()
                        .get_shell_surface(6, 5);

                    c_client
                        .get_obj(6)
                        .unwrap()
                        .try_get_wl_shell_surface()
                        .unwrap()
                        .set_toplevel();
                }
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
