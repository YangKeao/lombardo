use super::socket::WaylandSocket;
use super::wayland;
use super::wayland::{WlDisplay, WlObject, WlRawObject};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::{Arc, RwLock};
use std::thread;

#[derive(Clone)]
pub struct Client {
    socket: Arc<WaylandSocket>,
    pub obj_map: Arc<Mutex<HashMap<u32, Arc<WlObject>>>>,
    pub event_listeners: Arc<RwLock<Vec<Box<Fn(&wayland::Event) + Send + Sync>>>>,
}

impl Client {
    pub fn connect(name: Option<&str>) -> Client {
        let socket = Arc::new(WaylandSocket::connect(name));

        let client = Client {
            socket,
            obj_map: Arc::new(Mutex::new(HashMap::new())),
            event_listeners: Arc::new(RwLock::new(Vec::new())),
        };
        client.bind_obj::<WlDisplay>(1);
        client.start_event_loop();

        return client;
    }

    pub fn start_event_loop(&self) {
        let this = self.clone();
        thread::spawn(move || loop {
            let (raw_event_header, msg_body) = this.socket.read_event(); // TODO: Handle Event
            let sender = this.get_obj(raw_event_header.sender_id).unwrap();
            let event = sender.parse_event(
                raw_event_header.sender_id,
                raw_event_header.op_code,
                msg_body,
            );
            for event_handler in this.event_listeners.read().unwrap().iter() {
                event_handler(&event);
            }
        });
    }

    pub fn get_display(&self) -> WlDisplay {
        self.get_obj(1).unwrap().try_get_wl_display().unwrap()
    }

    pub fn get_obj(&self, obj_id: u32) -> Option<Arc<WlObject>> {
        Some(self.obj_map.lock().unwrap().get(&obj_id)?.clone())
    }

    pub fn bind_obj<T: WlRawObject>(&self, obj_id: u32) {
        let wl_obj = Arc::new(T::new(obj_id, self.socket.clone()).to_enum());
        self.obj_map.lock().unwrap().insert(obj_id, wl_obj.clone());
    }

    pub fn add_event_listener(&self, event_handler: Box<Fn(&wayland::Event) + Send + Sync>) {
        self.event_listeners.write().unwrap().push(event_handler);
    }

    pub fn disconnect(&self) {
        self.socket.disconnect();
    }
}
