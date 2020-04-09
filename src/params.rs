#[repr(C)]
#[derive(Debug)]
pub struct SQRingOffsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub flags: u32,
    pub dropped: u32,
    pub array: u32,
    pub resv1: u32,
    pub resv2: u64,
}

impl Default for SQRingOffsets {
    fn default() -> Self {
        Self {
            head: 0,
            tail: 0,
            ring_mask: 0,
            ring_entries: 0,
            flags: 0,
            dropped: 0,
            array: 0,
            resv1: 0,
            resv2: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CQRingOffsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub overflow: u32,
    pub cqes: u32,
    pub resv: [u64; 2],
}

impl Default for CQRingOffsets {
    fn default() -> Self {
        Self {
            head: 0,
            tail: 0,
            ring_mask: 0,
            ring_entries: 0,
            overflow: 0,
            cqes: 0,
            resv: [0; 2],
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Params {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: u32,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
    pub resv: [u32; 5],
    pub sq_off: SQRingOffsets,
    pub cq_off: CQRingOffsets,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            sq_entries: 0,
            cq_entries: 0,
            flags: 0,
            sq_thread_cpu: 0,
            sq_thread_idle: 0,
            resv: [0; 5],
            sq_off: SQRingOffsets::default(),
            cq_off: CQRingOffsets::default(),
        }
    }
}
