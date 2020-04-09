#[repr(C)]
#[derive(Debug)]
pub struct SubmissionOffsets {
    pub head: u32,
    pub tail: u32,
    pub mask: u32,
    pub entries: u32,
    pub flags: u32,
    pub dropped: u32,
    pub array: u32,
    pub resv1: u32,
    pub resv2: u64,
}

impl Default for SubmissionOffsets {
    fn default() -> Self {
        Self {
            head: 0,
            tail: 0,
            mask: 0,
            entries: 0,
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
pub struct CompletionOffsets {
    pub head: u32,
    pub tail: u32,
    pub mask: u32,
    pub entries: u32,
    pub overflow: u32,
    pub cqes: u32,
    pub resv: [u64; 2],
}

impl Default for CompletionOffsets {
    fn default() -> Self {
        Self {
            head: 0,
            tail: 0,
            mask: 0,
            entries: 0,
            overflow: 0,
            cqes: 0,
            resv: [0; 2],
        }
    }
}
