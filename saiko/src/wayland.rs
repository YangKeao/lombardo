use super::socket::*;
use crate::unix_socket::UnixSocket;
use std::io::Read;
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
pub trait IWlDisplay {
    fn sync(&self, callback: NewId);
    fn get_registry(&self, registry: NewId);
}
pub trait IWlRegistry {
    fn bind(&self, name: Uint, interface_name: String, interface_version: Uint, id: NewId);
}
pub trait IWlCallback {}
pub trait IWlCompositor {
    fn create_surface(&self, id: NewId);
    fn create_region(&self, id: NewId);
}
pub trait IWlShmPool {
    fn create_buffer(
        &self,
        id: NewId,
        offset: Int,
        width: Int,
        height: Int,
        stride: Int,
        format: Uint,
    );
    fn destroy(&self);
    fn resize(&self, size: Int);
}
pub trait IWlShm {
    fn create_pool(&self, id: NewId, fd: Fd, size: Int);
}
pub trait IWlBuffer {
    fn destroy(&self);
}
pub trait IWlDataOffer {
    fn accept(&self, serial: Uint, mime_type: String);
    fn receive(&self, mime_type: String, fd: Fd);
    fn destroy(&self);
    fn finish(&self);
    fn set_actions(&self, dnd_actions: Uint, preferred_action: Uint);
}
pub trait IWlDataSource {
    fn offer(&self, mime_type: String);
    fn destroy(&self);
    fn set_actions(&self, dnd_actions: Uint);
}
pub trait IWlDataDevice {
    fn start_drag(&self, source: Object, origin: Object, icon: Object, serial: Uint);
    fn set_selection(&self, source: Object, serial: Uint);
    fn release(&self);
}
pub trait IWlDataDeviceManager {
    fn create_data_source(&self, id: NewId);
    fn get_data_device(&self, id: NewId, seat: Object);
}
pub trait IWlShell {
    fn get_shell_surface(&self, id: NewId, surface: Object);
}
pub trait IWlShellSurface {
    fn pong(&self, serial: Uint);
    fn mv(&self, seat: Object, serial: Uint);
    fn resize(&self, seat: Object, serial: Uint, edges: Uint);
    fn set_toplevel(&self);
    fn set_transient(&self, parent: Object, x: Int, y: Int, flags: Uint);
    fn set_fullscreen(&self, method: Uint, framerate: Uint, output: Object);
    fn set_popup(&self, seat: Object, serial: Uint, parent: Object, x: Int, y: Int, flags: Uint);
    fn set_maximized(&self, output: Object);
    fn set_title(&self, title: String);
    fn set_class(&self, class_: String);
}
pub trait IWlSurface {
    fn destroy(&self);
    fn attach(&self, buffer: Object, x: Int, y: Int);
    fn damage(&self, x: Int, y: Int, width: Int, height: Int);
    fn frame(&self, callback: NewId);
    fn set_opaque_region(&self, region: Object);
    fn set_input_region(&self, region: Object);
    fn commit(&self);
    fn set_buffer_transform(&self, transform: Int);
    fn set_buffer_scale(&self, scale: Int);
    fn damage_buffer(&self, x: Int, y: Int, width: Int, height: Int);
}
pub trait IWlSeat {
    fn get_pointer(&self, id: NewId);
    fn get_keyboard(&self, id: NewId);
    fn get_touch(&self, id: NewId);
    fn release(&self);
}
pub trait IWlPointer {
    fn set_cursor(&self, serial: Uint, surface: Object, hotspot_x: Int, hotspot_y: Int);
    fn release(&self);
}
pub trait IWlKeyboard {
    fn release(&self);
}
pub trait IWlTouch {
    fn release(&self);
}
pub trait IWlOutput {
    fn release(&self);
}
pub trait IWlRegion {
    fn destroy(&self);
    fn add(&self, x: Int, y: Int, width: Int, height: Int);
    fn subtract(&self, x: Int, y: Int, width: Int, height: Int);
}
pub trait IWlSubcompositor {
    fn destroy(&self);
    fn get_subsurface(&self, id: NewId, surface: Object, parent: Object);
}
pub trait IWlSubsurface {
    fn destroy(&self);
    fn set_position(&self, x: Int, y: Int);
    fn place_above(&self, sibling: Object);
    fn place_below(&self, sibling: Object);
    fn set_sync(&self);
    fn set_desync(&self);
}
#[derive(Clone)]
pub struct WlDisplay {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl IWlDisplay for WlDisplay {
    fn sync(&self, callback: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &callback as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn get_registry(&self, registry: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &registry as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
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
impl IWlRegistry for WlRegistry {
    fn bind(&self, name: Uint, interface_name: String, interface_version: Uint, id: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += ((interface_name.len() + 1) as f64 / 4.0).ceil() as usize * 4 + 4;
        raw_size += size_of::<Uint>();
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &name as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
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
        written_len += buf_len + 4;
        unsafe {
            std::ptr::copy(
                &interface_version as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
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
impl IWlCallback for WlCallback {}
#[derive(Clone)]
pub struct WlCompositor {
    #[allow(dead_code)]
    pub object_id: u32,
    #[allow(dead_code)]
    pub socket: Arc<WaylandSocket>,
}
impl IWlCompositor for WlCompositor {
    fn create_surface(&self, id: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn create_region(&self, id: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
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
impl IWlShmPool for WlShmPool {
    fn create_buffer(
        &self,
        id: NewId,
        offset: Int,
        width: Int,
        height: Int,
        stride: Int,
        format: Uint,
    ) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &offset as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &stride as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &format as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn destroy(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn resize(&self, size: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &size as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
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
impl IWlShm for WlShm {
    fn create_pool(&self, id: NewId, fd: Fd, size: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
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
impl IWlBuffer for WlBuffer {
    fn destroy(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
impl IWlDataOffer for WlDataOffer {
    fn accept(&self, serial: Uint, mime_type: String) {
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += ((mime_type.len() + 1) as f64 / 4.0).ceil() as usize * 4 + 4;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
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
        written_len += buf_len + 4;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn receive(&self, mime_type: String, fd: Fd) {
        let mut raw_size = 8;
        raw_size += ((mime_type.len() + 1) as f64 / 4.0).ceil() as usize * 4 + 4;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
        written_len += buf_len + 4;
        info!("Send FD: {}", fd);
        send_fd[send_fd_num] = fd;
        send_fd_num += 1;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn destroy(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn finish(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_actions(&self, dnd_actions: Uint, preferred_action: Uint) {
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (4u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &dnd_actions as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &preferred_action as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
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
impl IWlDataSource for WlDataSource {
    fn offer(&self, mime_type: String) {
        let mut raw_size = 8;
        raw_size += ((mime_type.len() + 1) as f64 / 4.0).ceil() as usize * 4 + 4;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
        written_len += buf_len + 4;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn destroy(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_actions(&self, dnd_actions: Uint) {
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &dnd_actions as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
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
impl IWlDataDevice for WlDataDevice {
    fn start_drag(&self, source: Object, origin: Object, icon: Object, serial: Uint) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &source as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &origin as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &icon as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_selection(&self, source: Object, serial: Uint) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &source as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn release(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
impl IWlDataDeviceManager for WlDataDeviceManager {
    fn create_data_source(&self, id: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn get_data_device(&self, id: NewId, seat: Object) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &seat as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
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
impl IWlShell for WlShell {
    fn get_shell_surface(&self, id: NewId, surface: Object) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &surface as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
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
impl IWlShellSurface for WlShellSurface {
    fn pong(&self, serial: Uint) {
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn mv(&self, seat: Object, serial: Uint) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &seat as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn resize(&self, seat: Object, serial: Uint, edges: Uint) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &seat as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &edges as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_toplevel(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_transient(&self, parent: Object, x: Int, y: Int, flags: Uint) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (4u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &parent as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &flags as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_fullscreen(&self, method: Uint, framerate: Uint, output: Object) {
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (5u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &method as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &framerate as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &output as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_popup(&self, seat: Object, serial: Uint, parent: Object, x: Int, y: Int, flags: Uint) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Uint>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (6u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &seat as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &parent as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &flags as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_maximized(&self, output: Object) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (7u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &output as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_title(&self, title: String) {
        let mut raw_size = 8;
        raw_size += ((title.len() + 1) as f64 / 4.0).ceil() as usize * 4 + 4;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (8u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
        written_len += buf_len + 4;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_class(&self, class_: String) {
        let mut raw_size = 8;
        raw_size += ((class_.len() + 1) as f64 / 4.0).ceil() as usize * 4 + 4;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (9u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
impl IWlSurface for WlSurface {
    fn destroy(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn attach(&self, buffer: Object, x: Int, y: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &buffer as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn damage(&self, x: Int, y: Int, width: Int, height: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn frame(&self, callback: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &callback as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_opaque_region(&self, region: Object) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (4u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &region as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_input_region(&self, region: Object) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (5u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &region as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn commit(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (6u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_buffer_transform(&self, transform: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (7u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &transform as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_buffer_scale(&self, scale: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (8u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &scale as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn damage_buffer(&self, x: Int, y: Int, width: Int, height: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (9u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
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
impl IWlSeat for WlSeat {
    fn get_pointer(&self, id: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn get_keyboard(&self, id: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn get_touch(&self, id: NewId) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn release(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
impl IWlPointer for WlPointer {
    fn set_cursor(&self, serial: Uint, surface: Object, hotspot_x: Int, hotspot_y: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Uint>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &serial as *const Uint,
                &mut send_buffer[written_len] as *mut u8 as *mut Uint,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &surface as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &hotspot_x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &hotspot_y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn release(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
impl IWlKeyboard for WlKeyboard {
    fn release(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
impl IWlTouch for WlTouch {
    fn release(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
impl IWlOutput for WlOutput {
    fn release(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
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
impl IWlRegion for WlRegion {
    fn destroy(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn add(&self, x: Int, y: Int, width: Int, height: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn subtract(&self, x: Int, y: Int, width: Int, height: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &width as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &height as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
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
impl IWlSubcompositor for WlSubcompositor {
    fn destroy(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn get_subsurface(&self, id: NewId, surface: Object, parent: Object) {
        let mut raw_size = 8;
        raw_size += size_of::<NewId>();
        raw_size += size_of::<Object>();
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &id as *const NewId,
                &mut send_buffer[written_len] as *mut u8 as *mut NewId,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &surface as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &parent as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
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
impl IWlSubsurface for WlSubsurface {
    fn destroy(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (0u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_position(&self, x: Int, y: Int) {
        let mut raw_size = 8;
        raw_size += size_of::<Int>();
        raw_size += size_of::<Int>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (1u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &x as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            std::ptr::copy(
                &y as *const Int,
                &mut send_buffer[written_len] as *mut u8 as *mut Int,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn place_above(&self, sibling: Object) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (2u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &sibling as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn place_below(&self, sibling: Object) {
        let mut raw_size = 8;
        raw_size += size_of::<Object>();
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (3u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            std::ptr::copy(
                &sibling as *const Object,
                &mut send_buffer[written_len] as *mut u8 as *mut Object,
                1,
            );
        }
        written_len += size_of::<u32>();
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_sync(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (4u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
    fn set_desync(&self) {
        let mut raw_size = 8;
        let mut send_buffer: Vec<u8> = vec![0; raw_size];
        let mut send_fd = vec![0; 16];
        let mut send_fd_num = 0;
        unsafe {
            std::ptr::copy(
                &self.object_id as *const u32,
                &mut send_buffer[0] as *mut u8 as *mut u32,
                1,
            );
            let op_code_and_length: u32 = ((raw_size as u32) << 16) + (5u16 as u32);
            std::ptr::copy(
                &op_code_and_length as *const u32,
                &mut send_buffer[size_of::<u32>()] as *mut u8 as *mut u32,
                1,
            );
        }
        let mut written_len: usize = 8;
        unsafe {
            send_fd.set_len(send_fd_num);
        }
        self.socket.send(&send_buffer, &send_fd);
    }
}
pub trait WlRawObject {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> Self;
    fn to_enum(self) -> WlObject;
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
impl WlRawObject for WlDisplay {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDisplay {
        WlDisplay { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDisplay(self)
    }
}
impl WlRawObject for WlRegistry {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlRegistry {
        WlRegistry { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlRegistry(self)
    }
}
impl WlRawObject for WlCallback {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlCallback {
        WlCallback { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlCallback(self)
    }
}
impl WlRawObject for WlCompositor {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlCompositor {
        WlCompositor { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlCompositor(self)
    }
}
impl WlRawObject for WlShmPool {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlShmPool {
        WlShmPool { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlShmPool(self)
    }
}
impl WlRawObject for WlShm {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlShm {
        WlShm { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlShm(self)
    }
}
impl WlRawObject for WlBuffer {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlBuffer {
        WlBuffer { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlBuffer(self)
    }
}
impl WlRawObject for WlDataOffer {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDataOffer {
        WlDataOffer { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDataOffer(self)
    }
}
impl WlRawObject for WlDataSource {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDataSource {
        WlDataSource { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDataSource(self)
    }
}
impl WlRawObject for WlDataDevice {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDataDevice {
        WlDataDevice { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDataDevice(self)
    }
}
impl WlRawObject for WlDataDeviceManager {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlDataDeviceManager {
        WlDataDeviceManager { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlDataDeviceManager(self)
    }
}
impl WlRawObject for WlShell {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlShell {
        WlShell { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlShell(self)
    }
}
impl WlRawObject for WlShellSurface {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlShellSurface {
        WlShellSurface { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlShellSurface(self)
    }
}
impl WlRawObject for WlSurface {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlSurface {
        WlSurface { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlSurface(self)
    }
}
impl WlRawObject for WlSeat {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlSeat {
        WlSeat { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlSeat(self)
    }
}
impl WlRawObject for WlPointer {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlPointer {
        WlPointer { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlPointer(self)
    }
}
impl WlRawObject for WlKeyboard {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlKeyboard {
        WlKeyboard { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlKeyboard(self)
    }
}
impl WlRawObject for WlTouch {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlTouch {
        WlTouch { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlTouch(self)
    }
}
impl WlRawObject for WlOutput {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlOutput {
        WlOutput { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlOutput(self)
    }
}
impl WlRawObject for WlRegion {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlRegion {
        WlRegion { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlRegion(self)
    }
}
impl WlRawObject for WlSubcompositor {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlSubcompositor {
        WlSubcompositor { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlSubcompositor(self)
    }
}
impl WlRawObject for WlSubsurface {
    fn new(object_id: u32, socket: Arc<WaylandSocket>) -> WlSubsurface {
        WlSubsurface { object_id, socket }
    }
    fn to_enum(self) -> WlObject {
        WlObject::WlSubsurface(self)
    }
}
pub struct WlDisplayErrorEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub object_id: Object,
    #[allow(dead_code)]
    pub code: Uint,
    #[allow(dead_code)]
    pub message: String,
}
pub struct WlDisplayDeleteIdEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: Uint,
}
pub struct WlRegistryGlobalEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub name: Uint,
    #[allow(dead_code)]
    pub interface: String,
    #[allow(dead_code)]
    pub version: Uint,
}
pub struct WlRegistryGlobalRemoveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub name: Uint,
}
pub struct WlCallbackDoneEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub callback_data: Uint,
}
pub struct WlShmFormatEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub format: Uint,
}
pub struct WlBufferReleaseEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataOfferOfferEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub mime_type: String,
}
pub struct WlDataOfferSourceActionsEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub source_actions: Uint,
}
pub struct WlDataOfferActionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub dnd_action: Uint,
}
pub struct WlDataSourceTargetEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub mime_type: String,
}
pub struct WlDataSourceSendEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub mime_type: String,
    #[allow(dead_code)]
    pub fd: Fd,
}
pub struct WlDataSourceCancelledEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataSourceDndDropPerformedEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataSourceDndFinishedEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataSourceActionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub dnd_action: Uint,
}
pub struct WlDataDeviceDataOfferEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: NewId,
}
pub struct WlDataDeviceEnterEvent {
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
pub struct WlDataDeviceLeaveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataDeviceMotionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub x: Fixed,
    #[allow(dead_code)]
    pub y: Fixed,
}
pub struct WlDataDeviceDropEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlDataDeviceSelectionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: Object,
}
pub struct WlShellSurfacePingEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
}
pub struct WlShellSurfaceConfigureEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub edges: Uint,
    #[allow(dead_code)]
    pub width: Int,
    #[allow(dead_code)]
    pub height: Int,
}
pub struct WlShellSurfacePopupDoneEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlSurfaceEnterEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub output: Object,
}
pub struct WlSurfaceLeaveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub output: Object,
}
pub struct WlSeatCapabilitiesEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub capabilities: Uint,
}
pub struct WlSeatNameEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub name: String,
}
pub struct WlPointerEnterEvent {
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
pub struct WlPointerLeaveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub surface: Object,
}
pub struct WlPointerMotionEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub surface_x: Fixed,
    #[allow(dead_code)]
    pub surface_y: Fixed,
}
pub struct WlPointerButtonEvent {
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
pub struct WlPointerAxisEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub axis: Uint,
    #[allow(dead_code)]
    pub value: Fixed,
}
pub struct WlPointerFrameEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlPointerAxisSourceEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub axis_source: Uint,
}
pub struct WlPointerAxisStopEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub axis: Uint,
}
pub struct WlPointerAxisDiscreteEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub axis: Uint,
    #[allow(dead_code)]
    pub discrete: Int,
}
pub struct WlKeyboardKeymapEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub format: Uint,
    #[allow(dead_code)]
    pub fd: Fd,
    #[allow(dead_code)]
    pub size: Uint,
}
pub struct WlKeyboardEnterEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub surface: Object,
    #[allow(dead_code)]
    pub keys: Array,
}
pub struct WlKeyboardLeaveEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub surface: Object,
}
pub struct WlKeyboardKeyEvent {
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
pub struct WlKeyboardModifiersEvent {
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
pub struct WlKeyboardRepeatInfoEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub rate: Int,
    #[allow(dead_code)]
    pub delay: Int,
}
pub struct WlTouchDownEvent {
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
pub struct WlTouchUpEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub serial: Uint,
    #[allow(dead_code)]
    pub time: Uint,
    #[allow(dead_code)]
    pub id: Int,
}
pub struct WlTouchMotionEvent {
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
pub struct WlTouchFrameEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlTouchCancelEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlTouchShapeEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: Int,
    #[allow(dead_code)]
    pub major: Fixed,
    #[allow(dead_code)]
    pub minor: Fixed,
}
pub struct WlTouchOrientationEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub id: Int,
    #[allow(dead_code)]
    pub orientation: Fixed,
}
pub struct WlOutputGeometryEvent {
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
pub struct WlOutputModeEvent {
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
pub struct WlOutputDoneEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
}
pub struct WlOutputScaleEvent {
    #[allow(dead_code)]
    pub sender_id: u32,
    #[allow(dead_code)]
    pub factor: Int,
}
#[allow(dead_code)]
pub enum WlDisplayEvent {
    WlDisplayErrorEvent(WlDisplayErrorEvent),
    WlDisplayDeleteIdEvent(WlDisplayDeleteIdEvent),
    None,
}
#[allow(dead_code)]
pub enum WlRegistryEvent {
    WlRegistryGlobalEvent(WlRegistryGlobalEvent),
    WlRegistryGlobalRemoveEvent(WlRegistryGlobalRemoveEvent),
    None,
}
#[allow(dead_code)]
pub enum WlCallbackEvent {
    WlCallbackDoneEvent(WlCallbackDoneEvent),
    None,
}
#[allow(dead_code)]
pub enum WlCompositorEvent {
    None,
}
#[allow(dead_code)]
pub enum WlShmPoolEvent {
    None,
}
#[allow(dead_code)]
pub enum WlShmEvent {
    WlShmFormatEvent(WlShmFormatEvent),
    None,
}
#[allow(dead_code)]
pub enum WlBufferEvent {
    WlBufferReleaseEvent(WlBufferReleaseEvent),
    None,
}
#[allow(dead_code)]
pub enum WlDataOfferEvent {
    WlDataOfferOfferEvent(WlDataOfferOfferEvent),
    WlDataOfferSourceActionsEvent(WlDataOfferSourceActionsEvent),
    WlDataOfferActionEvent(WlDataOfferActionEvent),
    None,
}
#[allow(dead_code)]
pub enum WlDataSourceEvent {
    WlDataSourceTargetEvent(WlDataSourceTargetEvent),
    WlDataSourceSendEvent(WlDataSourceSendEvent),
    WlDataSourceCancelledEvent(WlDataSourceCancelledEvent),
    WlDataSourceDndDropPerformedEvent(WlDataSourceDndDropPerformedEvent),
    WlDataSourceDndFinishedEvent(WlDataSourceDndFinishedEvent),
    WlDataSourceActionEvent(WlDataSourceActionEvent),
    None,
}
#[allow(dead_code)]
pub enum WlDataDeviceEvent {
    WlDataDeviceDataOfferEvent(WlDataDeviceDataOfferEvent),
    WlDataDeviceEnterEvent(WlDataDeviceEnterEvent),
    WlDataDeviceLeaveEvent(WlDataDeviceLeaveEvent),
    WlDataDeviceMotionEvent(WlDataDeviceMotionEvent),
    WlDataDeviceDropEvent(WlDataDeviceDropEvent),
    WlDataDeviceSelectionEvent(WlDataDeviceSelectionEvent),
    None,
}
#[allow(dead_code)]
pub enum WlDataDeviceManagerEvent {
    None,
}
#[allow(dead_code)]
pub enum WlShellEvent {
    None,
}
#[allow(dead_code)]
pub enum WlShellSurfaceEvent {
    WlShellSurfacePingEvent(WlShellSurfacePingEvent),
    WlShellSurfaceConfigureEvent(WlShellSurfaceConfigureEvent),
    WlShellSurfacePopupDoneEvent(WlShellSurfacePopupDoneEvent),
    None,
}
#[allow(dead_code)]
pub enum WlSurfaceEvent {
    WlSurfaceEnterEvent(WlSurfaceEnterEvent),
    WlSurfaceLeaveEvent(WlSurfaceLeaveEvent),
    None,
}
#[allow(dead_code)]
pub enum WlSeatEvent {
    WlSeatCapabilitiesEvent(WlSeatCapabilitiesEvent),
    WlSeatNameEvent(WlSeatNameEvent),
    None,
}
#[allow(dead_code)]
pub enum WlPointerEvent {
    WlPointerEnterEvent(WlPointerEnterEvent),
    WlPointerLeaveEvent(WlPointerLeaveEvent),
    WlPointerMotionEvent(WlPointerMotionEvent),
    WlPointerButtonEvent(WlPointerButtonEvent),
    WlPointerAxisEvent(WlPointerAxisEvent),
    WlPointerFrameEvent(WlPointerFrameEvent),
    WlPointerAxisSourceEvent(WlPointerAxisSourceEvent),
    WlPointerAxisStopEvent(WlPointerAxisStopEvent),
    WlPointerAxisDiscreteEvent(WlPointerAxisDiscreteEvent),
    None,
}
#[allow(dead_code)]
pub enum WlKeyboardEvent {
    WlKeyboardKeymapEvent(WlKeyboardKeymapEvent),
    WlKeyboardEnterEvent(WlKeyboardEnterEvent),
    WlKeyboardLeaveEvent(WlKeyboardLeaveEvent),
    WlKeyboardKeyEvent(WlKeyboardKeyEvent),
    WlKeyboardModifiersEvent(WlKeyboardModifiersEvent),
    WlKeyboardRepeatInfoEvent(WlKeyboardRepeatInfoEvent),
    None,
}
#[allow(dead_code)]
pub enum WlTouchEvent {
    WlTouchDownEvent(WlTouchDownEvent),
    WlTouchUpEvent(WlTouchUpEvent),
    WlTouchMotionEvent(WlTouchMotionEvent),
    WlTouchFrameEvent(WlTouchFrameEvent),
    WlTouchCancelEvent(WlTouchCancelEvent),
    WlTouchShapeEvent(WlTouchShapeEvent),
    WlTouchOrientationEvent(WlTouchOrientationEvent),
    None,
}
#[allow(dead_code)]
pub enum WlOutputEvent {
    WlOutputGeometryEvent(WlOutputGeometryEvent),
    WlOutputModeEvent(WlOutputModeEvent),
    WlOutputDoneEvent(WlOutputDoneEvent),
    WlOutputScaleEvent(WlOutputScaleEvent),
    None,
}
#[allow(dead_code)]
pub enum WlRegionEvent {
    None,
}
#[allow(dead_code)]
pub enum WlSubcompositorEvent {
    None,
}
#[allow(dead_code)]
pub enum WlSubsurfaceEvent {
    None,
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
        let (size, num_fds) = self.read(&mut buffer, &mut fds);
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
impl WlObject {
    pub fn parse_event(&self, sender_id: u32, op_code: u16, msg_body: Vec<u8>) -> Event {
        match self {
            WlObject::WlDisplay(_obj) => Event::WlDisplayEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlDisplayErrorEvent");
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
                    WlDisplayEvent::WlDisplayErrorEvent(WlDisplayErrorEvent {
                        sender_id,
                        object_id,
                        code,
                        message,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlDisplayDeleteIdEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let id = unsafe { *raw_ptr };
                    WlDisplayEvent::WlDisplayDeleteIdEvent(WlDisplayDeleteIdEvent { sender_id, id })
                }
                _ => {
                    warn!("No such op_code");
                    WlDisplayEvent::None
                }
            }),
            WlObject::WlRegistry(_obj) => Event::WlRegistryEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlRegistryGlobalEvent");
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
                    WlRegistryEvent::WlRegistryGlobalEvent(WlRegistryGlobalEvent {
                        sender_id,
                        name,
                        interface,
                        version,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlRegistryGlobalRemoveEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let name = unsafe { *raw_ptr };
                    WlRegistryEvent::WlRegistryGlobalRemoveEvent(WlRegistryGlobalRemoveEvent {
                        sender_id,
                        name,
                    })
                }
                _ => {
                    warn!("No such op_code");
                    WlRegistryEvent::None
                }
            }),
            WlObject::WlCallback(_obj) => Event::WlCallbackEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlCallbackDoneEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let callback_data = unsafe { *raw_ptr };
                    WlCallbackEvent::WlCallbackDoneEvent(WlCallbackDoneEvent {
                        sender_id,
                        callback_data,
                    })
                }
                _ => {
                    warn!("No such op_code");
                    WlCallbackEvent::None
                }
            }),
            WlObject::WlCompositor(_obj) => Event::WlCompositorEvent(match op_code {
                _ => {
                    warn!("No such op_code");
                    WlCompositorEvent::None
                }
            }),
            WlObject::WlShmPool(_obj) => Event::WlShmPoolEvent(match op_code {
                _ => {
                    warn!("No such op_code");
                    WlShmPoolEvent::None
                }
            }),
            WlObject::WlShm(_obj) => Event::WlShmEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlShmFormatEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let format = unsafe { *raw_ptr };
                    WlShmEvent::WlShmFormatEvent(WlShmFormatEvent { sender_id, format })
                }
                _ => {
                    warn!("No such op_code");
                    WlShmEvent::None
                }
            }),
            WlObject::WlBuffer(_obj) => Event::WlBufferEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlBufferReleaseEvent");
                    let mut parsed_len: usize = 0;
                    WlBufferEvent::WlBufferReleaseEvent(WlBufferReleaseEvent { sender_id })
                }
                _ => {
                    warn!("No such op_code");
                    WlBufferEvent::None
                }
            }),
            WlObject::WlDataOffer(_obj) => Event::WlDataOfferEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlDataOfferOfferEvent");
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
                    WlDataOfferEvent::WlDataOfferOfferEvent(WlDataOfferOfferEvent {
                        sender_id,
                        mime_type,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlDataOfferSourceActionsEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let source_actions = unsafe { *raw_ptr };
                    WlDataOfferEvent::WlDataOfferSourceActionsEvent(WlDataOfferSourceActionsEvent {
                        sender_id,
                        source_actions,
                    })
                }
                2u16 => {
                    info!("Receive event {}", "WlDataOfferActionEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let dnd_action = unsafe { *raw_ptr };
                    WlDataOfferEvent::WlDataOfferActionEvent(WlDataOfferActionEvent {
                        sender_id,
                        dnd_action,
                    })
                }
                _ => {
                    warn!("No such op_code");
                    WlDataOfferEvent::None
                }
            }),
            WlObject::WlDataSource(_obj) => Event::WlDataSourceEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlDataSourceTargetEvent");
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
                    WlDataSourceEvent::WlDataSourceTargetEvent(WlDataSourceTargetEvent {
                        sender_id,
                        mime_type,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlDataSourceSendEvent");
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
                    WlDataSourceEvent::WlDataSourceSendEvent(WlDataSourceSendEvent {
                        sender_id,
                        mime_type,
                        fd,
                    })
                }
                2u16 => {
                    info!("Receive event {}", "WlDataSourceCancelledEvent");
                    let mut parsed_len: usize = 0;
                    WlDataSourceEvent::WlDataSourceCancelledEvent(WlDataSourceCancelledEvent {
                        sender_id,
                    })
                }
                3u16 => {
                    info!("Receive event {}", "WlDataSourceDndDropPerformedEvent");
                    let mut parsed_len: usize = 0;
                    WlDataSourceEvent::WlDataSourceDndDropPerformedEvent(
                        WlDataSourceDndDropPerformedEvent { sender_id },
                    )
                }
                4u16 => {
                    info!("Receive event {}", "WlDataSourceDndFinishedEvent");
                    let mut parsed_len: usize = 0;
                    WlDataSourceEvent::WlDataSourceDndFinishedEvent(WlDataSourceDndFinishedEvent {
                        sender_id,
                    })
                }
                5u16 => {
                    info!("Receive event {}", "WlDataSourceActionEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let dnd_action = unsafe { *raw_ptr };
                    WlDataSourceEvent::WlDataSourceActionEvent(WlDataSourceActionEvent {
                        sender_id,
                        dnd_action,
                    })
                }
                _ => {
                    warn!("No such op_code");
                    WlDataSourceEvent::None
                }
            }),
            WlObject::WlDataDevice(_obj) => Event::WlDataDeviceEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlDataDeviceDataOfferEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<NewId>();
                    let start = parsed_len - size_of::<NewId>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const NewId;
                    let id = unsafe { *raw_ptr };
                    WlDataDeviceEvent::WlDataDeviceDataOfferEvent(WlDataDeviceDataOfferEvent {
                        sender_id,
                        id,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlDataDeviceEnterEvent");
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
                    unimplemented!();
                    let y: f32 = 0.0;
                    unimplemented!();
                    parsed_len += size_of::<Object>();
                    let start = parsed_len - size_of::<Object>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                    let id = unsafe { *raw_ptr };
                    WlDataDeviceEvent::WlDataDeviceEnterEvent(WlDataDeviceEnterEvent {
                        sender_id,
                        serial,
                        surface,
                        x,
                        y,
                        id,
                    })
                }
                2u16 => {
                    info!("Receive event {}", "WlDataDeviceLeaveEvent");
                    let mut parsed_len: usize = 0;
                    WlDataDeviceEvent::WlDataDeviceLeaveEvent(WlDataDeviceLeaveEvent { sender_id })
                }
                3u16 => {
                    info!("Receive event {}", "WlDataDeviceMotionEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let time = unsafe { *raw_ptr };
                    let x: f32 = 0.0;
                    unimplemented!();
                    let y: f32 = 0.0;
                    unimplemented!();
                    WlDataDeviceEvent::WlDataDeviceMotionEvent(WlDataDeviceMotionEvent {
                        sender_id,
                        time,
                        x,
                        y,
                    })
                }
                4u16 => {
                    info!("Receive event {}", "WlDataDeviceDropEvent");
                    let mut parsed_len: usize = 0;
                    WlDataDeviceEvent::WlDataDeviceDropEvent(WlDataDeviceDropEvent { sender_id })
                }
                5u16 => {
                    info!("Receive event {}", "WlDataDeviceSelectionEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Object>();
                    let start = parsed_len - size_of::<Object>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                    let id = unsafe { *raw_ptr };
                    WlDataDeviceEvent::WlDataDeviceSelectionEvent(WlDataDeviceSelectionEvent {
                        sender_id,
                        id,
                    })
                }
                _ => {
                    warn!("No such op_code");
                    WlDataDeviceEvent::None
                }
            }),
            WlObject::WlDataDeviceManager(_obj) => Event::WlDataDeviceManagerEvent(match op_code {
                _ => {
                    warn!("No such op_code");
                    WlDataDeviceManagerEvent::None
                }
            }),
            WlObject::WlShell(_obj) => Event::WlShellEvent(match op_code {
                _ => {
                    warn!("No such op_code");
                    WlShellEvent::None
                }
            }),
            WlObject::WlShellSurface(_obj) => Event::WlShellSurfaceEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlShellSurfacePingEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let serial = unsafe { *raw_ptr };
                    WlShellSurfaceEvent::WlShellSurfacePingEvent(WlShellSurfacePingEvent {
                        sender_id,
                        serial,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlShellSurfaceConfigureEvent");
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
                    WlShellSurfaceEvent::WlShellSurfaceConfigureEvent(
                        WlShellSurfaceConfigureEvent {
                            sender_id,
                            edges,
                            width,
                            height,
                        },
                    )
                }
                2u16 => {
                    info!("Receive event {}", "WlShellSurfacePopupDoneEvent");
                    let mut parsed_len: usize = 0;
                    WlShellSurfaceEvent::WlShellSurfacePopupDoneEvent(
                        WlShellSurfacePopupDoneEvent { sender_id },
                    )
                }
                _ => {
                    warn!("No such op_code");
                    WlShellSurfaceEvent::None
                }
            }),
            WlObject::WlSurface(_obj) => Event::WlSurfaceEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlSurfaceEnterEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Object>();
                    let start = parsed_len - size_of::<Object>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                    let output = unsafe { *raw_ptr };
                    WlSurfaceEvent::WlSurfaceEnterEvent(WlSurfaceEnterEvent { sender_id, output })
                }
                1u16 => {
                    info!("Receive event {}", "WlSurfaceLeaveEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Object>();
                    let start = parsed_len - size_of::<Object>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                    let output = unsafe { *raw_ptr };
                    WlSurfaceEvent::WlSurfaceLeaveEvent(WlSurfaceLeaveEvent { sender_id, output })
                }
                _ => {
                    warn!("No such op_code");
                    WlSurfaceEvent::None
                }
            }),
            WlObject::WlSeat(_obj) => Event::WlSeatEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlSeatCapabilitiesEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let capabilities = unsafe { *raw_ptr };
                    WlSeatEvent::WlSeatCapabilitiesEvent(WlSeatCapabilitiesEvent {
                        sender_id,
                        capabilities,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlSeatNameEvent");
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
                    WlSeatEvent::WlSeatNameEvent(WlSeatNameEvent { sender_id, name })
                }
                _ => {
                    warn!("No such op_code");
                    WlSeatEvent::None
                }
            }),
            WlObject::WlPointer(_obj) => Event::WlPointerEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlPointerEnterEvent");
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
                    unimplemented!();
                    let surface_y: f32 = 0.0;
                    unimplemented!();
                    WlPointerEvent::WlPointerEnterEvent(WlPointerEnterEvent {
                        sender_id,
                        serial,
                        surface,
                        surface_x,
                        surface_y,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlPointerLeaveEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let serial = unsafe { *raw_ptr };
                    parsed_len += size_of::<Object>();
                    let start = parsed_len - size_of::<Object>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                    let surface = unsafe { *raw_ptr };
                    WlPointerEvent::WlPointerLeaveEvent(WlPointerLeaveEvent {
                        sender_id,
                        serial,
                        surface,
                    })
                }
                2u16 => {
                    info!("Receive event {}", "WlPointerMotionEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let time = unsafe { *raw_ptr };
                    let surface_x: f32 = 0.0;
                    unimplemented!();
                    let surface_y: f32 = 0.0;
                    unimplemented!();
                    WlPointerEvent::WlPointerMotionEvent(WlPointerMotionEvent {
                        sender_id,
                        time,
                        surface_x,
                        surface_y,
                    })
                }
                3u16 => {
                    info!("Receive event {}", "WlPointerButtonEvent");
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
                    WlPointerEvent::WlPointerButtonEvent(WlPointerButtonEvent {
                        sender_id,
                        serial,
                        time,
                        button,
                        state,
                    })
                }
                4u16 => {
                    info!("Receive event {}", "WlPointerAxisEvent");
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
                    unimplemented!();
                    WlPointerEvent::WlPointerAxisEvent(WlPointerAxisEvent {
                        sender_id,
                        time,
                        axis,
                        value,
                    })
                }
                5u16 => {
                    info!("Receive event {}", "WlPointerFrameEvent");
                    let mut parsed_len: usize = 0;
                    WlPointerEvent::WlPointerFrameEvent(WlPointerFrameEvent { sender_id })
                }
                6u16 => {
                    info!("Receive event {}", "WlPointerAxisSourceEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let axis_source = unsafe { *raw_ptr };
                    WlPointerEvent::WlPointerAxisSourceEvent(WlPointerAxisSourceEvent {
                        sender_id,
                        axis_source,
                    })
                }
                7u16 => {
                    info!("Receive event {}", "WlPointerAxisStopEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let time = unsafe { *raw_ptr };
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let axis = unsafe { *raw_ptr };
                    WlPointerEvent::WlPointerAxisStopEvent(WlPointerAxisStopEvent {
                        sender_id,
                        time,
                        axis,
                    })
                }
                8u16 => {
                    info!("Receive event {}", "WlPointerAxisDiscreteEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let axis = unsafe { *raw_ptr };
                    parsed_len += size_of::<Int>();
                    let start = parsed_len - size_of::<Int>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                    let discrete = unsafe { *raw_ptr };
                    WlPointerEvent::WlPointerAxisDiscreteEvent(WlPointerAxisDiscreteEvent {
                        sender_id,
                        axis,
                        discrete,
                    })
                }
                _ => {
                    warn!("No such op_code");
                    WlPointerEvent::None
                }
            }),
            WlObject::WlKeyboard(_obj) => Event::WlKeyboardEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlKeyboardKeymapEvent");
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
                    WlKeyboardEvent::WlKeyboardKeymapEvent(WlKeyboardKeymapEvent {
                        sender_id,
                        format,
                        fd,
                        size,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlKeyboardEnterEvent");
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
                    unimplemented!();
                    WlKeyboardEvent::WlKeyboardEnterEvent(WlKeyboardEnterEvent {
                        sender_id,
                        serial,
                        surface,
                        keys,
                    })
                }
                2u16 => {
                    info!("Receive event {}", "WlKeyboardLeaveEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Uint>();
                    let start = parsed_len - size_of::<Uint>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Uint;
                    let serial = unsafe { *raw_ptr };
                    parsed_len += size_of::<Object>();
                    let start = parsed_len - size_of::<Object>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Object;
                    let surface = unsafe { *raw_ptr };
                    WlKeyboardEvent::WlKeyboardLeaveEvent(WlKeyboardLeaveEvent {
                        sender_id,
                        serial,
                        surface,
                    })
                }
                3u16 => {
                    info!("Receive event {}", "WlKeyboardKeyEvent");
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
                    WlKeyboardEvent::WlKeyboardKeyEvent(WlKeyboardKeyEvent {
                        sender_id,
                        serial,
                        time,
                        key,
                        state,
                    })
                }
                4u16 => {
                    info!("Receive event {}", "WlKeyboardModifiersEvent");
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
                    WlKeyboardEvent::WlKeyboardModifiersEvent(WlKeyboardModifiersEvent {
                        sender_id,
                        serial,
                        mods_depressed,
                        mods_latched,
                        mods_locked,
                        group,
                    })
                }
                5u16 => {
                    info!("Receive event {}", "WlKeyboardRepeatInfoEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Int>();
                    let start = parsed_len - size_of::<Int>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                    let rate = unsafe { *raw_ptr };
                    parsed_len += size_of::<Int>();
                    let start = parsed_len - size_of::<Int>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                    let delay = unsafe { *raw_ptr };
                    WlKeyboardEvent::WlKeyboardRepeatInfoEvent(WlKeyboardRepeatInfoEvent {
                        sender_id,
                        rate,
                        delay,
                    })
                }
                _ => {
                    warn!("No such op_code");
                    WlKeyboardEvent::None
                }
            }),
            WlObject::WlTouch(_obj) => Event::WlTouchEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlTouchDownEvent");
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
                    unimplemented!();
                    let y: f32 = 0.0;
                    unimplemented!();
                    WlTouchEvent::WlTouchDownEvent(WlTouchDownEvent {
                        sender_id,
                        serial,
                        time,
                        surface,
                        id,
                        x,
                        y,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlTouchUpEvent");
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
                    WlTouchEvent::WlTouchUpEvent(WlTouchUpEvent {
                        sender_id,
                        serial,
                        time,
                        id,
                    })
                }
                2u16 => {
                    info!("Receive event {}", "WlTouchMotionEvent");
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
                    unimplemented!();
                    let y: f32 = 0.0;
                    unimplemented!();
                    WlTouchEvent::WlTouchMotionEvent(WlTouchMotionEvent {
                        sender_id,
                        time,
                        id,
                        x,
                        y,
                    })
                }
                3u16 => {
                    info!("Receive event {}", "WlTouchFrameEvent");
                    let mut parsed_len: usize = 0;
                    WlTouchEvent::WlTouchFrameEvent(WlTouchFrameEvent { sender_id })
                }
                4u16 => {
                    info!("Receive event {}", "WlTouchCancelEvent");
                    let mut parsed_len: usize = 0;
                    WlTouchEvent::WlTouchCancelEvent(WlTouchCancelEvent { sender_id })
                }
                5u16 => {
                    info!("Receive event {}", "WlTouchShapeEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Int>();
                    let start = parsed_len - size_of::<Int>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                    let id = unsafe { *raw_ptr };
                    let major: f32 = 0.0;
                    unimplemented!();
                    let minor: f32 = 0.0;
                    unimplemented!();
                    WlTouchEvent::WlTouchShapeEvent(WlTouchShapeEvent {
                        sender_id,
                        id,
                        major,
                        minor,
                    })
                }
                6u16 => {
                    info!("Receive event {}", "WlTouchOrientationEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Int>();
                    let start = parsed_len - size_of::<Int>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                    let id = unsafe { *raw_ptr };
                    let orientation: f32 = 0.0;
                    unimplemented!();
                    WlTouchEvent::WlTouchOrientationEvent(WlTouchOrientationEvent {
                        sender_id,
                        id,
                        orientation,
                    })
                }
                _ => {
                    warn!("No such op_code");
                    WlTouchEvent::None
                }
            }),
            WlObject::WlOutput(_obj) => Event::WlOutputEvent(match op_code {
                0u16 => {
                    info!("Receive event {}", "WlOutputGeometryEvent");
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
                    WlOutputEvent::WlOutputGeometryEvent(WlOutputGeometryEvent {
                        sender_id,
                        x,
                        y,
                        physical_width,
                        physical_height,
                        subpixel,
                        make,
                        model,
                        transform,
                    })
                }
                1u16 => {
                    info!("Receive event {}", "WlOutputModeEvent");
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
                    WlOutputEvent::WlOutputModeEvent(WlOutputModeEvent {
                        sender_id,
                        flags,
                        width,
                        height,
                        refresh,
                    })
                }
                2u16 => {
                    info!("Receive event {}", "WlOutputDoneEvent");
                    let mut parsed_len: usize = 0;
                    WlOutputEvent::WlOutputDoneEvent(WlOutputDoneEvent { sender_id })
                }
                3u16 => {
                    info!("Receive event {}", "WlOutputScaleEvent");
                    let mut parsed_len: usize = 0;
                    parsed_len += size_of::<Int>();
                    let start = parsed_len - size_of::<Int>();
                    let raw_ptr = msg_body[start..parsed_len].as_ptr() as *const Int;
                    let factor = unsafe { *raw_ptr };
                    WlOutputEvent::WlOutputScaleEvent(WlOutputScaleEvent { sender_id, factor })
                }
                _ => {
                    warn!("No such op_code");
                    WlOutputEvent::None
                }
            }),
            WlObject::WlRegion(_obj) => Event::WlRegionEvent(match op_code {
                _ => {
                    warn!("No such op_code");
                    WlRegionEvent::None
                }
            }),
            WlObject::WlSubcompositor(_obj) => Event::WlSubcompositorEvent(match op_code {
                _ => {
                    warn!("No such op_code");
                    WlSubcompositorEvent::None
                }
            }),
            WlObject::WlSubsurface(_obj) => Event::WlSubsurfaceEvent(match op_code {
                _ => {
                    warn!("No such op_code");
                    WlSubsurfaceEvent::None
                }
            }),
        }
    }
}
