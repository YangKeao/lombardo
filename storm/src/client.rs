use super::socket::WaylandSocket;
use super::wayland::{WlDisplay, WlEnum, WlObject};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct Client {
    socket: Arc<WaylandSocket>,
    pub obj_map: Mutex<HashMap<u32, Arc<WlObject>>>,
}

impl Client {
    pub fn connect(name: Option<&str>) -> Client {
        let socket = Arc::new(WaylandSocket::connect(name));

        let read_socket = socket.clone();
        thread::spawn(move || loop {
            read_socket.read_event(); // TODO: Handle Event
        });

        let client = Client {
            socket,
            obj_map: Mutex::new(HashMap::new()),
        };
        client.bind_obj::<WlDisplay>(1);

        return client;
    }

    pub fn get_display(&self) -> WlDisplay {
        let wl_obj = self.obj_map.lock().unwrap().get(&1).unwrap().clone();
        match &*wl_obj {
            WlObject::WlDisplay(display) => display.clone(),
            _ => panic!("Object ID 1 is not Display"), // TODO: Handle error in rust way.
        }
    }

    pub fn bind_obj<T: WlEnum>(&self, obj_id: u32) {
        let wl_obj = Arc::new(T::new(obj_id, self.socket.clone()).to_enum());
        self.obj_map.lock().unwrap().insert(obj_id, wl_obj.clone());
    }

    pub fn disconnect(&self) {
        self.socket.disconnect();
    }
}
