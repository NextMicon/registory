//! Spectro peripheral driver for NextMicon (stub - no methods defined in C++ source)
use core::ptr::{read_volatile, write_volatile};

pub struct Spectro {
    base: *mut u32,
}

impl Spectro {
    pub const fn new(base: usize) -> Self {
        Self {
            base: base as *mut u32,
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    fn read_reg(&self, offset: usize) -> u32 {
        unsafe { read_volatile(self.base.add(offset)) }
    }

    #[inline(always)]
    #[allow(dead_code)]
    fn write_reg(&self, offset: usize, val: u32) {
        unsafe { write_volatile(self.base.add(offset), val) }
    }
}

unsafe impl Send for Spectro {}
unsafe impl Sync for Spectro {}
