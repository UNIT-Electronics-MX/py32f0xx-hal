#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    rcc::{RccExt, HSIFreq},
    adc::{Adc, AdcClockMode},
};

use cortex_m_rt::entry;
use embedded_hal_02::serial::{Read, Write as OtherWrite};
use embedded_hal_02::adc::OneShot;

#[entry]
fn main() -> ! {
    // USE THE SAME CONFIGURATION AS SERIAL_ECHO (WHICH WORKS)
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

    // Pin configuration for USART2 (WORKING CONFIGURATION)
    // PA0 as TX with AF9 for USART2
    // PA1 as RX with AF9 for USART2
    let tx = gpioa.pa0.into_alternate_af9();
    let rx = gpioa.pa1.into_alternate_af9();

    // PA2 as ADC analog input (available on DFN8)
    let mut adc_pin = gpioa.pa2.into_analog();

    // PB5 as debug pin (SAME AS WORKING BLINKY)
    let mut debug_pin = gpiob.pb5.into_push_pull_output();
    
    // Initialize debug pin LOW
    debug_pin.set_low();

    // Configure ADC
    let mut adc = Adc::new(p.ADC, AdcClockMode::default());

    // Configure USART2
    let mut serial = p.USART2.serial((tx, rx), 9_600.bps(), &rcc.clocks);
    
    // Startup messages
    serial.write_str("=== SERIAL ADC - PY32F003I DFN8 ===\r\n").ok();
    serial.write_str("USART2: PA0=TX(AF9), PA1=RX(AF9) @ 9600 bps\r\n").ok();
    serial.write_str("ADC: PA2 as analog input\r\n").ok();
    serial.write_str("Debug: PB5 LED\r\n").ok();
    serial.write_str("Clock: 24MHz\r\n").ok();
    serial.write_str("Commands:\r\n").ok();
    serial.write_str("  'r' = read ADC once\r\n").ok();
    serial.write_str("  's' = streaming mode (send ADC every 500ms)\r\n").ok();
    serial.write_str("  'q' = stop streaming\r\n").ok();
    serial.write_str("  'h' = help\r\n").ok();
    
    // 3 INITIALIZATION BLINKS
    serial.write_str("Starting system...\r\n").ok();
    
    for i in 1..=3 {
        serial.write_str("Blink ").ok();
        serial.write_str(match i {
            1 => "1",
            2 => "2", 
            3 => "3",
            _ => "?",
        }).ok();
        serial.write_str("\r\n").ok();
        
        debug_pin.set_high();
        for _ in 0..600_000 {  // ~0.5 seconds
            cortex_m::asm::nop();
        }
        debug_pin.set_low();
        for _ in 0..600_000 {  // ~0.5 seconds
            cortex_m::asm::nop();
        }
    }
    
    serial.write_str("=== System Ready! ===\r\n").ok();
    serial.write_str("Type 'h' for help\r\n").ok();

    let mut streaming = false;
    let mut stream_counter = 0u32;

    loop {
        // Indicate waiting for data (debug pin LOW)
        debug_pin.set_low();

        // Check if serial data is available (non-blocking)
        match serial.read() {
            Ok(received) => {
                // Indicate processing data (debug pin HIGH)
                debug_pin.set_high();
                
                // Process commands
                match received {
                    b'r' | b'R' => {
                        // Read ADC once
                        serial.write_str("Reading ADC... ").ok();
                        let adc_value: u16 = adc.read(&mut adc_pin).unwrap_or(0);
                        let voltage_mv = (adc_value as u32 * 3300) / 4096; // Convert to mV (assuming Vref=3.3V)
                        
                        serial.write_str("ADC=").ok();
                        write_u16(&mut serial, adc_value);
                        serial.write_str(" (").ok();
                        write_u32(&mut serial, voltage_mv);
                        serial.write_str("mV)\r\n").ok();
                    },
                    b's' | b'S' => {
                        // Start streaming
                        streaming = true;
                        stream_counter = 0;
                        serial.write_str("ADC streaming started (500ms intervals)\r\n").ok();
                    },
                    b'q' | b'Q' => {
                        // Stop streaming
                        streaming = false;
                        serial.write_str("ADC streaming stopped\r\n").ok();
                    },
                    b'h' | b'H' => {
                        // Help
                        serial.write_str("\r\n=== HELP ===\r\n").ok();
                        serial.write_str("r = Read ADC once\r\n").ok();
                        serial.write_str("s = Streaming every 500ms\r\n").ok();
                        serial.write_str("q = Stop streaming\r\n").ok();
                        serial.write_str("h = This help\r\n").ok();
                        serial.write_str("ADC: PA2 (0-3.3V -> 0-4095)\r\n").ok();
                        serial.write_str("=============\r\n").ok();
                    },
                    _ => {
                        // Echo received character + new line
                        nb::block!(serial.write(received)).ok();
                        serial.write_str(" (press 'h' for help)\r\n").ok();
                    }
                }
                
                // Small pause to visualize the pulse
                for _ in 0..120_000 {  // ~100ms
                    cortex_m::asm::nop();
                }
            },
            Err(nb::Error::WouldBlock) => {
                // No data available, continue
            },
            Err(_) => {
                // Communication error
                debug_pin.set_high();
                for _ in 0..60_000 {  // ~50ms error blink
                    cortex_m::asm::nop();
                }
            }
        }

        // ADC streaming if enabled
        if streaming {
            stream_counter += 1;
            if stream_counter >= 60_000 {  // ~500ms @ 24MHz with loop overhead
                stream_counter = 0;
                
                let adc_value: u16 = adc.read(&mut adc_pin).unwrap_or(0);
                let voltage_mv = (adc_value as u32 * 3300) / 4096;
                
                serial.write_str("ADC: ").ok();
                write_u16(&mut serial, adc_value);
                serial.write_str(" (").ok();
                write_u32(&mut serial, voltage_mv);
                serial.write_str("mV)\r\n").ok();
                
                // Pulse LED to indicate reading
                debug_pin.set_high();
                for _ in 0..24_000 {  // ~20ms pulse
                    cortex_m::asm::nop();
                }
                debug_pin.set_low();
            }
        }
    }
}

// Helper function to write u16 as string
fn write_u16<T: core::fmt::Write>(serial: &mut T, value: u16) {
    let mut buffer = [0u8; 6]; // Maximum 5 digits + null
    let mut pos = 0;
    let mut val = value;
    
    if val == 0 {
        serial.write_str("0").ok();
        return;
    }
    
    while val > 0 && pos < 5 {
        buffer[pos] = (val % 10) as u8 + b'0';
        val /= 10;
        pos += 1;
    }
    
    // Write in reverse order
    while pos > 0 {
        pos -= 1;
        serial.write_char(buffer[pos] as char).ok();
    }
}

// Helper function to write u32 as string
fn write_u32<T: core::fmt::Write>(serial: &mut T, value: u32) {
    let mut buffer = [0u8; 11]; // Maximum 10 digits + null
    let mut pos = 0;
    let mut val = value;
    
    if val == 0 {
        serial.write_str("0").ok();
        return;
    }
    
    while val > 0 && pos < 10 {
        buffer[pos] = (val % 10) as u8 + b'0';
        val /= 10;
        pos += 1;
    }
    
    // Write in reverse order
    while pos > 0 {
        pos -= 1;
        serial.write_char(buffer[pos] as char).ok();
    }
}
