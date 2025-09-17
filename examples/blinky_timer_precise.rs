#![no_main]
#![no_std]

use panic_halt as _;

use py32f0xx_hal as hal;
use crate::hal::{pac, prelude::*, rcc::{RccExt, HSIFreq}, timer::TimerExt};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    // Configure RCC for 24MHz HSI (equivalent to your C code)
    let rcc = p.RCC
        .configure()
        .hsi(HSIFreq::Freq24mhz)  // Set HSI to 24MHz like in your C code
        .sysclk(24.MHz())         // Set system clock to 24MHz
        .freeze(&mut p.FLASH);

    // Initialize GPIO A
    let gpioa = p.GPIOA.split();

    // Configure PA1 as output
    let mut led = gpioa.pa1.into_push_pull_output();

    // Create a timer for precise delays
    let mut timer = p.TIM1.timer(1.Hz(), &rcc.clocks);

    loop {
        // Turn LED on
        led.set_high();
        // Wait exactly 1 second using timer
        timer.start(1.Hz());
        nb::block!(timer.wait()).unwrap();
        
        // Turn LED off  
        led.set_low();
        // Wait exactly 1 second using timer
        timer.start(1.Hz());
        nb::block!(timer.wait()).unwrap();
    }
}
