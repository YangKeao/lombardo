use super::socket::WaylandSocket;
use super::wayland::{WlDisplay, WlObject};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub struct Client {
    socket: Arc<WaylandSocket>,
    pub obj_map: HashMap<u32, Arc<WlObject>>,
    pub display: Arc<WlObject>,
}

impl Client {
    pub fn connect(name: Option<&str>) -> Client {
        let socket = Arc::new(WaylandSocket::connect(name));
        let sub_socket = socket.clone();

        let read_socket = socket.clone();
        thread::spawn(move || loop {
            read_socket.read_event()
        });

        let display = Arc::new(WlObject::WlDisplay(WlDisplay {
            object_id: 1,
            socket: sub_socket,
        }));
        let mut obj_map = HashMap::new();
        obj_map.insert(1, display.clone());

        let client = Client {
            socket,
            obj_map,
            display,
        };

        return client;
    }

    pub fn get_display(&self) -> &WlDisplay {
        match &*self.display {
            WlObject::WlDisplay(display) => &display,
            _ => panic!("Display is not display type"),
        }
    }

    pub fn disconnect(&self) {
        self.socket.disconnect();
    }
}
