use super::socket::WaylandSocket;
use super::wayland::WlDisplay;
use std::sync::Arc;

pub struct Client {
    socket: Arc<WaylandSocket>,
    pub display: WlDisplay,
}

impl Client {
    pub fn connect(name: Option<&str>) -> Client {
        let socket = Arc::new(WaylandSocket::connect(name));
        let sub_socket = socket.clone();

        let client = Client {
            socket,
            display: WlDisplay {
                object_id: 1,
                socket: sub_socket,
            },
        };

        return client;
    }
}
