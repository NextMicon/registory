//! I2C Slave peripheral driver for NextMicon
use core::ptr::{read_volatile, write_volatile};

pub struct I2CSlave {
    base: *mut u32,
}

impl I2CSlave {
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

    /// Check if this slave has been addressed by a master
    pub fn addressed(&self) -> bool {
        unsafe { (read_volatile(self.base) >> 8) & 1 != 0 }
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

unsafe impl Send for I2CSlave {}
unsafe impl Sync for I2CSlave {}
