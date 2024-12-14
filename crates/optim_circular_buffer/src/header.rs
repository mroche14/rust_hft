use std::sync::atomic::AtomicUsize;
use std::mem::MaybeUninit;

#[repr(C)]
pub struct SharedHeader {
    pub config_len: usize,
    pub head: AtomicUsize,
    pub lock_mode: u8,
    // config JSON is next, then data
}

impl SharedHeader {
    pub fn new(config_len: usize, locked: bool) -> Self {
        SharedHeader {
            config_len,
            head: AtomicUsize::new(0),
            lock_mode: if locked { 1 } else { 0 },
        }
    }
}
