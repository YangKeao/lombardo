extern crate saiko;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate nix;
extern crate tempfile;

use log::Level;
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use saiko::wayland;
use saiko::wayland::*;
use std::ffi::c_void;
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};
use std::thread;
use tempfile::tempfile;

fn main() {
    env_logger::init();

    let mut client = saiko::client::Client::connect(None);
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
            wayland::WlDisplayEvent::WlDisplaydeleteIdEvent(rm_id_ev) => {
                c_client.delete_obj(rm_id_ev.id);
            }
            _ => {}
        },
        wayland::Event::WlRegistryEvent(reg_ev) => match reg_ev {
            wayland::WlRegistryEvent::WlRegistryglobalEvent(gl_ev) => {
                let wl_registry = c_client
                    .get_obj(gl_ev.sender_id)
                    .unwrap()
                    .try_get_wl_registry()
                    .unwrap();

                info!(
                    "WlRegistryGlobalEvent: Name: {}, Interface: {}",
                    gl_ev.name, gl_ev.interface
                );
                if gl_ev.interface == "wl_compositor" {
                    let mut obj_id = c_wl_compositor_id.lock().unwrap();
                    *obj_id = c_client.new_obj::<WlCompositor>();
                    wl_registry.bind(
                        gl_ev.name,
                        String::from("wl_compositor"),
                        gl_ev.version,
                        *obj_id,
                    );
                } else if gl_ev.interface == "wl_shell" {
                    let mut obj_id = c_wl_shell_id.lock().unwrap();
                    *obj_id = c_client.new_obj::<WlShell>();
                    wl_registry.bind(gl_ev.name, String::from("wl_shell"), gl_ev.version, *obj_id);
                } else if gl_ev.interface == "wl_shm" {
                    let mut obj_id = c_wl_shm_id.lock().unwrap();
                    *obj_id = c_client.new_obj::<WlShm>();
                    wl_registry.bind(gl_ev.name, String::from("wl_shm"), gl_ev.version, *obj_id);
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
    let wl_surface = client
        .get_obj(wl_surface_id)
        .unwrap()
        .try_get_wl_surface()
        .unwrap();

    let mut wl_shell_surface_id = client.new_obj::<WlShellSurface>();
    let c_client = client.clone();
    client.add_event_listener(Box::new(move |ev| match ev {
        wayland::Event::WlShellSurfaceEvent(wl_shell_surface_ev) => match wl_shell_surface_ev {
            wayland::WlShellSurfaceEvent::WlShellSurfacepingEvent(ping_ev) => {
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
    let wl_shell = client
        .get_obj(wl_shell_id)
        .unwrap()
        .try_get_wl_shell()
        .unwrap();
    wl_shell.get_shell_surface(wl_shell_surface_id, wl_surface_id);

    let wl_shell_surface = client
        .get_obj(wl_shell_surface_id)
        .unwrap()
        .try_get_wl_shell_surface()
        .unwrap();
    wl_shell_surface.set_toplevel();

    let width = 480;
    let height = 360;
    let size = width * height * 4;

    let (buffer_fd, buffer_file_name) = nix::unistd::mkstemp(
        &std::path::Path::new(&std::env::var("XDG_RUNTIME_DIR").unwrap())
            .join("weston-shared-XXXXXX"),
    )
    .unwrap();

    let buffer_fd_flags = FdFlag::from_bits(fcntl(buffer_fd, FcntlArg::F_GETFD).unwrap()).unwrap();
    fcntl(
        buffer_fd,
        nix::fcntl::F_SETFD(FdFlag::FD_CLOEXEC | buffer_fd_flags),
    );

    nix::unistd::unlink(&buffer_file_name);
    nix::unistd::ftruncate(buffer_fd, size).unwrap();
    let mut shm_data = unsafe {
        std::slice::from_raw_parts_mut(
            nix::sys::mman::mmap(
                std::ptr::null::<c_void>() as *mut c_void,
                size as usize,
                nix::sys::mman::ProtFlags::PROT_READ | nix::sys::mman::ProtFlags::PROT_WRITE,
                nix::sys::mman::MapFlags::MAP_SHARED,
                buffer_fd,
                0,
            )
            .unwrap() as *mut u32,
            (width * height) as usize,
        )
    };
    for i in 0..(width * height) as usize {
        shm_data[i] = 0xffff;
    }

    let dup_fd =
        fcntl(buffer_fd, nix::fcntl::F_DUPFD_CLOEXEC(0)).unwrap() as std::os::unix::io::RawFd;
    let wl_shm = client.get_obj(wl_shm_id).unwrap().try_get_wl_shm().unwrap();
    let wl_shm_pool_id = client.new_obj::<WlShmPool>();
    wl_shm.create_pool(wl_shm_pool_id, dup_fd, size as i32);
    let wl_shm_pool = client
        .get_obj(wl_shm_pool_id)
        .unwrap()
        .try_get_wl_shm_pool()
        .unwrap();
    nix::unistd::close(dup_fd);

    let wl_buffer = client.new_obj::<WlBuffer>();
    wl_shm_pool.create_buffer(
        wl_buffer,
        0,
        width as i32,
        height as i32,
        (width * 4) as i32,
        1,
    );

    wl_surface.attach(wl_buffer, 0, 0);
    wl_surface.commit();

    let wl_callback = client.new_obj::<WlCallback>();
    let c_wl_callback = wl_callback.clone();
    let c_client = client.clone();
    client.add_event_listener(Box::new(move |ev| match ev {
        wayland::Event::WlCallbackEvent(callback_ev) => match callback_ev {
            wayland::WlCallbackEvent::WlCallbackdoneEvent(done) => {
                if done.sender_id == c_wl_callback {
                    info!("Redraw");
                }
            }
            _ => {}
        },
        _ => {}
    }));
    wl_surface.frame(wl_callback);

    client.sync();
    loop {}
    client.disconnect();

    println!("Disconnected from display");
}
