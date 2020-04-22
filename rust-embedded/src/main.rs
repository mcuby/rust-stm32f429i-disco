#![no_main]
#![no_std]
extern crate cortex_m;
extern crate cortex_m_rt as runtime;
extern crate stm32f7;

use core::panic::PanicInfo;
use cortex_m::asm;

#[no_mangle]
fn main() -> ! {    
    loop {
        for _i in 0..100000 {
             asm::nop()
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
