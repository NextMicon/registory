//! Counter peripheral driver for NextMicon
use core::ptr::write_volatile;

pub struct Counter {
    base: *mut u32,
}

impl Counter {
    pub const fn new(base: usize) -> Self {
        Self {
            base: base as *mut u32,
        }
    }

    #[inline(always)]
    fn write_reg(&self, offset: usize, val: u32) {
        unsafe { write_volatile(self.base.add(offset), val) }
    }

    /// Set counter to a raw clock count value.
    pub fn set(&self, cnt: u32) {
        self.write_reg(0, cnt);
    }

    /// Set counter by seconds.
    pub fn set_sec(&self, sec: u32) {
        self.write_reg(0, super::CLK_HZ * sec);
    }

    /// Set counter by milliseconds.
    pub fn set_ms(&self, ms: u32) {
        self.write_reg(0, super::CLK_KHZ * ms);
    }

    /// Set counter by microseconds.
    pub fn set_us(&self, us: u32) {
        self.write_reg(0, super::CLK_MHZ * us);
    }

    /// Set counter by frequency (Hz).
    pub fn set_hz(&self, f: u32) {
        self.write_reg(0, super::CLK_HZ / f);
    }
}

unsafe impl Send for Counter {}
unsafe impl Sync for Counter {}
