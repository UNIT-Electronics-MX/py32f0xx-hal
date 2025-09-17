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
    
    // Probar diferentes pines GPIO disponibles en DFN8
    // Nota: Solo incluimos los que NO usamos para USART2 (PA1, PA2)
    let mut pa0 = system.gpioa.pa0.into_push_pull_output();  // Pin 8 en DFN8
    // PA1 y PA2 los usamos para USART2
    let mut pb0 = system.gpiob.pb0.into_push_pull_output();  // Pin que dijiste que tienes
    let mut pb5 = system.gpiob.pb5.into_push_pull_output();  // Pin que mencionaste
    
    loop {
        // Parpadear PA0 (1 vez)
        pa0.set_high();
        cortex_m::asm::delay(2_400_000); // ~100ms
        pa0.set_low();
        cortex_m::asm::delay(2_400_000);
        
        // Pausa
        cortex_m::asm::delay(4_800_000); // ~200ms
        
        // Parpadear PB0 (2 veces)
        for _ in 0..2 {
            pb0.set_high();
            cortex_m::asm::delay(2_400_000);
            pb0.set_low();
            cortex_m::asm::delay(2_400_000);
        }
        
        // Pausa
        cortex_m::asm::delay(4_800_000);
        
        // Parpadear PB5 (3 veces)
        for _ in 0..3 {
            pb5.set_high();
            cortex_m::asm::delay(2_400_000);
            pb5.set_low();
            cortex_m::asm::delay(2_400_000);
        }
        
        // Pausa larga antes de repetir
        cortex_m::asm::delay(24_000_000); // ~1 segundo
    }
}
