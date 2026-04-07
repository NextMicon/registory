use core::ptr::{read_volatile, write_volatile};

pub struct RegWrite {
    base: *mut u32,
}

impl RegWrite {
    pub const fn new(base: u32) -> Self {
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

pub struct RegRead {
    base: *mut u32,
}

impl RegRead {
    pub const fn new(base: u32) -> Self {
        Self { base: base as *mut u32 }
    }

    pub fn read(&self) -> u32 {
        unsafe { read_volatile(self.base) }
    }
}

unsafe impl Send for RegRead {}
unsafe impl Sync for RegRead {}

pub struct RegTrig {
    base: *mut u32,
}

impl RegTrig {
    pub const fn new(base: u32) -> Self {
        Self { base: base as *mut u32 }
    }

    pub fn trigger(&self) {
        unsafe { write_volatile(self.base, 1) }
    }
}

unsafe impl Send for RegTrig {}
unsafe impl Sync for RegTrig {}
