use std::sync::Mutex;
use std::sync::Arc;
use std::mem::transmute;
use std::os::unix::net::UnixStream;
use std::io::Write;
use std::io::Read;
use std::thread;

pub struct Display {
    socket: Arc<Mutex<UnixStream>>,
    listen_thread: std::thread::JoinHandle<()>,
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

        let socket = Arc::new(Mutex::new(UnixStream::connect(path).unwrap()));

        let c_socket = socket.clone();
        let listen_thread = thread::spawn(move || {
            let mut head: [u8; 8] = [0; 8];
            loop {
                c_socket.lock().unwrap().read_exact(&mut head).unwrap();

                println!("{:?}", head);
            }
        });

        Display {
            socket,
            listen_thread,
        }
    }

    pub fn disconnect(&self) {
        self.socket.lock().unwrap().shutdown(std::net::Shutdown::Both).unwrap();
    }

    pub fn get_registry(&mut self) {
        let buffer: (u32, u32, u32) = (1, (12 << 16) + 1, 2);
        self.socket.lock().unwrap().write_all(unsafe {&transmute::<(u32, u32, u32), [u8; 12]>(buffer)}).unwrap();
    }
}
