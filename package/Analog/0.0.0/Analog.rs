//! Analog output (PWM DAC) peripheral driver for NextMicon
use core::ptr::write_volatile;

pub struct Analog {
    base: *mut u32,
}

impl Analog {
    pub const fn new(base: usize) -> Self {
        Self {
            base: base as *mut u32,
        }
    }

    #[inline(always)]
    fn write_reg(&self, offset: usize, val: u32) {
        unsafe { write_volatile(self.base.add(offset), val) }
    }

    /// Set duty ratio 0..=255 (output voltage = val * 3.3V / 256).
    pub fn duty(&self, val: u32) {
        self.write_reg(0, val);
    }
}

unsafe impl Send for Analog {}
unsafe impl Sync for Analog {}
