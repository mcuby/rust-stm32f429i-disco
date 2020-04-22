#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f4xx_hal as hal;

use crate::hal::{
    prelude::*,
    spi::Spi, 
};
use hal::spi::{Mode, Phase, Polarity};

use cortex_m_rt::{ExceptionFrame, entry, exception};
use embedded_graphics::{image::*,
    prelude::*};
use ssd1306::{prelude::*, Builder as SSD1306Builder};

use stm32f4::stm32f429;

#[entry]
fn main() -> ! {

    let dp = stm32f429::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpioe = dp.GPIOE.split();    

    //spi4
    //sck  - pe2
    //miso - pe5
    //mosi - pe6
    //cs - pe4
    //dc - pe3

    let sck = gpioe.pe2.into_alternate_af5();
    let miso = gpioe.pe5.into_alternate_af5();
    let mosi = gpioe.pe6.into_alternate_af5();

    let spi = Spi::spi4(dp.SPI4, (sck, miso, mosi), Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    }, stm32f4xx_hal::time::KiloHertz(2000).into(),clocks);

    let dc = gpioe.pe3.into_push_pull_output();
    let mut cs = gpioe.pe4.into_push_pull_output();
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    cs.set_high().unwrap();
    delay.delay_ms(100_u32);
    cs.set_low().unwrap();

    let btn = gpioa.pa0.into_pull_down_input();

    // Set up the display
    let mut disp: GraphicsMode<_> = SSD1306Builder::new().connect_spi(spi, dc).into();
    disp.init().unwrap();
    disp.flush().unwrap();

    // Display the rustacean
    let im = Image1BPP::new(include_bytes!("./rust.raw"), 64, 64);
    // let im = Image1BPP::new(include_bytes!("./ssd1306-image.data"), 128, 64);
    
    disp.draw(im.into_iter());
    disp.flush().unwrap();

    // Set up state for the loop
    let mut orientation = DisplayRotation::Rotate0;
    let mut was_pressed = btn.is_low().unwrap();

    // This runs continuously, as fast as possible
    loop {
        // Check if the button has just been pressed.
        // Remember, active low.
        let is_pressed = btn.is_low().unwrap();
        if !was_pressed && is_pressed {
            // Since the button was pressed, flip the screen upside down
            orientation = get_next_rotation(orientation);
            disp.set_rotation(orientation).unwrap();
            // Now that we've flipped the screen, store the fact that the button is pressed.
            was_pressed = true;
        } else if !is_pressed {
            // If the button is released, confirm this so that next time it's pressed we'll
            // know it's time to flip the screen.
             was_pressed = false;
        }
    }
}

/// Helper function - what rotation flips the screen upside down from
/// the rotation we're in now?
fn get_next_rotation(rotation: DisplayRotation) -> DisplayRotation {
    return match rotation {
        DisplayRotation::Rotate0 => DisplayRotation::Rotate180,
        DisplayRotation::Rotate180 => DisplayRotation::Rotate0,

        // Default branch - if for some reason we end up in one of the portrait modes,
        // reset to 0 degrees landscape. On most SSD1306 displays, this means down is towards
        // the flat flex coming out of the display (and up is towards the breakout board pins).
        _ => DisplayRotation::Rotate0,
    };
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}