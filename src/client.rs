use std::mem::transmute;
use std::os::unix::net::UnixStream;
use std::io::Write;

pub struct Display {
    socket: UnixStream
}

impl Display {
    pub fn connect(name: Option<&str>) -> Display {
        let default_name = std::env::var("WAYLAND_DISPLAY").unwrap_or("wayland-0".to_string());
        let name = name.unwrap_or(&default_name);

        let path = std::path::Path::new(name);
        let path = if path.is_relative() {
            std::path::Path::new(&std::env::var("XDG_RUNTIME_DIR").unwrap()).join(path)
        } else {
            path.to_path_buf()
        };

        Display {
            socket: UnixStream::connect(path).unwrap()
        }
    }

    pub fn disconnect(&self) {
        self.socket.shutdown(std::net::Shutdown::Both).unwrap();
    }

    pub fn get_registry(&mut self) {
        let buffer: (u32, u32, u32) = (1, (12 << 16) + 1, 2);
        self.socket.write_all(unsafe {&transmute::<(u32, u32, u32), [u8; 12]>(buffer)}).unwrap();
    }
}
