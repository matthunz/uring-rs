use std::ptr::NonNull;

mod entry;
pub use self::entry::SubmissionQueueEntry;

// TODO MUNMAP
pub struct SubmissionQueue {
    pub entries: u32,
    pub head: NonNull<u32>,
    pub tail: NonNull<u32>,
    pub mask: NonNull<u32>,
    pub array: NonNull<u32>,
    pub sqes: NonNull<SubmissionQueueEntry>,
}

impl SubmissionQueue {
    pub fn next_sqe(&mut self) -> Option<&mut SubmissionQueueEntry> {
        let tail = unsafe { self.tail.as_mut() };
        let head = unsafe { self.head.as_ref() };
        let next = *tail + 1;

        if next - head <= self.entries {
            let sqe = unsafe { self.next_sqe_unchecked() };
            Some(sqe)
        } else {
            None
        }
    }
    pub unsafe fn next_sqe_unchecked(&mut self) -> &mut SubmissionQueueEntry {
        let index = self.tail.as_ref() & self.mask.as_ref();
        *self.array.as_ptr().offset(index as isize) = index;
        *self.tail.as_mut() += 1;

        &mut *self.sqes.as_ptr().offset(index as isize)
    }
}
