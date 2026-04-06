use core::ptr::read_volatile;

pub struct RegRead {
    base: *mut u32,
}

impl RegRead {
    pub const fn new(base: usize) -> Self {
        Self { base: base as *mut u32 }
    }

    pub fn read(&self) -> u32 {
        unsafe { read_volatile(self.base) }
    }
}

unsafe impl Send for RegRead {}
unsafe impl Sync for RegRead {}
