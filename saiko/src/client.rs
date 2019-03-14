use super::socket::WaylandSocket;
use super::wayland;
use super::wayland::{WlDisplay, WlObject, WlRawObject};
use crate::wayland::WlCallback;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::{Arc, Condvar, RwLock};
use std::thread;

#[derive(Clone)]
pub struct Client {
    socket: Arc<WaylandSocket>,
    pub obj_map: Arc<Mutex<HashMap<u32, Arc<WlObject>>>>,
    pub max_id: Arc<Mutex<u32>>,
    pub event_listeners: Arc<RwLock<Vec<Box<Fn(&wayland::Event) + Send + Sync>>>>,
}

impl Client {
    pub fn connect(name: Option<&str>) -> Client {
        let socket = Arc::new(WaylandSocket::connect(name));

        let client = Client {
            socket,
            obj_map: Arc::new(Mutex::new(HashMap::new())),
            max_id: Arc::new(Mutex::new(0)),
            event_listeners: Arc::new(RwLock::new(Vec::new())),
        };
        client.bind_obj::<WlDisplay>(1);
        client.start_event_loop();

        return client;
    }

    pub fn start_event_loop(&self) {
        let this = self.clone();
        thread::spawn(move || loop {
            let evs = this.socket.read_event();
            for (raw_event_header, msg_body) in evs {
                let sender = this.get_obj(raw_event_header.sender_id).unwrap();
                let event = sender.parse_event(
                    raw_event_header.sender_id,
                    raw_event_header.op_code,
                    msg_body,
                );
                for event_handler in this.event_listeners.read().unwrap().iter() {
                    event_handler(&event);
                }
            }
        });
    }

    pub fn get_display(&self) -> WlDisplay {
        self.get_obj(1).unwrap().try_get_wl_display().unwrap()
    }

    pub fn sync(&self) {
        let callback_id = self.new_obj::<WlCallback>();

        let done_pair = Arc::new((Mutex::new(false), Condvar::new()));
        let c_done_pair = done_pair.clone();
        self.add_event_listener(Box::new(move |ev| match ev {
            wayland::Event::WlCallbackEvent(callback_ev) => match callback_ev {
                wayland::WlCallbackEvent::WlCallbackdoneEvent(done) => {
                    if done.sender_id == callback_id {
                        info!("Callback id {} Done", callback_id);
                        let &(ref done, ref cond_var) = &*c_done_pair;
                        *(done.lock().unwrap()) = true;
                        cond_var.notify_all();
                    }
                }
                _ => {}
            },
            _ => {}
        }));
        self.get_display().sync(callback_id);

        let &(ref done, ref cond_var) = &*done_pair;
        let mut done = done.lock().unwrap();
        while !*done {
            done = cond_var.wait(done).unwrap();
        }
    }

    pub fn get_obj(&self, obj_id: u32) -> Option<Arc<WlObject>> {
        Some(self.obj_map.lock().unwrap().get(&obj_id)?.clone())
    }

    pub fn new_obj<T: WlRawObject>(&self) -> u32 {
        let mut hash_map = self.obj_map.lock().unwrap();
        let mut now_max = self.max_id.lock().unwrap();

        let new_id = now_max.clone() + 1;
        let wl_obj = Arc::new(T::new(new_id, self.socket.clone()).to_enum());
        hash_map.insert(new_id, wl_obj.clone());
        *(now_max) += 1;

        return new_id;
    }

    pub fn delete_obj(&self, obj_id: u32) {
        self.obj_map.lock().unwrap().remove(&obj_id);
    }

    pub fn bind_obj<T: WlRawObject>(&self, obj_id: u32) {
        let wl_obj = Arc::new(T::new(obj_id, self.socket.clone()).to_enum());
        self.obj_map.lock().unwrap().insert(obj_id, wl_obj.clone());

        let now_max = self.max_id.lock().unwrap().clone();
        *(self.max_id.lock().unwrap()) = std::cmp::max(now_max, obj_id);
    }

    pub fn add_event_listener(&self, event_handler: Box<Fn(&wayland::Event) + Send + Sync>) {
        self.event_listeners.write().unwrap().push(event_handler);
    }

    pub fn disconnect(&self) {
        self.socket.disconnect();
    }
}
