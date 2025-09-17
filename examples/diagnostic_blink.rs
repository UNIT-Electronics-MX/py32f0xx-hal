#![no_main]
#![no_std]

use panic_halt as _;

use py32f0xx_hal as hal;
use crate::hal::{pac, prelude::*, rcc::{RccExt, HSIFreq}};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    // Configure RCC for 24MHz HSI (EXACTAMENTE IGUAL AL BLINKY ORIGINAL)
    let _rcc = p.RCC
        .configure()
        .hsi(HSIFreq::Freq24mhz)  // Set HSI to 24MHz like in your C code
        .sysclk(24.MHz())         // Set system clock to 24MHz
        .freeze(&mut p.FLASH);

    // Initialize GPIO B (EXACTAMENTE IGUAL AL BLINKY ORIGINAL)
    let gpiob = p.GPIOB.split();

    // Configure PB5 as output (EXACTAMENTE IGUAL AL BLINKY ORIGINAL)
    let mut led = gpiob.pb5.into_push_pull_output();

    // PATRÓN DE DIAGNÓSTICO:
    // 3 parpadeos rápidos = sistema iniciado
    // Luego parpadeo lento continuo = funcionando
    
    // 3 parpadeos rápidos de inicio
    for _ in 0..3 {
        led.set_high();
        for _ in 0..300_000 {  // ~250ms
            cortex_m::asm::nop();
        }
        led.set_low();
        for _ in 0..300_000 {  // ~250ms
            cortex_m::asm::nop();
        }
    }
    
    // Pausa larga
    for _ in 0..2_400_000 {  // ~2 segundos
        cortex_m::asm::nop();
    }

    // Loop principal: parpadeo lento
    loop {
        // Turn LED on
        led.set_high();
        // Wait 1 second
        for _ in 0..1_200_000 {  // ~1 segundo
            cortex_m::asm::nop();
        }
        
        // Turn LED off  
        led.set_low();
        // Wait 1 second
        for _ in 0..1_200_000 {  // ~1 segundo
            cortex_m::asm::nop();
        }
    }
}
