extern crate storm;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate nix;
extern crate tempfile;

use log::Level;
use std::ffi::c_void;
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};
use std::thread;
use storm::wayland;
use storm::wayland::*;
use tempfile::tempfile;

fn main() {
    env_logger::init();

    let mut client = storm::client::Client::connect(None);
    println!("Connected to display");

    let mut wl_compositor_id = Arc::new(Mutex::new(0));
    let mut c_wl_compositor_id = wl_compositor_id.clone();

    let mut wl_shell_id = Arc::new(Mutex::new(0));
    let mut c_wl_shell_id = wl_shell_id.clone();

    let mut wl_shm_id = Arc::new(Mutex::new(0));
    let mut c_wl_shm_id = wl_shm_id.clone();

    let mut c_client = client.clone();
    client.add_event_listener(Box::new(move |event| match event {
        wayland::Event::WlDisplayEvent(display_ev) => match display_ev {
            wayland::WlDisplayEvent::WlDisplayDeleteIdEvent(rm_id_ev) => {
                c_client.delete_obj(rm_id_ev.id);
            }
            _ => {}
        },
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
                } else if gl_ev.interface == "wl_shm" {
                    let mut obj_id = c_wl_shm_id.lock().unwrap();
                    *obj_id = c_client.new_obj::<WlShell>();
                    c_client
                        .get_obj(gl_ev.sender_id)
                        .unwrap()
                        .try_get_wl_registry()
                        .unwrap()
                        .bind(gl_ev.name, String::from("wl_shm"), gl_ev.version, *obj_id);
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
    let mut wl_shm_id = *(wl_shm_id.lock().unwrap());

    let mut wl_surface_id = client.new_obj::<WlSurface>();
    client
        .get_obj(wl_compositor_id)
        .unwrap()
        .try_get_wl_compositor()
        .unwrap()
        .create_surface(wl_surface_id);

    let mut wl_shell_surface_id = client.new_obj::<WlShellSurface>();
    let c_client = client.clone();
    client.add_event_listener(Box::new(move |ev| match ev {
        wayland::Event::WlShellSurfaceEvent(wl_shell_surface_ev) => match wl_shell_surface_ev {
            wayland::WlShellSurfaceEvent::WlShellSurfacePingEvent(ping_ev) => {
                c_client
                    .get_obj(ping_ev.sender_id)
                    .unwrap()
                    .try_get_wl_shell_surface()
                    .unwrap()
                    .pong(ping_ev.serial);
            }
            _ => {}
        },
        _ => {}
    }));
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

    let width = 480;
    let height = 360;
    let size = width * height * 4;

    let buffer_fd = tempfile().unwrap().as_raw_fd();
    unsafe {
        nix::sys::mman::mmap(
            0 as *mut c_void,
            size,
            nix::sys::mman::ProtFlags::from_bits(1 & 2).unwrap(),
            nix::sys::mman::MapFlags::from_bits(1).unwrap(),
            buffer_fd,
            0,
        );
    }

    let wl_shm_pool_id = client.new_obj::<WlShmPool>();
    client
        .get_obj(wl_shm_id)
        .unwrap()
        .try_get_wl_shm()
        .unwrap()
        .create_pool(wl_shm_pool_id, buffer_fd, size as i32);

    let wl_buffer = client.new_obj::<WlBuffer>();
    client
        .get_obj(wl_shm_pool_id)
        .unwrap()
        .try_get_wl_shm_pool()
        .unwrap()
        .create_buffer(
            wl_buffer,
            0,
            width as i32,
            height as i32,
            (width * 4) as i32,
            1,
        );

    client
        .get_obj(wl_surface_id)
        .unwrap()
        .try_get_wl_surface()
        .unwrap()
        .attach(wl_buffer, 0, 0);
    client
        .get_obj(wl_surface_id)
        .unwrap()
        .try_get_wl_surface()
        .unwrap()
        .commit();

    loop {}
    client.disconnect();

    println!("Disconnected from display");
}
