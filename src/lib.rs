use std::io::{self, Error};
use std::mem;
use std::ptr::NonNull;
use sys_call::sys_call;

pub mod params;
pub use params::Params;

mod submission_queue;
pub use submission_queue::{SubmissionQueue, SubmissionQueueEntry};

pub fn setup(mut params: Params) -> io::Result<SubmissionQueue> {
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
        .and_then(|ptr| {
            to_result(unsafe {
                const OFFSET_SQES: isize = 0x10000000;
                mmap(
                    params.sq_entries as isize * mem::size_of::<SubmissionQueueEntry>() as isize,
                    fd,
                    OFFSET_SQES,
                )
            })
            .map(|sqes| {
                let sqes = sqes as *mut SubmissionQueueEntry;
                unsafe {
                    SubmissionQueue {
                        entries: params.sq_entries,
                        head: field(ptr, params.sq_off.head),
                        tail: field(ptr, params.sq_off.tail),
                        mask: field(ptr, params.sq_off.ring_mask),
                        array: field(ptr, params.sq_off.array),
                        sqes: NonNull::new_unchecked(sqes),
                    }
                }
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
