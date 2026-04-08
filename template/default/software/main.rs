#![no_std]
#![no_main]

mod micon;
use micon::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut on: u32 = 0;
    loop {
        on ^= 1;
        LED_REG.write(on);

        // ~500ms delay
        let scale = CLK / 1_000_000;
        let start = rdcycle() / scale;
        while (rdcycle() / scale).wrapping_sub(start) < 500_000 {}
    }
}

#[no_mangle]
pub extern "C" fn irq(_regs: *mut u32, _irqs: u32) {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
