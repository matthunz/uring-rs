use std::fmt;
use std::io::{IoSliceMut, IoSlice};
use std::os::unix::io::RawFd;

#[repr(C)]
pub union BufIndex {
    pub buf_index: u16,
    pub pad: [u64; 3],
}

#[repr(C)]
pub union CommandFlags {
    pub rw_flags: i32,
    pub sync_flags: u32,
    pub poll_events: u16,
    pub sync_range_flags: u32,
    pub fmsg_flags: u32,
}

#[repr(C)]
pub struct SubmissionQueueEntry {
    pub opcode: u8,
    pub flags: u8,
    pub priority: u16,
    pub fd: RawFd,
    pub offset: u64,
    pub addr: u64,
    pub len: u32,
    pub cmd_flags: CommandFlags,
    pub user_data: u64,
    pub buf_index: BufIndex,
}

impl SubmissionQueueEntry {
    pub fn prep_read_vectored(&mut self, fd: RawFd, buf: &mut [IoSliceMut]) {
        self.opcode = 1;
        self.fd = fd;
        self.addr = buf.as_mut_ptr() as u64;
        self.len = buf.len() as u32;
    }
    pub fn prep_write_vectored(&mut self, fd: RawFd, buf: &[IoSlice]) {
        self.opcode = 2;
        self.fd = fd;
        self.addr = buf.as_ptr() as u64;
        self.len = buf.len() as u32;
    }
}

impl fmt::Debug for SubmissionQueueEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SubmissionQueueEntry")
            .field("opcode", &self.opcode)
            .field("flags", &self.flags)
            .field("priority", &self.priority)
            .field("fd", &self.fd)
            .field("offset", &self.offset)
            .field("addr", &self.addr)
            .field("len", &self.len)
            .field("user_data", &self.user_data)
            .finish()
    }
}
