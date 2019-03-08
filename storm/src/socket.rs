use super::wayland::EventHeader;
use super::wayland::ReadEvent;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::sync::Mutex;

pub struct WaylandSocket {
    write_stream: Arc<Mutex<UnixStream>>,
    read_stream: Arc<Mutex<UnixStream>>,
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

        let stream = UnixStream::connect(path).unwrap();
        let write_stream = Arc::new(Mutex::new(stream.try_clone().unwrap()));
        let read_stream = Arc::new(Mutex::new(stream.try_clone().unwrap()));

        WaylandSocket {
            write_stream,
            read_stream,
        }
    }

    pub fn disconnect(&self) {
        self.write_stream
            .lock()
            .unwrap()
            .shutdown(std::net::Shutdown::Both)
            .unwrap();
    }

    pub fn send(&self, buffer: &[u8]) {
        info!("Send to server {:?}", buffer);
        //        self.socket.write().unwrap().write_all(buffer).unwrap();
        self.write_stream.lock().unwrap().write_all(buffer).unwrap();
    }

    pub fn read_event(&self) -> (EventHeader, std::vec::Vec<u8>) {
        self.read_stream.lock().unwrap().read_event()
    }
}
