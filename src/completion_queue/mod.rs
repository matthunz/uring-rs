use crate::to_result;
use std::io;
use std::os::unix::io::RawFd;
use std::ptr::NonNull;
use sys_call::sys_call;

#[derive(Debug)]
pub struct CompletionQueueEntry {
    pub user_data: u64,
    pub res: i32,
    pub flags: u32,
}

pub struct CompletionQueue {
    pub fd: RawFd,
    pub entries: u32,
    pub head: NonNull<u32>,
    pub tail: NonNull<u32>,
    pub mask: NonNull<u32>,
    pub cqes: NonNull<CompletionQueueEntry>,
}

impl CompletionQueue {
    pub fn wait_for_cqes(&self, submit: isize, count: isize) -> io::Result<isize> {
        to_result(unsafe {
            const IO_URING_ENTER: isize = 426;
            sys_call!(IO_URING_ENTER, self.fd as isize, submit, count, 1, 0)
        })
    }
    pub fn next_cqe(&self) -> Option<&CompletionQueueEntry> {
        if unsafe { self.head.as_ref() != self.tail.as_ref() } {
            Some(unsafe { self.next_cqe_unchecked() })
        } else {
            None
        }
    }
    pub unsafe fn next_cqe_unchecked(&self) -> &CompletionQueueEntry {
        let index = self.head.as_ref() & self.mask.as_ref();
        &*self.cqes.as_ptr().offset(index as isize)
    }
}
