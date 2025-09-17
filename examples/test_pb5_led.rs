#![no_main]
#![no_std]

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    prelude::*,
    system_init::SystemInit,
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Inicialización del sistema
    let system = SystemInit::init_24mhz();
    
    // Configurar PB5 como salida
    let mut led = system.gpiob.pb5.into_push_pull_output();
    
    // Parpadeo continuo simple
    loop {
        led.set_high();
        cortex_m::asm::delay(12_000_000); // ~500ms a 24MHz
        led.set_low();
        cortex_m::asm::delay(12_000_000); // ~500ms a 24MHz
    }
}
