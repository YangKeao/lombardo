use super::wayland::EventHeader;
use super::wayland::ReadEvent;
use crate::unix_socket::UnixSocket;
use std::io::Write;
use std::os::unix::io::RawFd;
use std::sync::Arc;
use std::sync::Mutex;

pub struct WaylandSocket {
    write_stream: Arc<Mutex<UnixSocket>>,
    read_stream: Arc<Mutex<UnixSocket>>,
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

        let socket = UnixSocket::connect(path);
        let write_stream = Arc::new(Mutex::new(socket.clone()));
        let read_stream = Arc::new(Mutex::new(socket.clone()));

        WaylandSocket {
            write_stream,
            read_stream,
        }
    }

    pub fn disconnect(&self) {
        self.write_stream.lock().unwrap().shutdown();
    }

    pub fn send(&self, buffer: &[u8], fd: Option<RawFd>) {
        info!("Send to server Buffer:{:?}", buffer);
        self.write_stream.lock().unwrap().write(buffer, fd);
    }

    pub fn read_event(&self) -> (EventHeader, std::vec::Vec<u8>) {
        self.read_stream.lock().unwrap().read_event()
    }
}
