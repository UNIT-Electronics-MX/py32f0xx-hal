#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::system_init::SystemInit;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Inicialización automática del sistema
    let sys = SystemInit::init();
    
    // ============= EJEMPLOS DE DIFERENTES PUERTOS =============
    
    // Puerto A - Pines disponibles: PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, etc.
    let mut pa0 = sys.gpioa.pa0.into_push_pull_output();
    let mut pa1 = sys.gpioa.pa1.into_push_pull_output();
    let mut pa2 = sys.gpioa.pa2.into_push_pull_output();
    
    // Puerto B - Pines disponibles: PB0, PB1, PB2, PB3, PB4, PB5, PB6, PB7, etc.
    let mut pb0 = sys.gpiob.pb0.into_push_pull_output();
    let mut pb1 = sys.gpiob.pb1.into_push_pull_output();
    let mut pb5 = sys.gpiob.pb5.into_push_pull_output();
    
    // Puerto F - Pines disponibles: PF0, PF1, PF2, PF3, etc.
    let mut pf0 = sys.gpiof.pf0.into_push_pull_output();
    let mut pf1 = sys.gpiof.pf1.into_push_pull_output();
    
    let mut counter = 0u32;
    
    // Bucle principal - alternar diferentes pines
    loop {
        match counter % 8 {
            0 => {
                // PA0 ON, otros OFF
                pa0.set_high(); pa1.set_low(); pa2.set_low(); 
                pb0.set_low(); pb1.set_low(); pb5.set_low();
                pf0.set_low(); pf1.set_low();
            },
            1 => {
                // PA1 ON, otros OFF
                pa0.set_low(); pa1.set_high(); pa2.set_low(); 
                pb0.set_low(); pb1.set_low(); pb5.set_low();
                pf0.set_low(); pf1.set_low();
            },
            2 => {
                // PA2 ON, otros OFF
                pa0.set_low(); pa1.set_low(); pa2.set_high(); 
                pb0.set_low(); pb1.set_low(); pb5.set_low();
                pf0.set_low(); pf1.set_low();
            },
            3 => {
                // PB0 ON, otros OFF
                pa0.set_low(); pa1.set_low(); pa2.set_low(); 
                pb0.set_high(); pb1.set_low(); pb5.set_low();
                pf0.set_low(); pf1.set_low();
            },
            4 => {
                // PB1 ON, otros OFF
                pa0.set_low(); pa1.set_low(); pa2.set_low(); 
                pb0.set_low(); pb1.set_high(); pb5.set_low();
                pf0.set_low(); pf1.set_low();
            },
            5 => {
                // PB5 ON, otros OFF
                pa0.set_low(); pa1.set_low(); pa2.set_low(); 
                pb0.set_low(); pb1.set_low(); pb5.set_high();
                pf0.set_low(); pf1.set_low();
            },
            6 => {
                // PF0 ON, otros OFF
                pa0.set_low(); pa1.set_low(); pa2.set_low(); 
                pb0.set_low(); pb1.set_low(); pb5.set_low();
                pf0.set_high(); pf1.set_low();
            },
            7 => {
                // PF1 ON, otros OFF
                pa0.set_low(); pa1.set_low(); pa2.set_low(); 
                pb0.set_low(); pb1.set_low(); pb5.set_low();
                pf0.set_low(); pf1.set_high();
            },
            _ => {}
        }
        
        // Demora
        for _ in 0..200000 {
            cortex_m::asm::nop();
        }
        
        counter += 1;
    }
}
