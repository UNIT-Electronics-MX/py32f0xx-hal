#![no_main]
#![no_std]

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    system_init::SystemInit,
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Inicialización del sistema
    let system = SystemInit::init_24mhz();
    
    // SOLO PROBAR PA0 como LED - SIN SERIAL
    let mut led = system.gpioa.pa0.into_push_pull_output();
    led.set_low();
    
    // Test inicial: 10 blinks rápidos para verificar que PA0 funciona
    for _ in 0..10 {
        led.set_high();
        cortex_m::asm::delay(2_400_000); // ~100ms a 24MHz
        led.set_low();
        cortex_m::asm::delay(2_400_000); // ~100ms a 24MHz
    }
    
    // Pausa larga
    cortex_m::asm::delay(12_000_000); // ~500ms
    
    // Loop principal: blink cada 500ms
    loop {
        led.set_high();
        cortex_m::asm::delay(12_000_000); // ~500ms
        
        led.set_low();
        cortex_m::asm::delay(12_000_000); // ~500ms
    }
}
