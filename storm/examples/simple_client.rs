extern crate storm;
#[macro_use]
extern crate log;
extern crate env_logger;

use log::Level;
use std::sync::{Arc, Mutex};
use std::thread;
use storm::wayland;
use storm::wayland::*;

fn main() {
    env_logger::init();

    let mut client = storm::client::Client::connect(None);
    println!("Connected to display");

    let mut wl_compositor_id = Arc::new(Mutex::new(0));
    let mut c_wl_compositor_id = wl_compositor_id.clone();

    let mut wl_shell_id = Arc::new(Mutex::new(0));
    let mut c_wl_shell_id = wl_shell_id.clone();

    let mut c_client = client.clone();
    client.add_event_listener(Box::new(move |event| match event {
        wayland::Event::WlRegistryEvent(reg_ev) => match reg_ev {
            wayland::WlRegistryEvent::WlRegistryGlobalEvent(gl_ev) => {
                info!(
                    "WlRegistryGlobalEvent: Name: {}, Interface: {}",
                    gl_ev.name, gl_ev.interface
                );
                if gl_ev.interface == "wl_compositor" {
                    let mut obj_id = c_wl_compositor_id.lock().unwrap();
                    *obj_id = c_client.new_obj::<WlCompositor>();
                    c_client
                        .get_obj(gl_ev.sender_id)
                        .unwrap()
                        .try_get_wl_registry()
                        .unwrap()
                        .bind(
                            gl_ev.name,
                            String::from("wl_compositor"),
                            gl_ev.version,
                            *obj_id,
                        );
                } else if gl_ev.interface == "wl_shell" {
                    let mut obj_id = c_wl_shell_id.lock().unwrap();
                    *obj_id = c_client.new_obj::<WlShell>();
                    c_client
                        .get_obj(gl_ev.sender_id)
                        .unwrap()
                        .try_get_wl_registry()
                        .unwrap()
                        .bind(gl_ev.name, String::from("wl_shell"), gl_ev.version, *obj_id);
                }
            }
            _ => {}
        },
        _ => {}
    }));

    client.bind_obj::<WlRegistry>(2);
    client.get_display().get_registry(2);
    println!("Get Registry at id 2");
    client.sync();
    println!("Wayland Sync");

    let mut wl_compositor_id = *(wl_compositor_id.lock().unwrap());
    let mut wl_shell_id = *(wl_shell_id.lock().unwrap());

    let mut wl_surface_id = client.new_obj::<WlSurface>();
    client
        .get_obj(wl_compositor_id)
        .unwrap()
        .try_get_wl_compositor()
        .unwrap()
        .create_surface(wl_surface_id);

    let mut wl_shell_surface_id = client.new_obj::<WlShellSurface>();
    client
        .get_obj(wl_shell_id)
        .unwrap()
        .try_get_wl_shell()
        .unwrap()
        .get_shell_surface(wl_shell_surface_id, wl_surface_id);

    client
        .get_obj(wl_shell_surface_id)
        .unwrap()
        .try_get_wl_shell_surface()
        .unwrap()
        .set_toplevel();

    client.disconnect();

    println!("Disconnected from display");
}
