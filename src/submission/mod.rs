use crate::Queue;
use std::ptr::NonNull;

mod entry;
pub use self::entry::SubmissionQueueEntry;

// TODO MUNMAP
pub struct SubmissionQueue {
    pub queue: Queue<SubmissionQueueEntry>,
    pub array: NonNull<u32>,
}

impl SubmissionQueue {
    pub fn next_sqe(&mut self) -> Option<&mut SubmissionQueueEntry> {
        let tail = unsafe { self.queue.tail.as_mut() };
        let head = unsafe { self.queue.head.as_ref() };
        let next = *tail + 1;

        if next - head <= self.queue.len {
            let sqe = unsafe { self.next_sqe_unchecked() };
            Some(sqe)
        } else {
            None
        }
    }
    pub unsafe fn next_sqe_unchecked(&mut self) -> &mut SubmissionQueueEntry {
        let index = self.index() as isize;
        *self.queue.tail.as_mut() += 1;

        &mut *self.queue.entries.as_ptr().offset(index)
    }
    pub fn submit_sqe(&mut self) {
        let index = self.index();
        unsafe {
            *self.array.as_ptr().offset(index as isize) = index;
        }
    }
    pub fn index(&self) -> u32 {
        unsafe { self.queue.tail.as_ref() & self.queue.mask.as_ref() }
    }
}
