use byteorder::WriteBytesExt;
use nix::sys::socket;
use nix::sys::socket::ControlMessage;
use nix::sys::socket::MsgFlags;
use nix::sys::socket::{recvmsg, sendmsg, CmsgSpace};
use nix::sys::uio::IoVec;
use std::io::Cursor;
use std::os::unix::io::RawFd;

#[derive(Clone)]
pub struct UnixSocket {
    fd: RawFd,
}

impl UnixSocket {
    pub fn connect(path: std::path::PathBuf) -> UnixSocket {
        let fd = socket::socket(
            socket::AddressFamily::Unix,
            socket::SockType::Stream,
            socket::SockFlag::SOCK_CLOEXEC,
            None,
        )
        .unwrap();

        socket::connect(
            fd,
            &socket::SockAddr::Unix(socket::UnixAddr::new(&path).unwrap()),
        );

        return UnixSocket { fd };
    }

    pub fn write(&mut self, buffer: &[u8], fd: Option<RawFd>) {
        match fd {
            Some(fd) => sendmsg(
                self.fd,
                &[IoVec::from_slice(buffer)],
                &[ControlMessage::ScmRights(&[fd])],
                MsgFlags::all(),
                None,
            )
            .unwrap(),
            None => sendmsg(
                self.fd,
                &[IoVec::from_slice(buffer)],
                &[ControlMessage::ScmRights(&[])],
                MsgFlags::all(),
                None,
            )
            .unwrap(),
        };
    }

    pub fn read(&mut self, buffer: &mut [u8], fds: &mut [u8]) -> (usize, i32) {
        let mut iov: [IoVec<&mut [u8]>; 1] = [IoVec::from_mut_slice(buffer); 1];
        let mut cmsg: CmsgSpace<[RawFd; 1]> = CmsgSpace::new();

        let msg = recvmsg(self.fd, &iov, Some(&mut cmsg), MsgFlags::all()).unwrap();

        let mut num_fds = 0;
        let mut buf = Cursor::new(fds);
        for cmsg in msg.cmsgs() {
            match cmsg {
                ControlMessage::ScmRights(newfds) => {
                    buf.write_i32::<byteorder::NativeEndian>(newfds[0]).unwrap();
                    num_fds += 1;
                }
                _ => {}
            }
        }

        return (msg.bytes, num_fds);
    }

    pub fn shutdown(&mut self) {
        unimplemented!()
    }
}
