use std::io::{self, Error};
use std::mem;
use std::os::unix::io::RawFd;
use std::ptr::NonNull;
use sys_call::sys_call;

mod offsets;
use offsets::{CompletionOffsets, SubmissionOffsets};

mod queue;
use queue::Queue;

mod completion;
pub use completion::{CompletionQueue, CompletionQueueEntry};

mod submission;
pub use submission::{SubmissionQueue, SubmissionQueueEntry};

#[repr(C)]
#[derive(Debug)]
struct Params {
    sq_entries: u32,
    cq_entries: u32,
    flags: u32,
    sq_thread_cpu: u32,
    sq_thread_idle: u32,
    resv: [u32; 5],
    sq_off: SubmissionOffsets,
    cq_off: CompletionOffsets,
}

pub struct IoUring {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: u32,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
}

impl Default for IoUring {
    fn default() -> Self {
        Self {
            sq_entries: 0,
            cq_entries: 0,
            flags: 0,
            sq_thread_cpu: 0,
            sq_thread_idle: 0,
        }
    }
}

impl IoUring {
    pub fn with_entries(sq_entries: u32) -> Self {
        Self {
            sq_entries,
            ..Default::default()
        }
    }
    pub fn setup(&self) -> io::Result<(SubmissionQueue, CompletionQueue)> {
        let mut params = Params {
            sq_entries: self.sq_entries,
            cq_entries: self.cq_entries,
            flags: self.flags,
            sq_thread_cpu: self.sq_thread_cpu,
            sq_thread_idle: self.sq_thread_idle,
            resv: [0; 5],
            sq_off: Default::default(),
            cq_off: Default::default(),
        };

        to_result(unsafe {
            const IO_URING_SETUP: isize = 425;
            sys_call!(
                IO_URING_SETUP,
                self.sq_entries as isize,
                &mut params as *mut _ as isize
            )
        })
        .and_then(|fd| {
            to_result(unsafe {
                const OFFSET_SQ_RING: isize = 0;
                mmap(
                    params.sq_off.array as isize
                        + (self.sq_entries as isize * mem::size_of::<u32>() as isize),
                    fd,
                    OFFSET_SQ_RING,
                )
            })
            .and_then(|sq_ptr| {
                to_result(unsafe {
                    const OFFSET_CQ_RING: isize = 0x8000000;
                    mmap(
                        params.cq_off.cqes as isize
                            + (self.cq_entries as isize
                                * mem::size_of::<CompletionQueueEntry>() as isize),
                        fd,
                        OFFSET_CQ_RING,
                    )
                })
                .and_then(|cq_ptr| {
                    to_result(unsafe {
                        const OFFSET_SQES: isize = 0x10000000;
                        mmap(
                            self.sq_entries as isize
                                * mem::size_of::<SubmissionQueueEntry>() as isize,
                            fd,
                            OFFSET_SQES,
                        )
                    })
                    .map(|sqes| {
                        let sqes = sqes as *mut SubmissionQueueEntry;
                        let cqes = (cq_ptr + params.cq_off.cqes as isize) as *mut _;
                        unsafe {
                            (
                                SubmissionQueue {
                                    queue: Queue {
                                        head: field(sq_ptr, params.sq_off.head),
                                        tail: field(sq_ptr, params.sq_off.tail),
                                        mask: field(sq_ptr, params.sq_off.mask),
                                        len: self.sq_entries,
                                        entries: NonNull::new_unchecked(sqes),
                                    },
                                    array: field(sq_ptr, params.sq_off.array),
                                },
                                CompletionQueue {
                                    fd: fd as RawFd,
                                    queue: Queue {
                                        head: field(cq_ptr, params.cq_off.head),
                                        tail: field(cq_ptr, params.cq_off.tail),
                                        mask: field(cq_ptr, params.cq_off.mask),
                                        len: self.cq_entries,
                                        entries: NonNull::new_unchecked(cqes),
                                    },
                                },
                            )
                        }
                    })
                })
            })
        })
    }
}

fn to_result(ret: isize) -> io::Result<isize> {
    if ret > 0 {
        Ok(ret)
    } else {
        Err(Error::from_raw_os_error(ret as i32 * -1))
    }
}

unsafe fn mmap(size: isize, fd: isize, offset: isize) -> isize {
    sys_call!(9, 0, size, 1 | 2, 0x0001 | 0x08000, fd, offset)
}

unsafe fn field(ptr: isize, field: u32) -> NonNull<u32> {
    NonNull::new_unchecked((ptr + field as isize) as *mut u32)
}
