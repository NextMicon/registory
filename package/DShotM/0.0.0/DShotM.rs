//! DShot motor protocol peripheral driver for NextMicon
use core::ptr::{read_volatile, write_volatile};

/// DShot control request.
pub struct DshotReq {
    /// Throttle value: 0..=2000
    pub throttle: u16,
    /// Telemetry request flag
    pub telem: bool,
}

/// DShot telemetry response.
pub struct DshotRes {
    /// Rotation value
    pub rot: u16,
    /// CRC check passed
    pub crc_ok: bool,
}

pub struct DShotM {
    base: *mut u32,
}

impl DShotM {
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

    /// Send a DShot control request.
    pub fn send_ctrl(&self, req: &DshotReq) {
        let telem_bit: u32 = if req.telem { 0x10 } else { 0 };
        let val = ((req.throttle as u32 + 47) << 5) + telem_bit;
        self.write_reg(0, val);
    }

    /// Get telemetry response.
    pub fn get_telem(&self) -> DshotRes {
        DshotRes {
            rot: (self.read_reg(1) >> 4) as u16,
            crc_ok: self.read_reg(2) != 0,
        }
    }
}

unsafe impl Send for DShotM {}
unsafe impl Sync for DShotM {}
