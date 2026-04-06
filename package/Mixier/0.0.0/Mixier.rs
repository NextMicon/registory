//! Mixier (audio mixer) peripheral driver for NextMicon
use core::ptr::write_volatile;

pub struct Mixier {
    base: *mut u32,
}

impl Mixier {
    pub const fn new(base: usize) -> Self {
        Self {
            base: base as *mut u32,
        }
    }

    #[inline(always)]
    fn write_reg(&self, offset: usize, val: u32) {
        unsafe { write_volatile(self.base.add(offset), val) }
    }

    /// Set volume for a channel.
    ///
    /// - `ch`: channel index 0..4
    /// - `vol`: volume 0..=15
    pub fn set_vol(&self, ch: u32, vol: u32) {
        if ch < 4 {
            self.write_reg(ch as usize, vol);
        }
    }
}

unsafe impl Send for Mixier {}
unsafe impl Sync for Mixier {}
