//! Digital GPIO peripheral driver for NextMicon
use core::ptr::{read_volatile, write_volatile};

const REG_IOSEL: usize = 0;
const REG_OUT: usize = 1;
const REG_IN: usize = 2;

/// GPIO pin mode.
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Mode {
    In = 0,
    Out = 1,
}

pub struct Digital {
    base: *mut u32,
}

impl Digital {
    pub const fn new(base: usize) -> Self {
        Self {
            base: base as *mut u32,
        }
    }

    #[inline(always)]
    fn read_reg(&self, offset: usize) -> u32 {
        unsafe { read_volatile(self.base.add(offset)) }
    }

    #[inline(always)]
    fn write_reg(&self, offset: usize, val: u32) {
        unsafe { write_volatile(self.base.add(offset), val) }
    }

    /// Select Input (`Mode::In`) or Output (`Mode::Out`).
    pub fn set_mode(&self, mode: Mode) {
        self.write_reg(REG_IOSEL, mode as u32);
    }

    /// Read input value (when mode is `Mode::In`).
    pub fn read(&self) -> u32 {
        self.read_reg(REG_IN)
    }

    /// Write output value (when mode is `Mode::Out`).
    pub fn write(&self, val: u32) {
        self.write_reg(REG_OUT, val);
    }
}

unsafe impl Send for Digital {}
unsafe impl Sync for Digital {}
