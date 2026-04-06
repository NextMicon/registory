use core::ptr::{read_volatile, write_volatile};

pub struct RegWrite {
    base: *mut u32,
}

impl RegWrite {
    pub const fn new(base: usize) -> Self {
        Self { base: base as *mut u32 }
    }

    pub fn write(&self, val: u32) {
        unsafe { write_volatile(self.base, val) }
    }

    pub fn read(&self) -> u32 {
        unsafe { read_volatile(self.base) }
    }
}

unsafe impl Send for RegWrite {}
unsafe impl Sync for RegWrite {}
