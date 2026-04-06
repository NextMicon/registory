//! SPI DAC peripheral driver for NextMicon (e.g. MCP4921)
use core::ptr::write_volatile;

const BUFF: u16 = 0b0100_0000_0000_0000;
const GAIN: u16 = 0b0010_0000_0000_0000;
const SHDN: u16 = 0b0001_0000_0000_0000;

pub struct SPIDAC {
    base: *mut u32,
}

impl SPIDAC {
    pub const fn new(base: usize) -> Self {
        Self {
            base: base as *mut u32,
        }
    }

    #[inline(always)]
    fn write_reg(&self, offset: usize, val: u32) {
        unsafe { write_volatile(self.base.add(offset), val) }
    }

    /// Set analog output value.
    ///
    /// - `value`: analog output value 0..=65535 (upper 12/10/8 bits used)
    /// - `buf`: VREF input buffer control (true = buffered, false = unbuffered)
    /// - `ga`: output gain selection (true = 1x, false = 2x)
    /// - `shdn`: output shutdown control (true = active, false = shutdown)
    pub fn analog(&self, value: u16, buf: bool, ga: bool, shdn: bool) {
        let cmd: u16 = (if buf { BUFF } else { 0 })
            | (if ga { GAIN } else { 0 })
            | (if shdn { SHDN } else { 0 })
            | ((value >> 4) & 0xFFF);
        self.write_reg(0, cmd as u32);
    }

    /// Set analog output value with default settings (buffered, 1x gain, active).
    pub fn analog_default(&self, value: u16) {
        self.analog(value, true, true, true);
    }
}

unsafe impl Send for SPIDAC {}
unsafe impl Sync for SPIDAC {}
