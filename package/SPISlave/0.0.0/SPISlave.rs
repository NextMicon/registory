//! SPI Slave peripheral driver for NextMicon
use core::ptr::{read_volatile, write_volatile};

pub struct SPISlave {
    base: *mut u32,
}

impl SPISlave {
    pub const fn new(base: usize) -> Self {
        Self {
            base: base as *mut u32,
        }
    }

    /// Read received byte
    pub fn read(&self) -> u8 {
        unsafe { read_volatile(self.base) as u8 }
    }

    /// Set transmit byte
    pub fn write(&self, value: u8) {
        unsafe { write_volatile(self.base, value as u32) }
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

unsafe impl Send for SPISlave {}
unsafe impl Sync for SPISlave {}
