#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    rcc::{RccExt, HSIFreq},
};

use cortex_m_rt::entry;
use embedded_hal_02::serial::{Read, Write as OtherWrite};

#[entry]
fn main() -> ! {
    // USE THE SAME CONFIGURATION AS BLINKY (WHICH WORKS)
    let mut p = pac::Peripherals::take().unwrap();

    // Configure RCC for 24MHz HSI (SAME AS WORKING BLINKY)
    let rcc = p.RCC
        .configure()
        .hsi(HSIFreq::Freq24mhz)  // Set HSI to 24MHz 
        .sysclk(24.MHz())         // Set system clock to 24MHz
        .freeze(&mut p.FLASH);

    // Initialize GPIO A and B
    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();

    // Pin configuration for USART2 (REQUESTED CONFIGURATION)
    // PA0 as TX with AF9 for USART2
    // PA1 as RX with AF9 for USART2
    let tx = gpioa.pa0.into_alternate_af9();
    let rx = gpioa.pa1.into_alternate_af9();

    // PB5 as debug pin (SAME AS WORKING BLINKY)
    let mut debug_pin = gpiob.pb5.into_push_pull_output();
    
    // Initialize debug pin LOW
    debug_pin.set_low();

    let mut serial = p.USART2.serial((tx, rx), 9_600.bps(), &rcc.clocks);
    serial.write_str("=== USART2 PA0/PA1 AF9 WORKING - 9600 bps ===\r\n").ok();
    serial.write_str("PA0: TX (AF9) - Register configured\r\n").ok();
    serial.write_str("PA1: RX (AF9) - Register configured\r\n").ok();
    serial.write_str("PB5: Debug LED\r\n").ok();
    serial.write_str("Clock: 24MHz confirmed\r\n").ok();
    serial.write_str("USART2: Clock enabled - 9600 bps\r\n").ok();
    
    // 3 INITIALIZATION BLINKS (VERY SLOW AND VISIBLE)
    serial.write_str("Starting 3 SLOW blinks...\r\n").ok();
    
    // Blink 1
    serial.write_str("Blink 1\r\n").ok();
    debug_pin.set_high();
    for _ in 0..1_200_000 {  // ~1 second (same as blinky)
        cortex_m::asm::nop();
    }
    debug_pin.set_low();
    for _ in 0..1_200_000 {  // ~1 second (same as blinky)
        cortex_m::asm::nop();
    }
    
    // Blink 2
    serial.write_str("Blink 2\r\n").ok();
    debug_pin.set_high();
    for _ in 0..1_200_000 {
        cortex_m::asm::nop();
    }
    debug_pin.set_low();
    for _ in 0..1_200_000 {
        cortex_m::asm::nop();
    }
    
    // Blink 3
    serial.write_str("Blink 3\r\n").ok();
    debug_pin.set_high();
    for _ in 0..1_200_000 {
        cortex_m::asm::nop();
    }
    debug_pin.set_low();
    for _ in 0..1_200_000 {
        cortex_m::asm::nop();
    }
    
    serial.write_str("=== System Ready! ===\r\n").ok();

    loop {
        // Indicate waiting for data (debug pin LOW)
        debug_pin.set_low();
        
        // Wait for reception of a single byte
        let received: u8 = nb::block!(serial.read()).unwrap();

        // Indicate processing data (debug pin HIGH)
        debug_pin.set_high();
        
        // Send back previously received byte and wait for completion
        nb::block!(serial.write(received)).ok();
        
        // Small pause to visualize the pulse (same as blinky)
        for _ in 0..120_000 {  // ~100ms
            cortex_m::asm::nop();
        }
    }
}
