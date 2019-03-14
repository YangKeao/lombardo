use super::socket::*;
use crate::unix_socket::UnixSocket;
use std::mem::size_of;
use std::mem::transmute;
use std::sync::Arc;
type NewId = u32;
type Uint = u32;
type Int = i32;
type Fd = i32;
type Object = u32;
type Fixed = f32;
type Array = Vec<u32>;
#[derive(Clone)]
pub struct WlDisplay {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlDisplay {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDisplay {
        WlDisplay { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDisplay(self)
    }
}
impl WlDisplay {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlDisplayErrorEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let object_id = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let code = unsafe { *raw_ptr };
                parsed_len += size_of::<u32>();
                let start = parsed_len - size_of::<u32>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const u32;
                let str_len = unsafe { *raw_ptr };
                let str_len = (str_len as f64 / 4.0).ceil() as usize * 4;
                parsed_len += str_len;
                let src_ptr = msg_body[(start + size_of::<u32>())..parsed_len].as_ptr();
                let mut tmp_ptr = Vec::with_capacity(str_len);
                unsafe {
                    tmp_ptr.set_len(str_len);
                    std::ptr::copy(src_ptr, tmp_ptr.as_mut_ptr(), str_len);
                };
                let message = std::str::from_utf8(&tmp_ptr)
                    .unwrap()
                    .trim_matches('\0')
                    .to_string();
                Event::WlDisplayEvent(WlDisplayEvent::WlDisplayerrorEvent(WlDisplayerrorEvent {
                    sender_id,
                    object_id,
                    code,
                    message,
                }))
            }
            1u16 => {
                info!("Receive event {}", "WlDisplayDeleteIdEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let id = unsafe { *raw_ptr };
                Event::WlDisplayEvent(WlDisplayEvent::WlDisplaydeleteIdEvent(
                    WlDisplaydeleteIdEvent { sender_id, id },
                ))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn sync(&self, callback: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &callback as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn get_registry(&self, registry: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &registry as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlRegistry {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlRegistry {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlRegistry {
        WlRegistry { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlRegistry(self)
    }
}
impl WlRegistry {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlRegistryGlobalEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let name = unsafe { *raw_ptr };
                parsed_len += size_of::<u32>();
                let start = parsed_len - size_of::<u32>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const u32;
                let str_len = unsafe { *raw_ptr };
                let str_len = (str_len as f64 / 4.0).ceil() as usize * 4;
                parsed_len += str_len;
                let src_ptr = msg_body[(start + size_of::<u32>())..parsed_len].as_ptr();
                let mut tmp_ptr = Vec::with_capacity(str_len);
                unsafe {
                    tmp_ptr.set_len(str_len);
                    std::ptr::copy(src_ptr, tmp_ptr.as_mut_ptr(), str_len);
                };
                let interface = std::str::from_utf8(&tmp_ptr)
                    .unwrap()
                    .trim_matches('\0')
                    .to_string();
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let version = unsafe { *raw_ptr };
                Event::WlRegistryEvent(WlRegistryEvent::WlRegistryglobalEvent(
                    WlRegistryglobalEvent {
                        sender_id,
                        name,
                        interface,
                        version,
                    },
                ))
            }
            1u16 => {
                info!("Receive event {}", "WlRegistryGlobalRemoveEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let name = unsafe { *raw_ptr };
                Event::WlRegistryEvent(WlRegistryEvent::WlRegistryglobalRemoveEvent(
                    WlRegistryglobalRemoveEvent { sender_id, name },
                ))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn bind(&self, name: Uint, interface_name: String, interface_version: Uint, id: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += size_of::<String>();
        raw_size += size_of::<Uint>();
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &name as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        let str_len = interface_name.len();
        let buf_len = ((interface_name.len() + 1) as f64 / 4.0).ceil() as usize * 4;
        unsafe {
            std::ptr::copy(
                &buf_len as *const usize as *const u8,
                &mut send_buffer[written_len] as *mut u8,
                str_len + 1,
            );
            std::ptr::copy(
                &interface_name.into_bytes()[0] as *const u8,
                &mut send_buffer[written_len + 4] as *mut u8,
                str_len,
            );
        }
        #[allow(unused)]
        written_len += buf_len + 4;
        unsafe {
            std::ptr::copy(
                &interface_version as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlCallback {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlCallback {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlCallback {
        WlCallback { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlCallback(self)
    }
}
impl WlCallback {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlCallbackDoneEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let callback_data = unsafe { *raw_ptr };
                Event::WlCallbackEvent(WlCallbackEvent::WlCallbackdoneEvent(WlCallbackdoneEvent {
                    sender_id,
                    callback_data,
                }))
            }
            _ => panic!("Unknown event"),
        }
    }
}
#[derive(Clone)]
pub struct WlCompositor {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlCompositor {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlCompositor {
        WlCompositor { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlCompositor(self)
    }
}
impl WlCompositor {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            _ => panic!("Unknown event"),
        }
    }
    pub fn create_surface(&self, id: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn create_region(&self, id: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlShmPool {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlShmPool {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlShmPool {
        WlShmPool { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlShmPool(self)
    }
}
impl WlShmPool {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            _ => panic!("Unknown event"),
        }
    }
    pub fn create_buffer(
        &self,
        id: NewId,
        offset: Int,
        width: Int,
        height: Int,
        stride: Int,
        format: Uint,
    ) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &offset as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &stride as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &format as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn destroy(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn resize(&self, size: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &size as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlShm {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlShm {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlShm {
        WlShm { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlShm(self)
    }
}
impl WlShm {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlShmFormatEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let format = unsafe { *raw_ptr };
                Event::WlShmEvent(WlShmEvent::WlShmformatEvent(WlShmformatEvent {
                    sender_id,
                    format,
                }))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn create_pool(&self, id: NewId, fd: Fd, size: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Fd>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        info!("Send FD: {}", fd);
        send_fd[send_fd_num] = fd;
        send_fd_num += 1;
        unsafe {
            std::ptr::copy(
                &size as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlBuffer {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlBuffer {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlBuffer {
        WlBuffer { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlBuffer(self)
    }
}
impl WlBuffer {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlBufferReleaseEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlBufferEvent(WlBufferEvent::WlBufferreleaseEvent(WlBufferreleaseEvent {
                    sender_id,
                }))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn destroy(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlDataOffer {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlDataOffer {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDataOffer {
        WlDataOffer { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDataOffer(self)
    }
}
impl WlDataOffer {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlDataOfferOfferEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<u32>();
                let start = parsed_len - size_of::<u32>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const u32;
                let str_len = unsafe { *raw_ptr };
                let str_len = (str_len as f64 / 4.0).ceil() as usize * 4;
                parsed_len += str_len;
                let src_ptr = msg_body[(start + size_of::<u32>())..parsed_len].as_ptr();
                let mut tmp_ptr = Vec::with_capacity(str_len);
                unsafe {
                    tmp_ptr.set_len(str_len);
                    std::ptr::copy(src_ptr, tmp_ptr.as_mut_ptr(), str_len);
                };
                let mime_type = std::str::from_utf8(&tmp_ptr)
                    .unwrap()
                    .trim_matches('\0')
                    .to_string();
                Event::WlDataOfferEvent(WlDataOfferEvent::WlDataOfferofferEvent(
                    WlDataOfferofferEvent {
                        sender_id,
                        mime_type,
                    },
                ))
            }
            1u16 => {
                info!("Receive event {}", "WlDataOfferSourceActionsEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let source_actions = unsafe { *raw_ptr };
                Event::WlDataOfferEvent(WlDataOfferEvent::WlDataOffersourceActionsEvent(
                    WlDataOffersourceActionsEvent {
                        sender_id,
                        source_actions,
                    },
                ))
            }
            2u16 => {
                info!("Receive event {}", "WlDataOfferActionEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let dnd_action = unsafe { *raw_ptr };
                Event::WlDataOfferEvent(WlDataOfferEvent::WlDataOfferactionEvent(
                    WlDataOfferactionEvent {
                        sender_id,
                        dnd_action,
                    },
                ))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn accept(&self, serial: Uint, mime_type: String) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += size_of::<String>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        let str_len = mime_type.len();
        let buf_len = ((mime_type.len() + 1) as f64 / 4.0).ceil() as usize * 4;
        unsafe {
            std::ptr::copy(
                &buf_len as *const usize as *const u8,
                &mut send_buffer[written_len] as *mut u8,
                str_len + 1,
            );
            std::ptr::copy(
                &mime_type.into_bytes()[0] as *const u8,
                &mut send_buffer[written_len + 4] as *mut u8,
                str_len,
            );
        }
        #[allow(unused)]
        written_len += buf_len + 4;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn receive(&self, mime_type: String, fd: Fd) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<String>();
        raw_size += size_of::<Fd>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        let str_len = mime_type.len();
        let buf_len = ((mime_type.len() + 1) as f64 / 4.0).ceil() as usize * 4;
        unsafe {
            std::ptr::copy(
                &buf_len as *const usize as *const u8,
                &mut send_buffer[written_len] as *mut u8,
                str_len + 1,
            );
            std::ptr::copy(
                &mime_type.into_bytes()[0] as *const u8,
                &mut send_buffer[written_len + 4] as *mut u8,
                str_len,
            );
        }
        #[allow(unused)]
        written_len += buf_len + 4;
        info!("Send FD: {}", fd);
        send_fd[send_fd_num] = fd;
        send_fd_num += 1;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn destroy(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn finish(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_actions(&self, dnd_actions: Uint, preferred_action: Uint) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (4i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &dnd_actions as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &preferred_action as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlDataSource {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlDataSource {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDataSource {
        WlDataSource { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDataSource(self)
    }
}
impl WlDataSource {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlDataSourceTargetEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<u32>();
                let start = parsed_len - size_of::<u32>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const u32;
                let str_len = unsafe { *raw_ptr };
                let str_len = (str_len as f64 / 4.0).ceil() as usize * 4;
                parsed_len += str_len;
                let src_ptr = msg_body[(start + size_of::<u32>())..parsed_len].as_ptr();
                let mut tmp_ptr = Vec::with_capacity(str_len);
                unsafe {
                    tmp_ptr.set_len(str_len);
                    std::ptr::copy(src_ptr, tmp_ptr.as_mut_ptr(), str_len);
                };
                let mime_type = std::str::from_utf8(&tmp_ptr)
                    .unwrap()
                    .trim_matches('\0')
                    .to_string();
                Event::WlDataSourceEvent(WlDataSourceEvent::WlDataSourcetargetEvent(
                    WlDataSourcetargetEvent {
                        sender_id,
                        mime_type,
                    },
                ))
            }
            1u16 => {
                info!("Receive event {}", "WlDataSourceSendEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<u32>();
                let start = parsed_len - size_of::<u32>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const u32;
                let str_len = unsafe { *raw_ptr };
                let str_len = (str_len as f64 / 4.0).ceil() as usize * 4;
                parsed_len += str_len;
                let src_ptr = msg_body[(start + size_of::<u32>())..parsed_len].as_ptr();
                let mut tmp_ptr = Vec::with_capacity(str_len);
                unsafe {
                    tmp_ptr.set_len(str_len);
                    std::ptr::copy(src_ptr, tmp_ptr.as_mut_ptr(), str_len);
                };
                let mime_type = std::str::from_utf8(&tmp_ptr)
                    .unwrap()
                    .trim_matches('\0')
                    .to_string();
                parsed_len += size_of::<Fd>();
                let start = parsed_len - size_of::<Fd>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Fd;
                let fd = unsafe { *raw_ptr };
                Event::WlDataSourceEvent(WlDataSourceEvent::WlDataSourcesendEvent(
                    WlDataSourcesendEvent {
                        sender_id,
                        mime_type,
                        fd,
                    },
                ))
            }
            2u16 => {
                info!("Receive event {}", "WlDataSourceCancelledEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlDataSourceEvent(WlDataSourceEvent::WlDataSourcecancelledEvent(
                    WlDataSourcecancelledEvent { sender_id },
                ))
            }
            3u16 => {
                info!("Receive event {}", "WlDataSourceDndDropPerformedEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlDataSourceEvent(WlDataSourceEvent::WlDataSourcedndDropPerformedEvent(
                    WlDataSourcedndDropPerformedEvent { sender_id },
                ))
            }
            4u16 => {
                info!("Receive event {}", "WlDataSourceDndFinishedEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlDataSourceEvent(WlDataSourceEvent::WlDataSourcedndFinishedEvent(
                    WlDataSourcedndFinishedEvent { sender_id },
                ))
            }
            5u16 => {
                info!("Receive event {}", "WlDataSourceActionEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let dnd_action = unsafe { *raw_ptr };
                Event::WlDataSourceEvent(WlDataSourceEvent::WlDataSourceactionEvent(
                    WlDataSourceactionEvent {
                        sender_id,
                        dnd_action,
                    },
                ))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn offer(&self, mime_type: String) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<String>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        let str_len = mime_type.len();
        let buf_len = ((mime_type.len() + 1) as f64 / 4.0).ceil() as usize * 4;
        unsafe {
            std::ptr::copy(
                &buf_len as *const usize as *const u8,
                &mut send_buffer[written_len] as *mut u8,
                str_len + 1,
            );
            std::ptr::copy(
                &mime_type.into_bytes()[0] as *const u8,
                &mut send_buffer[written_len + 4] as *mut u8,
                str_len,
            );
        }
        #[allow(unused)]
        written_len += buf_len + 4;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn destroy(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_actions(&self, dnd_actions: Uint) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &dnd_actions as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlDataDevice {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlDataDevice {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDataDevice {
        WlDataDevice { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDataDevice(self)
    }
}
impl WlDataDevice {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlDataDeviceDataOfferEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<NewId>();
                let start = parsed_len - size_of::<NewId>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const NewId;
                let id = unsafe { *raw_ptr };
                Event::WlDataDeviceEvent(WlDataDeviceEvent::WlDataDevicedataOfferEvent(
                    WlDataDevicedataOfferEvent { sender_id, id },
                ))
            }
            1u16 => {
                info!("Receive event {}", "WlDataDeviceEnterEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let surface = unsafe { *raw_ptr };
                let x: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                let y: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let id = unsafe { *raw_ptr };
                Event::WlDataDeviceEvent(WlDataDeviceEvent::WlDataDeviceenterEvent(
                    WlDataDeviceenterEvent {
                        sender_id,
                        serial,
                        surface,
                        x,
                        y,
                        id,
                    },
                ))
            }
            2u16 => {
                info!("Receive event {}", "WlDataDeviceLeaveEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlDataDeviceEvent(WlDataDeviceEvent::WlDataDeviceleaveEvent(
                    WlDataDeviceleaveEvent { sender_id },
                ))
            }
            3u16 => {
                info!("Receive event {}", "WlDataDeviceMotionEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let time = unsafe { *raw_ptr };
                let x: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                let y: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                Event::WlDataDeviceEvent(WlDataDeviceEvent::WlDataDevicemotionEvent(
                    WlDataDevicemotionEvent {
                        sender_id,
                        time,
                        x,
                        y,
                    },
                ))
            }
            4u16 => {
                info!("Receive event {}", "WlDataDeviceDropEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlDataDeviceEvent(WlDataDeviceEvent::WlDataDevicedropEvent(
                    WlDataDevicedropEvent { sender_id },
                ))
            }
            5u16 => {
                info!("Receive event {}", "WlDataDeviceSelectionEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let id = unsafe { *raw_ptr };
                Event::WlDataDeviceEvent(WlDataDeviceEvent::WlDataDeviceselectionEvent(
                    WlDataDeviceselectionEvent { sender_id, id },
                ))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn start_drag(&self, source: Object, origin: Object, icon: Object, serial: Uint) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &source as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &origin as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &icon as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_selection(&self, source: Object, serial: Uint) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &source as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn release(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlDataDeviceManager {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlDataDeviceManager {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDataDeviceManager {
        WlDataDeviceManager { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDataDeviceManager(self)
    }
}
impl WlDataDeviceManager {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            _ => panic!("Unknown event"),
        }
    }
    pub fn create_data_source(&self, id: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn get_data_device(&self, id: NewId, seat: Object) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &seat as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlShell {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlShell {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlShell {
        WlShell { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlShell(self)
    }
}
impl WlShell {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            _ => panic!("Unknown event"),
        }
    }
    pub fn get_shell_surface(&self, id: NewId, surface: Object) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &surface as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlShellSurface {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlShellSurface {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlShellSurface {
        WlShellSurface { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlShellSurface(self)
    }
}
impl WlShellSurface {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlShellSurfacePingEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                Event::WlShellSurfaceEvent(WlShellSurfaceEvent::WlShellSurfacepingEvent(
                    WlShellSurfacepingEvent { sender_id, serial },
                ))
            }
            1u16 => {
                info!("Receive event {}", "WlShellSurfaceConfigureEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let edges = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let width = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let height = unsafe { *raw_ptr };
                Event::WlShellSurfaceEvent(WlShellSurfaceEvent::WlShellSurfaceconfigureEvent(
                    WlShellSurfaceconfigureEvent {
                        sender_id,
                        edges,
                        width,
                        height,
                    },
                ))
            }
            2u16 => {
                info!("Receive event {}", "WlShellSurfacePopupDoneEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlShellSurfaceEvent(WlShellSurfaceEvent::WlShellSurfacepopupDoneEvent(
                    WlShellSurfacepopupDoneEvent { sender_id },
                ))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn pong(&self, serial: Uint) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn mv(&self, seat: Object, serial: Uint) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &seat as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn resize(&self, seat: Object, serial: Uint, edges: Uint) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &seat as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &edges as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_toplevel(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_transient(&self, parent: Object, x: Int, y: Int, flags: Uint) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (4i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &parent as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &flags as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_fullscreen(&self, method: Uint, framerate: Uint, output: Object) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (5i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &method as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &framerate as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &output as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_popup(
        &self,
        seat: Object,
        serial: Uint,
        parent: Object,
        x: Int,
        y: Int,
        flags: Uint,
    ) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (6i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &seat as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &parent as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &flags as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_maximized(&self, output: Object) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (7i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &output as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_title(&self, title: String) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<String>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (8i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        let str_len = title.len();
        let buf_len = ((title.len() + 1) as f64 / 4.0).ceil() as usize * 4;
        unsafe {
            std::ptr::copy(
                &buf_len as *const usize as *const u8,
                &mut send_buffer[written_len] as *mut u8,
                str_len + 1,
            );
            std::ptr::copy(
                &title.into_bytes()[0] as *const u8,
                &mut send_buffer[written_len + 4] as *mut u8,
                str_len,
            );
        }
        #[allow(unused)]
        written_len += buf_len + 4;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_class(&self, class_: String) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<String>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (9i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        let str_len = class_.len();
        let buf_len = ((class_.len() + 1) as f64 / 4.0).ceil() as usize * 4;
        unsafe {
            std::ptr::copy(
                &buf_len as *const usize as *const u8,
                &mut send_buffer[written_len] as *mut u8,
                str_len + 1,
            );
            std::ptr::copy(
                &class_.into_bytes()[0] as *const u8,
                &mut send_buffer[written_len + 4] as *mut u8,
                str_len,
            );
        }
        #[allow(unused)]
        written_len += buf_len + 4;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlSurface {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlSurface {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlSurface {
        WlSurface { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlSurface(self)
    }
}
impl WlSurface {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlSurfaceEnterEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let output = unsafe { *raw_ptr };
                Event::WlSurfaceEvent(WlSurfaceEvent::WlSurfaceenterEvent(WlSurfaceenterEvent {
                    sender_id,
                    output,
                }))
            }
            1u16 => {
                info!("Receive event {}", "WlSurfaceLeaveEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let output = unsafe { *raw_ptr };
                Event::WlSurfaceEvent(WlSurfaceEvent::WlSurfaceleaveEvent(WlSurfaceleaveEvent {
                    sender_id,
                    output,
                }))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn destroy(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn attach(&self, buffer: Object, x: Int, y: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &buffer as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn damage(&self, x: Int, y: Int, width: Int, height: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn frame(&self, callback: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &callback as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_opaque_region(&self, region: Object) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (4i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &region as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_input_region(&self, region: Object) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (5i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &region as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn commit(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (6i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_buffer_transform(&self, transform: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (7i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &transform as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_buffer_scale(&self, scale: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (8i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &scale as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn damage_buffer(&self, x: Int, y: Int, width: Int, height: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (9i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlSeat {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlSeat {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlSeat {
        WlSeat { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlSeat(self)
    }
}
impl WlSeat {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlSeatCapabilitiesEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let capabilities = unsafe { *raw_ptr };
                Event::WlSeatEvent(WlSeatEvent::WlSeatcapabilitiesEvent(
                    WlSeatcapabilitiesEvent {
                        sender_id,
                        capabilities,
                    },
                ))
            }
            1u16 => {
                info!("Receive event {}", "WlSeatNameEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<u32>();
                let start = parsed_len - size_of::<u32>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const u32;
                let str_len = unsafe { *raw_ptr };
                let str_len = (str_len as f64 / 4.0).ceil() as usize * 4;
                parsed_len += str_len;
                let src_ptr = msg_body[(start + size_of::<u32>())..parsed_len].as_ptr();
                let mut tmp_ptr = Vec::with_capacity(str_len);
                unsafe {
                    tmp_ptr.set_len(str_len);
                    std::ptr::copy(src_ptr, tmp_ptr.as_mut_ptr(), str_len);
                };
                let name = std::str::from_utf8(&tmp_ptr)
                    .unwrap()
                    .trim_matches('\0')
                    .to_string();
                Event::WlSeatEvent(WlSeatEvent::WlSeatnameEvent(WlSeatnameEvent {
                    sender_id,
                    name,
                }))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn get_pointer(&self, id: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn get_keyboard(&self, id: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn get_touch(&self, id: NewId) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn release(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlPointer {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlPointer {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlPointer {
        WlPointer { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlPointer(self)
    }
}
impl WlPointer {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlPointerEnterEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let surface = unsafe { *raw_ptr };
                let surface_x: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                let surface_y: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                Event::WlPointerEvent(WlPointerEvent::WlPointerenterEvent(WlPointerenterEvent {
                    sender_id,
                    serial,
                    surface,
                    surface_x,
                    surface_y,
                }))
            }
            1u16 => {
                info!("Receive event {}", "WlPointerLeaveEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let surface = unsafe { *raw_ptr };
                Event::WlPointerEvent(WlPointerEvent::WlPointerleaveEvent(WlPointerleaveEvent {
                    sender_id,
                    serial,
                    surface,
                }))
            }
            2u16 => {
                info!("Receive event {}", "WlPointerMotionEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let time = unsafe { *raw_ptr };
                let surface_x: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                let surface_y: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                Event::WlPointerEvent(WlPointerEvent::WlPointermotionEvent(WlPointermotionEvent {
                    sender_id,
                    time,
                    surface_x,
                    surface_y,
                }))
            }
            3u16 => {
                info!("Receive event {}", "WlPointerButtonEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let time = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let button = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let state = unsafe { *raw_ptr };
                Event::WlPointerEvent(WlPointerEvent::WlPointerbuttonEvent(WlPointerbuttonEvent {
                    sender_id,
                    serial,
                    time,
                    button,
                    state,
                }))
            }
            4u16 => {
                info!("Receive event {}", "WlPointerAxisEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let time = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let axis = unsafe { *raw_ptr };
                let value: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                Event::WlPointerEvent(WlPointerEvent::WlPointeraxisEvent(WlPointeraxisEvent {
                    sender_id,
                    time,
                    axis,
                    value,
                }))
            }
            5u16 => {
                info!("Receive event {}", "WlPointerFrameEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlPointerEvent(WlPointerEvent::WlPointerframeEvent(WlPointerframeEvent {
                    sender_id,
                }))
            }
            6u16 => {
                info!("Receive event {}", "WlPointerAxisSourceEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let axis_source = unsafe { *raw_ptr };
                Event::WlPointerEvent(WlPointerEvent::WlPointeraxisSourceEvent(
                    WlPointeraxisSourceEvent {
                        sender_id,
                        axis_source,
                    },
                ))
            }
            7u16 => {
                info!("Receive event {}", "WlPointerAxisStopEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let time = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let axis = unsafe { *raw_ptr };
                Event::WlPointerEvent(WlPointerEvent::WlPointeraxisStopEvent(
                    WlPointeraxisStopEvent {
                        sender_id,
                        time,
                        axis,
                    },
                ))
            }
            8u16 => {
                info!("Receive event {}", "WlPointerAxisDiscreteEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let axis = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let discrete = unsafe { *raw_ptr };
                Event::WlPointerEvent(WlPointerEvent::WlPointeraxisDiscreteEvent(
                    WlPointeraxisDiscreteEvent {
                        sender_id,
                        axis,
                        discrete,
                    },
                ))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn set_cursor(&self, serial: Uint, surface: Object, hotspot_x: Int, hotspot_y: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &surface as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &hotspot_x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &hotspot_y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn release(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlKeyboard {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlKeyboard {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlKeyboard {
        WlKeyboard { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlKeyboard(self)
    }
}
impl WlKeyboard {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlKeyboardKeymapEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let format = unsafe { *raw_ptr };
                parsed_len += size_of::<Fd>();
                let start = parsed_len - size_of::<Fd>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Fd;
                let fd = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let size = unsafe { *raw_ptr };
                Event::WlKeyboardEvent(WlKeyboardEvent::WlKeyboardkeymapEvent(
                    WlKeyboardkeymapEvent {
                        sender_id,
                        format,
                        fd,
                        size,
                    },
                ))
            }
            1u16 => {
                info!("Receive event {}", "WlKeyboardEnterEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let surface = unsafe { *raw_ptr };
                let keys: Vec<u32> = Vec::new();
                warn!("Array value has not been implemented");
                Event::WlKeyboardEvent(WlKeyboardEvent::WlKeyboardenterEvent(
                    WlKeyboardenterEvent {
                        sender_id,
                        serial,
                        surface,
                        keys,
                    },
                ))
            }
            2u16 => {
                info!("Receive event {}", "WlKeyboardLeaveEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let surface = unsafe { *raw_ptr };
                Event::WlKeyboardEvent(WlKeyboardEvent::WlKeyboardleaveEvent(
                    WlKeyboardleaveEvent {
                        sender_id,
                        serial,
                        surface,
                    },
                ))
            }
            3u16 => {
                info!("Receive event {}", "WlKeyboardKeyEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let time = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let key = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let state = unsafe { *raw_ptr };
                Event::WlKeyboardEvent(WlKeyboardEvent::WlKeyboardkeyEvent(WlKeyboardkeyEvent {
                    sender_id,
                    serial,
                    time,
                    key,
                    state,
                }))
            }
            4u16 => {
                info!("Receive event {}", "WlKeyboardModifiersEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let mods_depressed = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let mods_latched = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let mods_locked = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let group = unsafe { *raw_ptr };
                Event::WlKeyboardEvent(WlKeyboardEvent::WlKeyboardmodifiersEvent(
                    WlKeyboardmodifiersEvent {
                        sender_id,
                        serial,
                        mods_depressed,
                        mods_latched,
                        mods_locked,
                        group,
                    },
                ))
            }
            5u16 => {
                info!("Receive event {}", "WlKeyboardRepeatInfoEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let rate = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let delay = unsafe { *raw_ptr };
                Event::WlKeyboardEvent(WlKeyboardEvent::WlKeyboardrepeatInfoEvent(
                    WlKeyboardrepeatInfoEvent {
                        sender_id,
                        rate,
                        delay,
                    },
                ))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn release(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlTouch {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlTouch {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlTouch {
        WlTouch { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlTouch(self)
    }
}
impl WlTouch {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlTouchDownEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let time = unsafe { *raw_ptr };
                parsed_len += size_of::<Object>();
                let start = parsed_len - size_of::<Object>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                let surface = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let id = unsafe { *raw_ptr };
                let x: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                let y: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                Event::WlTouchEvent(WlTouchEvent::WlTouchdownEvent(WlTouchdownEvent {
                    sender_id,
                    serial,
                    time,
                    surface,
                    id,
                    x,
                    y,
                }))
            }
            1u16 => {
                info!("Receive event {}", "WlTouchUpEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let serial = unsafe { *raw_ptr };
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let time = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let id = unsafe { *raw_ptr };
                Event::WlTouchEvent(WlTouchEvent::WlTouchupEvent(WlTouchupEvent {
                    sender_id,
                    serial,
                    time,
                    id,
                }))
            }
            2u16 => {
                info!("Receive event {}", "WlTouchMotionEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let time = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let id = unsafe { *raw_ptr };
                let x: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                let y: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                Event::WlTouchEvent(WlTouchEvent::WlTouchmotionEvent(WlTouchmotionEvent {
                    sender_id,
                    time,
                    id,
                    x,
                    y,
                }))
            }
            3u16 => {
                info!("Receive event {}", "WlTouchFrameEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlTouchEvent(WlTouchEvent::WlTouchframeEvent(WlTouchframeEvent {
                    sender_id,
                }))
            }
            4u16 => {
                info!("Receive event {}", "WlTouchCancelEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlTouchEvent(WlTouchEvent::WlTouchcancelEvent(WlTouchcancelEvent {
                    sender_id,
                }))
            }
            5u16 => {
                info!("Receive event {}", "WlTouchShapeEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let id = unsafe { *raw_ptr };
                let major: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                let minor: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                Event::WlTouchEvent(WlTouchEvent::WlTouchshapeEvent(WlTouchshapeEvent {
                    sender_id,
                    id,
                    major,
                    minor,
                }))
            }
            6u16 => {
                info!("Receive event {}", "WlTouchOrientationEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let id = unsafe { *raw_ptr };
                let orientation: f32 = 0.0;
                warn!("Fixed value has not been implemented");
                Event::WlTouchEvent(WlTouchEvent::WlTouchorientationEvent(
                    WlTouchorientationEvent {
                        sender_id,
                        id,
                        orientation,
                    },
                ))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn release(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlOutput {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlOutput {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlOutput {
        WlOutput { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlOutput(self)
    }
}
impl WlOutput {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            0u16 => {
                info!("Receive event {}", "WlOutputGeometryEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let x = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let y = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let physical_width = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let physical_height = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let subpixel = unsafe { *raw_ptr };
                parsed_len += size_of::<u32>();
                let start = parsed_len - size_of::<u32>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const u32;
                let str_len = unsafe { *raw_ptr };
                let str_len = (str_len as f64 / 4.0).ceil() as usize * 4;
                parsed_len += str_len;
                let src_ptr = msg_body[(start + size_of::<u32>())..parsed_len].as_ptr();
                let mut tmp_ptr = Vec::with_capacity(str_len);
                unsafe {
                    tmp_ptr.set_len(str_len);
                    std::ptr::copy(src_ptr, tmp_ptr.as_mut_ptr(), str_len);
                };
                let make = std::str::from_utf8(&tmp_ptr)
                    .unwrap()
                    .trim_matches('\0')
                    .to_string();
                parsed_len += size_of::<u32>();
                let start = parsed_len - size_of::<u32>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const u32;
                let str_len = unsafe { *raw_ptr };
                let str_len = (str_len as f64 / 4.0).ceil() as usize * 4;
                parsed_len += str_len;
                let src_ptr = msg_body[(start + size_of::<u32>())..parsed_len].as_ptr();
                let mut tmp_ptr = Vec::with_capacity(str_len);
                unsafe {
                    tmp_ptr.set_len(str_len);
                    std::ptr::copy(src_ptr, tmp_ptr.as_mut_ptr(), str_len);
                };
                let model = std::str::from_utf8(&tmp_ptr)
                    .unwrap()
                    .trim_matches('\0')
                    .to_string();
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let transform = unsafe { *raw_ptr };
                Event::WlOutputEvent(WlOutputEvent::WlOutputgeometryEvent(
                    WlOutputgeometryEvent {
                        sender_id,
                        x,
                        y,
                        physical_width,
                        physical_height,
                        subpixel,
                        make,
                        model,
                        transform,
                    },
                ))
            }
            1u16 => {
                info!("Receive event {}", "WlOutputModeEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Uint>();
                let start = parsed_len - size_of::<Uint>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                let flags = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let width = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let height = unsafe { *raw_ptr };
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let refresh = unsafe { *raw_ptr };
                Event::WlOutputEvent(WlOutputEvent::WlOutputmodeEvent(WlOutputmodeEvent {
                    sender_id,
                    flags,
                    width,
                    height,
                    refresh,
                }))
            }
            2u16 => {
                info!("Receive event {}", "WlOutputDoneEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                Event::WlOutputEvent(WlOutputEvent::WlOutputdoneEvent(WlOutputdoneEvent {
                    sender_id,
                }))
            }
            3u16 => {
                info!("Receive event {}", "WlOutputScaleEvent");
                #[allow(unused)]
                let mut parsed_len: usize = 0;
                parsed_len += size_of::<Int>();
                let start = parsed_len - size_of::<Int>();
                let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                let factor = unsafe { *raw_ptr };
                Event::WlOutputEvent(WlOutputEvent::WlOutputscaleEvent(WlOutputscaleEvent {
                    sender_id,
                    factor,
                }))
            }
            _ => panic!("Unknown event"),
        }
    }
    pub fn release(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlRegion {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlRegion {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlRegion {
        WlRegion { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlRegion(self)
    }
}
impl WlRegion {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            _ => panic!("Unknown event"),
        }
    }
    pub fn destroy(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn add(&self, x: Int, y: Int, width: Int, height: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn subtract(&self, x: Int, y: Int, width: Int, height: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlSubcompositor {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlSubcompositor {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlSubcompositor {
        WlSubcompositor { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlSubcompositor(self)
    }
}
impl WlSubcompositor {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            _ => panic!("Unknown event"),
        }
    }
    pub fn destroy(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn get_subsurface(&self, id: NewId, surface: Object, parent: Object) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &surface as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &parent as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
#[derive(Clone)]
pub struct WlSubsurface {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl WlRawObject for WlSubsurface {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlSubsurface {
        WlSubsurface { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlSubsurface(self)
    }
}
impl WlSubsurface {
    fn parse_event(sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match op_code {
            _ => panic!("Unknown event"),
        }
    }
    pub fn destroy(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_position(&self, x: Int, y: Int) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn place_above(&self, sibling: Object) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &sibling as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn place_below(&self, sibling: Object) {
        #[allow(unused)]
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &sibling as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        #[allow(unused)]
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_sync(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (4i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    pub fn set_desync(&self) {
        #[allow(unused)]
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        #[allow(unused)]
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (5i32 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        #[allow(unused)]
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
pub enum WlObject {
    WlDisplay(WlDisplay),
    WlRegistry(WlRegistry),
    WlCallback(WlCallback),
    WlCompositor(WlCompositor),
    WlShmPool(WlShmPool),
    WlShm(WlShm),
    WlBuffer(WlBuffer),
    WlDataOffer(WlDataOffer),
    WlDataSource(WlDataSource),
    WlDataDevice(WlDataDevice),
    WlDataDeviceManager(WlDataDeviceManager),
    WlShell(WlShell),
    WlShellSurface(WlShellSurface),
    WlSurface(WlSurface),
    WlSeat(WlSeat),
    WlPointer(WlPointer),
    WlKeyboard(WlKeyboard),
    WlTouch(WlTouch),
    WlOutput(WlOutput),
    WlRegion(WlRegion),
    WlSubcompositor(WlSubcompositor),
    WlSubsurface(WlSubsurface),
}
pub enum Event {
    WlDisplayEvent(WlDisplayEvent),
    WlRegistryEvent(WlRegistryEvent),
    WlCallbackEvent(WlCallbackEvent),
    WlCompositorEvent(WlCompositorEvent),
    WlShmPoolEvent(WlShmPoolEvent),
    WlShmEvent(WlShmEvent),
    WlBufferEvent(WlBufferEvent),
    WlDataOfferEvent(WlDataOfferEvent),
    WlDataSourceEvent(WlDataSourceEvent),
    WlDataDeviceEvent(WlDataDeviceEvent),
    WlDataDeviceManagerEvent(WlDataDeviceManagerEvent),
    WlShellEvent(WlShellEvent),
    WlShellSurfaceEvent(WlShellSurfaceEvent),
    WlSurfaceEvent(WlSurfaceEvent),
    WlSeatEvent(WlSeatEvent),
    WlPointerEvent(WlPointerEvent),
    WlKeyboardEvent(WlKeyboardEvent),
    WlTouchEvent(WlTouchEvent),
    WlOutputEvent(WlOutputEvent),
    WlRegionEvent(WlRegionEvent),
    WlSubcompositorEvent(WlSubcompositorEvent),
    WlSubsurfaceEvent(WlSubsurfaceEvent),
}
pub struct WlDisplayerrorEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub object_id: Object,
    #[allow(dead_code)]
    pub code: Uint,
    #[allow(dead_code)]
    pub message: String,
}
pub struct WlDisplaydeleteIdEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: Uint,
}
pub enum WlDisplayEvent {
    WlDisplayerrorEvent(WlDisplayerrorEvent),
    WlDisplaydeleteIdEvent(WlDisplaydeleteIdEvent),
}
pub struct WlRegistryglobalEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub name: Uint,
    #[allow(dead_code)]
    pub interface: String,
    #[allow(dead_code)]
    pub version: Uint,
}
pub struct WlRegistryglobalRemoveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub name: Uint,
}
pub enum WlRegistryEvent {
    WlRegistryglobalEvent(WlRegistryglobalEvent),
    WlRegistryglobalRemoveEvent(WlRegistryglobalRemoveEvent),
}
pub struct WlCallbackdoneEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub callback_data: Uint,
}
pub enum WlCallbackEvent {
    WlCallbackdoneEvent(WlCallbackdoneEvent),
}
pub enum WlCompositorEvent {}
pub enum WlShmPoolEvent {}
pub struct WlShmformatEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub format: Uint,
}
pub enum WlShmEvent {
    WlShmformatEvent(WlShmformatEvent),
}
pub struct WlBufferreleaseEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub enum WlBufferEvent {
    WlBufferreleaseEvent(WlBufferreleaseEvent),
}
pub struct WlDataOfferofferEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub mime_type: String,
}
pub struct WlDataOffersourceActionsEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub source_actions: Uint,
}
pub struct WlDataOfferactionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub dnd_action: Uint,
}
pub enum WlDataOfferEvent {
    WlDataOfferofferEvent(WlDataOfferofferEvent),
    WlDataOffersourceActionsEvent(WlDataOffersourceActionsEvent),
    WlDataOfferactionEvent(WlDataOfferactionEvent),
}
pub struct WlDataSourcetargetEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub mime_type: String,
}
pub struct WlDataSourcesendEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub mime_type: String,
    #[allow(dead_code)]
    pub fd: Fd,
}
pub struct WlDataSourcecancelledEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataSourcedndDropPerformedEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataSourcedndFinishedEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataSourceactionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub dnd_action: Uint,
}
pub enum WlDataSourceEvent {
    WlDataSourcetargetEvent(WlDataSourcetargetEvent),
    WlDataSourcesendEvent(WlDataSourcesendEvent),
    WlDataSourcecancelledEvent(WlDataSourcecancelledEvent),
    WlDataSourcedndDropPerformedEvent(WlDataSourcedndDropPerformedEvent),
    WlDataSourcedndFinishedEvent(WlDataSourcedndFinishedEvent),
    WlDataSourceactionEvent(WlDataSourceactionEvent),
}
pub struct WlDataDevicedataOfferEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: NewId,
}
pub struct WlDataDeviceenterEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub surface: Object,
    #[allow(dead_code)]
    pub x: Fixed,
    #[allow(dead_code)]
    pub y: Fixed,
    #[allow(dead_code)]
    pub id: Object,
}
pub struct WlDataDeviceleaveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataDevicemotionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub x: Fixed,
    #[allow(dead_code)]
    pub y: Fixed,
}
pub struct WlDataDevicedropEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataDeviceselectionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: Object,
}
pub enum WlDataDeviceEvent {
    WlDataDevicedataOfferEvent(WlDataDevicedataOfferEvent),
    WlDataDeviceenterEvent(WlDataDeviceenterEvent),
    WlDataDeviceleaveEvent(WlDataDeviceleaveEvent),
    WlDataDevicemotionEvent(WlDataDevicemotionEvent),
    WlDataDevicedropEvent(WlDataDevicedropEvent),
    WlDataDeviceselectionEvent(WlDataDeviceselectionEvent),
}
pub enum WlDataDeviceManagerEvent {}
pub enum WlShellEvent {}
pub struct WlShellSurfacepingEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
}
pub struct WlShellSurfaceconfigureEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub edges: Uint,
    #[allow(dead_code)]
    pub width: Int,
    #[allow(dead_code)]
    pub height: Int,
}
pub struct WlShellSurfacepopupDoneEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub enum WlShellSurfaceEvent {
    WlShellSurfacepingEvent(WlShellSurfacepingEvent),
    WlShellSurfaceconfigureEvent(WlShellSurfaceconfigureEvent),
    WlShellSurfacepopupDoneEvent(WlShellSurfacepopupDoneEvent),
}
pub struct WlSurfaceenterEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub output: Object,
}
pub struct WlSurfaceleaveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub output: Object,
}
pub enum WlSurfaceEvent {
    WlSurfaceenterEvent(WlSurfaceenterEvent),
    WlSurfaceleaveEvent(WlSurfaceleaveEvent),
}
pub struct WlSeatcapabilitiesEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub capabilities: Uint,
}
pub struct WlSeatnameEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub name: String,
}
pub enum WlSeatEvent {
    WlSeatcapabilitiesEvent(WlSeatcapabilitiesEvent),
    WlSeatnameEvent(WlSeatnameEvent),
}
pub struct WlPointerenterEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub surface: Object,
    #[allow(dead_code)]
    pub surface_x: Fixed,
    #[allow(dead_code)]
    pub surface_y: Fixed,
}
pub struct WlPointerleaveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub surface: Object,
}
pub struct WlPointermotionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub surface_x: Fixed,
    #[allow(dead_code)]
    pub surface_y: Fixed,
}
pub struct WlPointerbuttonEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub button: Uint,
    #[allow(dead_code)]
    pub state: Uint,
}
pub struct WlPointeraxisEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub axis: Uint,
    #[allow(dead_code)]
    pub value: Fixed,
}
pub struct WlPointerframeEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlPointeraxisSourceEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub axis_source: Uint,
}
pub struct WlPointeraxisStopEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub axis: Uint,
}
pub struct WlPointeraxisDiscreteEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub axis: Uint,
    #[allow(dead_code)]
    pub discrete: Int,
}
pub enum WlPointerEvent {
    WlPointerenterEvent(WlPointerenterEvent),
    WlPointerleaveEvent(WlPointerleaveEvent),
    WlPointermotionEvent(WlPointermotionEvent),
    WlPointerbuttonEvent(WlPointerbuttonEvent),
    WlPointeraxisEvent(WlPointeraxisEvent),
    WlPointerframeEvent(WlPointerframeEvent),
    WlPointeraxisSourceEvent(WlPointeraxisSourceEvent),
    WlPointeraxisStopEvent(WlPointeraxisStopEvent),
    WlPointeraxisDiscreteEvent(WlPointeraxisDiscreteEvent),
}
pub struct WlKeyboardkeymapEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub format: Uint,
    #[allow(dead_code)]
    pub fd: Fd,
    #[allow(dead_code)]
    pub size: Uint,
}
pub struct WlKeyboardenterEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub surface: Object,
    #[allow(dead_code)]
    pub keys: Array,
}
pub struct WlKeyboardleaveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub surface: Object,
}
pub struct WlKeyboardkeyEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub key: Uint,
    #[allow(dead_code)]
    pub state: Uint,
}
pub struct WlKeyboardmodifiersEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub mods_depressed: Uint,
    #[allow(dead_code)]
    pub mods_latched: Uint,
    #[allow(dead_code)]
    pub mods_locked: Uint,
    #[allow(dead_code)]
    pub group: Uint,
}
pub struct WlKeyboardrepeatInfoEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub rate: Int,
    #[allow(dead_code)]
    pub delay: Int,
}
pub enum WlKeyboardEvent {
    WlKeyboardkeymapEvent(WlKeyboardkeymapEvent),
    WlKeyboardenterEvent(WlKeyboardenterEvent),
    WlKeyboardleaveEvent(WlKeyboardleaveEvent),
    WlKeyboardkeyEvent(WlKeyboardkeyEvent),
    WlKeyboardmodifiersEvent(WlKeyboardmodifiersEvent),
    WlKeyboardrepeatInfoEvent(WlKeyboardrepeatInfoEvent),
}
pub struct WlTouchdownEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub surface: Object,
    #[allow(dead_code)]
    pub id: Int,
    #[allow(dead_code)]
    pub x: Fixed,
    #[allow(dead_code)]
    pub y: Fixed,
}
pub struct WlTouchupEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub id: Int,
}
pub struct WlTouchmotionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub id: Int,
    #[allow(dead_code)]
    pub x: Fixed,
    #[allow(dead_code)]
    pub y: Fixed,
}
pub struct WlTouchframeEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlTouchcancelEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlTouchshapeEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: Int,
    #[allow(dead_code)]
    pub major: Fixed,
    #[allow(dead_code)]
    pub minor: Fixed,
}
pub struct WlTouchorientationEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: Int,
    #[allow(dead_code)]
    pub orientation: Fixed,
}
pub enum WlTouchEvent {
    WlTouchdownEvent(WlTouchdownEvent),
    WlTouchupEvent(WlTouchupEvent),
    WlTouchmotionEvent(WlTouchmotionEvent),
    WlTouchframeEvent(WlTouchframeEvent),
    WlTouchcancelEvent(WlTouchcancelEvent),
    WlTouchshapeEvent(WlTouchshapeEvent),
    WlTouchorientationEvent(WlTouchorientationEvent),
}
pub struct WlOutputgeometryEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub x: Int,
    #[allow(dead_code)]
    pub y: Int,
    #[allow(dead_code)]
    pub physical_width: Int,
    #[allow(dead_code)]
    pub physical_height: Int,
    #[allow(dead_code)]
    pub subpixel: Int,
    #[allow(dead_code)]
    pub make: String,
    #[allow(dead_code)]
    pub model: String,
    #[allow(dead_code)]
    pub transform: Int,
}
pub struct WlOutputmodeEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub flags: Uint,
    #[allow(dead_code)]
    pub width: Int,
    #[allow(dead_code)]
    pub height: Int,
    #[allow(dead_code)]
    pub refresh: Int,
}
pub struct WlOutputdoneEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlOutputscaleEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub factor: Int,
}
pub enum WlOutputEvent {
    WlOutputgeometryEvent(WlOutputgeometryEvent),
    WlOutputmodeEvent(WlOutputmodeEvent),
    WlOutputdoneEvent(WlOutputdoneEvent),
    WlOutputscaleEvent(WlOutputscaleEvent),
}
pub enum WlRegionEvent {}
pub enum WlSubcompositorEvent {}
pub enum WlSubsurfaceEvent {}
impl WlObject {
    #[allow(dead_code)]
    pub fn try_get_wl_display(&self) -> Option<WlDisplay> {
        match self {
            WlObject::WlDisplay(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_registry(&self) -> Option<WlRegistry> {
        match self {
            WlObject::WlRegistry(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_callback(&self) -> Option<WlCallback> {
        match self {
            WlObject::WlCallback(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_compositor(&self) -> Option<WlCompositor> {
        match self {
            WlObject::WlCompositor(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_shm_pool(&self) -> Option<WlShmPool> {
        match self {
            WlObject::WlShmPool(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_shm(&self) -> Option<WlShm> {
        match self {
            WlObject::WlShm(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_buffer(&self) -> Option<WlBuffer> {
        match self {
            WlObject::WlBuffer(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_data_offer(&self) -> Option<WlDataOffer> {
        match self {
            WlObject::WlDataOffer(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_data_source(&self) -> Option<WlDataSource> {
        match self {
            WlObject::WlDataSource(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_data_device(&self) -> Option<WlDataDevice> {
        match self {
            WlObject::WlDataDevice(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_data_device_manager(&self) -> Option<WlDataDeviceManager> {
        match self {
            WlObject::WlDataDeviceManager(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_shell(&self) -> Option<WlShell> {
        match self {
            WlObject::WlShell(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_shell_surface(&self) -> Option<WlShellSurface> {
        match self {
            WlObject::WlShellSurface(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_surface(&self) -> Option<WlSurface> {
        match self {
            WlObject::WlSurface(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_seat(&self) -> Option<WlSeat> {
        match self {
            WlObject::WlSeat(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_pointer(&self) -> Option<WlPointer> {
        match self {
            WlObject::WlPointer(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_keyboard(&self) -> Option<WlKeyboard> {
        match self {
            WlObject::WlKeyboard(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_touch(&self) -> Option<WlTouch> {
        match self {
            WlObject::WlTouch(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_output(&self) -> Option<WlOutput> {
        match self {
            WlObject::WlOutput(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_region(&self) -> Option<WlRegion> {
        match self {
            WlObject::WlRegion(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_subcompositor(&self) -> Option<WlSubcompositor> {
        match self {
            WlObject::WlSubcompositor(item) => Some(item.clone()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn try_get_wl_subsurface(&self) -> Option<WlSubsurface> {
        match self {
            WlObject::WlSubsurface(item) => Some(item.clone()),
            _ => None,
        }
    }
}
#[repr(packed)]
struct EventHeaderPre {
    pub sender_id: u32,
    pub msg_size_and_op_code: u32,
}
#[repr(packed)]
pub struct EventHeader {
    pub sender_id: u32,
    pub msg_size: u16,
    pub op_code: u16,
}
impl EventHeaderPre {
    fn convert_to_event_header(self) -> EventHeader {
        let msg_size = {
            let size = self.msg_size_and_op_code >> 16;
            if size % 4 == 0 {
                size as u16
            } else {
                (size as f64 / 4.0).ceil() as u16 * 4
            }
        } - 8;
        EventHeader {
            sender_id: self.sender_id,
            msg_size,
            op_code: self.msg_size_and_op_code as u16,
        }
    }
}
pub trait ReadEvent {
    fn read_event(&mut self) -> Vec<(EventHeader, Vec<u8>)>;
}
impl ReadEvent for UnixSocket {
    fn read_event(&mut self) -> Vec<(EventHeader, Vec<u8>)> {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut fds: [u8; 24] = [0; 24];
        let (size, _) = self.read(&mut buffer, &mut fds);
        if size == 1024 {
            warn!("Buffer is full");
        }
        let mut ret_value = Vec::new();
        let mut read_size: usize = 0;
        while read_size < size {
            let mut event_header: [u8; size_of::<EventHeaderPre>()] =
                [0; size_of::<EventHeaderPre>()];
            unsafe {
                std::ptr::copy(
                    &buffer[read_size] as *const u8,
                    event_header.as_mut_ptr(),
                    size_of::<EventHeaderPre>(),
                );
            }
            let event_header = unsafe {
                transmute::<[u8; size_of::<EventHeaderPre>()], EventHeaderPre>(event_header)
                    .convert_to_event_header()
            };
            let msg_size = event_header.msg_size as usize;
            let mut msg_body = vec![0; event_header.msg_size as usize];
            unsafe {
                std::ptr::copy(
                    &buffer[read_size + size_of::<EventHeaderPre>()] as *const u8,
                    msg_body.as_mut_ptr(),
                    msg_size,
                );
            }
            ret_value.push((event_header, msg_body));
            read_size += size_of::<EventHeaderPre>() + msg_size;
        }
        return ret_value;
    }
}
pub trait WlRawObject {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> Self;
    fn to_enum(self) -> WlObject;
}
impl WlObject {
    pub fn parse_event(&self, sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match self {
            WlObject::WlDisplay(_obj) => WlDisplay::parse_event(sender_id, op_code, msg_body),
            WlObject::WlRegistry(_obj) => WlRegistry::parse_event(sender_id, op_code, msg_body),
            WlObject::WlCallback(_obj) => WlCallback::parse_event(sender_id, op_code, msg_body),
            WlObject::WlCompositor(_obj) => WlCompositor::parse_event(sender_id, op_code, msg_body),
            WlObject::WlShmPool(_obj) => WlShmPool::parse_event(sender_id, op_code, msg_body),
            WlObject::WlShm(_obj) => WlShm::parse_event(sender_id, op_code, msg_body),
            WlObject::WlBuffer(_obj) => WlBuffer::parse_event(sender_id, op_code, msg_body),
            WlObject::WlDataOffer(_obj) => WlDataOffer::parse_event(sender_id, op_code, msg_body),
            WlObject::WlDataSource(_obj) => WlDataSource::parse_event(sender_id, op_code, msg_body),
            WlObject::WlDataDevice(_obj) => WlDataDevice::parse_event(sender_id, op_code, msg_body),
            WlObject::WlDataDeviceManager(_obj) => {
                WlDataDeviceManager::parse_event(sender_id, op_code, msg_body)
            }
            WlObject::WlShell(_obj) => WlShell::parse_event(sender_id, op_code, msg_body),
            WlObject::WlShellSurface(_obj) => {
                WlShellSurface::parse_event(sender_id, op_code, msg_body)
            }
            WlObject::WlSurface(_obj) => WlSurface::parse_event(sender_id, op_code, msg_body),
            WlObject::WlSeat(_obj) => WlSeat::parse_event(sender_id, op_code, msg_body),
            WlObject::WlPointer(_obj) => WlPointer::parse_event(sender_id, op_code, msg_body),
            WlObject::WlKeyboard(_obj) => WlKeyboard::parse_event(sender_id, op_code, msg_body),
            WlObject::WlTouch(_obj) => WlTouch::parse_event(sender_id, op_code, msg_body),
            WlObject::WlOutput(_obj) => WlOutput::parse_event(sender_id, op_code, msg_body),
            WlObject::WlRegion(_obj) => WlRegion::parse_event(sender_id, op_code, msg_body),
            WlObject::WlSubcompositor(_obj) => {
                WlSubcompositor::parse_event(sender_id, op_code, msg_body)
            }
            WlObject::WlSubsurface(_obj) => WlSubsurface::parse_event(sender_id, op_code, msg_body),
        }
    }
}
