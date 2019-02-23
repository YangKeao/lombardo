use std::io::{Write, Read};

#[derive(Debug, Fail)]
pub enum SocketError {
    #[fail(display = "Cannot get env XDG_RUNTIME_DIR")]
    CannotGetXDGRUNTIMEDIR,
}


pub fn get_default_socket_path() -> Result<std::path::PathBuf, SocketError> {
    let mut xdg_runtime_dir = std::path::PathBuf::from(match std::env::var("XDG_RUNTIME_DIR") {
        Ok(var) => var,
        Err(_) => return Err(SocketError::CannotGetXDGRUNTIMEDIR),
    });
    if let Ok(sock) = std::env::var("WAYLAND_DISPLAY") {
        xdg_runtime_dir.push(sock);
    } else {
        xdg_runtime_dir.push("wayland-0");
    }
    Ok(xdg_runtime_dir)
}

struct WaylandServerSocket {
    listener: std::os::unix::net::UnixListener
}

impl WaylandServerSocket {
    pub fn new(path: std::path::PathBuf) -> WaylandServerSocket {
        return WaylandServerSocket {
            listener: std::os::unix::net::UnixListener::bind(path).unwrap()
        }
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut response = String::new();
                    stream.read_to_string(&mut response).unwrap();

                    if response == "PING" {
                        stream.write_all(b"PONG").unwrap();
                        println!("WRITE");
                    }
                }
                Err(err) => {
                    println!("ERROR");
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_listen() {
        let tmp_dir = tempdir::TempDir::new("storm_test").unwrap();
        let socket_path = tmp_dir.path().join("test.socket");

        let mut wayland_server_socket = WaylandServerSocket::new(std::path::PathBuf::from(&socket_path));
        let server_thread = std::thread::spawn(move || {
            wayland_server_socket.listen();

            drop(wayland_server_socket);
        });

        let mut stream = std::os::unix::net::UnixStream::connect(std::path::PathBuf::from(&socket_path)).unwrap();
        stream.write_all(b"PING\0").unwrap();
        stream.flush().unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        println!("RESPONSE");

        server_thread.join().unwrap();
        tmp_dir.close().unwrap();
    }
}
