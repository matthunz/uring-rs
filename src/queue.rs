use std::ptr::NonNull;

#[derive(Debug)]
pub struct Queue<E> {
    pub head: NonNull<u32>,
    pub tail: NonNull<u32>,
    pub mask: NonNull<u32>,
    pub len: u32,
    pub entries: NonNull<E>,
}
