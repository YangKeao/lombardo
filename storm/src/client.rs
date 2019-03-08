use super::socket::WaylandSocket;
use super::wayland::{WlDisplay, WlObject, WlRawObject};
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

        let client = Client {
            socket,
            obj_map: Mutex::new(HashMap::new()),
        };
        client.bind_obj::<WlDisplay>(1);
        client.start_event_loop();

        return client;
    }

    pub fn start_event_loop(&self) {
        let read_socket = self.socket.clone();
        thread::spawn(move || loop {
            let (raw_event_header, msg_body) = read_socket.read_event(); // TODO: Handle Event
        });
    }

    pub fn get_display(&self) -> WlDisplay {
        self.get_obj(1).try_get_wl_display().unwrap()
    }

    pub fn get_obj(&self, obj_id: u32) -> Arc<WlObject> {
        self.obj_map.lock().unwrap().get(&obj_id).unwrap().clone()
    }

    pub fn bind_obj<T: WlRawObject>(&self, obj_id: u32) {
        let wl_obj = Arc::new(T::new(obj_id, self.socket.clone()).to_enum());
        self.obj_map.lock().unwrap().insert(obj_id, wl_obj.clone());
    }

    pub fn disconnect(&self) {
        self.socket.disconnect();
    }
}
