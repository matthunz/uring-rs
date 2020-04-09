use crate::{to_result, Queue};
use std::io;
use std::os::unix::io::RawFd;
use sys_call::sys_call;

#[derive(Debug)]
pub struct CompletionQueueEntry {
    pub user_data: u64,
    pub res: i32,
    pub flags: u32,
}
#[derive(Debug)]
pub struct CompletionQueue {
    pub(super) fd: RawFd,
    pub(super) queue: Queue<CompletionQueueEntry>,
}

impl CompletionQueue {
    pub fn wait_for_cqe(&self) -> io::Result<isize> {
        self.wait_for_cqes(1, 1)
    }
    pub fn wait_for_cqes(&self, to_submit: isize, count: isize) -> io::Result<isize> {
        to_result(unsafe {
            const IO_URING_ENTER: isize = 426;
            sys_call!(IO_URING_ENTER, self.fd as isize, to_submit, count, 1, 0)
        })
    }
    pub fn next_cqe(&mut self) -> Option<&CompletionQueueEntry> {
        if unsafe { self.queue.head.as_ref() != self.queue.tail.as_ref() } {
            Some(unsafe { self.next_cqe_unchecked() })
        } else {
            None
        }
    }
    pub unsafe fn next_cqe_unchecked(&mut self) -> &CompletionQueueEntry {
        let index = self.queue.head.as_ref() & self.queue.mask.as_ref();
        *self.queue.head.as_mut() += 1;

        &*self.queue.entries.as_ptr().offset(index as isize)
    }
}
