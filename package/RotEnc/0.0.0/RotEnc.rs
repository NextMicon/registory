//! Rotary encoder peripheral driver for NextMicon
use core::ptr::read_volatile;

pub struct RotEnc {
    base: *mut u32,
}

impl RotEnc {
    pub const fn new(base: usize) -> Self {
        Self {
            base: base as *mut u32,
        }
    }

    #[inline(always)]
    fn read_reg(&self, offset: usize) -> u32 {
        unsafe { read_volatile(self.base.add(offset)) }
    }

    /// Get the current angle value.
    pub fn get_angle(&self) -> u32 {
        self.read_reg(0)
    }
}

unsafe impl Send for RotEnc {}
unsafe impl Sync for RotEnc {}
