#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LEDs. On the stm32f429i-disco they are connected to pin PG13 and PG14.
        let gpiog = dp.GPIOG.split();
        let mut led3 = gpiog.pg13.into_push_pull_output();
        let mut led4 = gpiog.pg14.into_push_pull_output();

        // Set up the system clock. We want to run at 180MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            // On for 1s, off for 1s.
            led3.set_high().unwrap();
            led4.set_low().unwrap();
            delay.delay_ms(1000_u32);
            
            led3.set_low().unwrap();
            led4.set_high().unwrap();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}