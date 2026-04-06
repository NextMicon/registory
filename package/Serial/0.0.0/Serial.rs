//! Serial peripheral driver for NextMicon
use core::ptr::{read_volatile, write_volatile};

const REG_BAUD: usize = 0;
const REG_IO: usize = 1;

pub struct Serial {
    base: *mut u32,
}

impl Serial {
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

    /// Set baudrate.
    pub fn baud(&self, baudrate: u32) {
        self.write_reg(REG_BAUD, super::CLK / baudrate);
    }

    /// Print a single character.
    pub fn print_char(&self, c: u8) -> &Self {
        self.write_reg(REG_IO, c as u32);
        self
    }

    /// Print a string (byte slice).
    pub fn print(&self, s: &[u8]) -> &Self {
        for &c in s {
            self.write_reg(REG_IO, c as u32);
        }
        self
    }

    /// Print a string (str).
    pub fn print_str(&self, s: &str) -> &Self {
        self.print(s.as_bytes())
    }

    /// Print an integer in hexadecimal with the given number of hex digits.
    pub fn hex(&self, num: u32, digits: u32) -> &Self {
        let hex_chars = b"0123456789ABCDEF";
        let mut i = (4 * digits) as i32 - 4;
        while i >= 0 {
            let nibble = ((num >> i as u32) & 0xF) as usize;
            self.write_reg(REG_IO, hex_chars[nibble] as u32);
            i -= 4;
        }
        self
    }

    /// Print an integer in decimal.
    pub fn dec(&self, mut num: u32) -> &Self {
        let mut buffer = [0u8; 10];
        let mut pos = 0usize;
        loop {
            unsafe { *buffer.as_mut_ptr().add(pos) = (num % 10) as u8 };
            pos += 1;
            num /= 10;
            if num == 0 {
                break;
            }
        }
        while pos > 0 {
            pos -= 1;
            let digit = unsafe { *buffer.as_ptr().add(pos) };
            self.write_reg(REG_IO, (b'0' + digit) as u32);
        }
        self
    }

    /// Receive a byte synchronously (blocks until data is available).
    pub fn receive(&self) -> u32 {
        loop {
            let received = self.read_reg(REG_IO) as i32;
            if received != -1 {
                return received as u32;
            }
        }
    }

    /// Receive a byte with timeout in microseconds.
    /// Returns the received value, or `u32::MAX` (0xFFFFFFFF) on timeout.
    pub fn receive_timeout(&self, timeout_us: u32) -> u32 {
        let start = super::rdcycle() / 4;
        loop {
            if super::rdcycle() / 4 > timeout_us.wrapping_add(start) {
                return u32::MAX;
            }
            let received = self.read_reg(REG_IO) as i32;
            if received != -1 {
                return received as u32;
            }
        }
    }

    /// Receive an unsigned integer from serial (reads decimal digits until a non-digit is received).
    pub fn receive_int(&self) -> u32 {
        let mut ret: u32 = 0;
        loop {
            let rcv = self.receive() as u8;
            if rcv >= b'0' && rcv <= b'9' {
                ret = ret * 10 + (rcv - b'0') as u32;
            } else {
                break;
            }
        }
        ret
    }
}

unsafe impl Send for Serial {}
unsafe impl Sync for Serial {}
