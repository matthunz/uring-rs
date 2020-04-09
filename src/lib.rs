use std::io::{self, Error};
use std::mem;
use std::os::unix::io::RawFd;
use std::ptr::NonNull;
use sys_call::sys_call;

pub mod params;
pub use params::Params;

mod completion_queue;
pub use completion_queue::{CompletionQueue, CompletionQueueEntry};

mod submission_queue;
pub use submission_queue::{SubmissionQueue, SubmissionQueueEntry};

pub fn setup(mut params: Params) -> io::Result<(SubmissionQueue, CompletionQueue)> {
    to_result(unsafe {
        const IO_URING_SETUP: isize = 425;
        sys_call!(
            IO_URING_SETUP,
            params.sq_entries as isize,
            &mut params as *mut _ as isize
        )
    })
    .and_then(|fd| {
        to_result(unsafe {
            const OFFSET_SQ_RING: isize = 0;
            mmap(
                params.sq_off.array as isize
                    + (params.sq_entries as isize * mem::size_of::<u32>() as isize),
                fd,
                OFFSET_SQ_RING,
            )
        })
        .and_then(|sq_ptr| {
            to_result(unsafe {
                const OFFSET_CQ_RING: isize = 0x8000000;
                mmap(
                    params.cq_off.cqes as isize
                        + (params.cq_entries as isize
                            * mem::size_of::<CompletionQueueEntry>() as isize),
                    fd,
                    OFFSET_CQ_RING,
                )
            })
            .and_then(|cq_ptr| {
                to_result(unsafe {
                    const OFFSET_SQES: isize = 0x10000000;
                    mmap(
                        params.sq_entries as isize
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
                                entries: params.sq_entries,
                                head: field(sq_ptr, params.sq_off.head),
                                tail: field(sq_ptr, params.sq_off.tail),
                                mask: field(sq_ptr, params.sq_off.ring_mask),
                                array: field(sq_ptr, params.sq_off.array),
                                sqes: NonNull::new_unchecked(sqes),
                            },
                            CompletionQueue {
                                fd: fd as RawFd,
                                entries: params.cq_entries,
                                head: field(cq_ptr, params.cq_off.head),
                                tail: field(cq_ptr, params.cq_off.tail),
                                mask: field(cq_ptr, params.cq_off.ring_mask),
                                cqes: NonNull::new_unchecked(cqes),
                            },
                        )
                    }
                })
            })
        })
    })
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
