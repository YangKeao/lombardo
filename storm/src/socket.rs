use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::os::unix::net::UnixStream;
use std::mem::transmute;
use std::io::Write;
use std::io::Read;

use super::wayland::*;

pub struct WaylandSocket {
    socket: Arc<RwLock<UnixStream>>,
    listen_thread: std::thread::JoinHandle<()>,
}

impl WaylandSocket {
    pub fn connect(name: Option<&str>) -> WaylandSocket {
        let default_name = std::env::var("WAYLAND_DISPLAY").unwrap_or("wayland-0".to_string());
        let name = name.unwrap_or(&default_name);

        let path = std::path::Path::new(name);
        let path = if path.is_relative() {
            std::path::Path::new(&std::env::var("XDG_RUNTIME_DIR").unwrap()).join(path)
        } else {
            path.to_path_buf()
        };

        let socket = Arc::new(RwLock::new(UnixStream::connect(path).unwrap()));

        let c_socket = socket.clone();
        let listen_thread = thread::spawn(move || {
            let mut head: [u8; 8] = [0; 8];
            loop {
                c_socket.write().unwrap().read_exact(&mut head).unwrap();

                println!("{:?}", head);
            }
        });

        WaylandSocket {
            socket,
            listen_thread,
        }
    }

    pub fn disconnect(&self) {
        self.socket
            .read()
            .unwrap()
            .shutdown(std::net::Shutdown::Both)
            .unwrap();
    }

    pub fn send(&self, buffer: &[u8]) {
        self.socket.write().unwrap().write_all(buffer).unwrap();
    }
}