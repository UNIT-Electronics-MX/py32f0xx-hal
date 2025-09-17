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
    
    // ========= INTERCAMBIAR PUERTOS ES MUY FÁCIL =========
    // ¡Solo descomenta la línea del pin que quieres usar!
    
    // let mut led = sys.gpioa.pa0.into_push_pull_output();  // Puerto A, Pin 0
    // let mut led = sys.gpioa.pa1.into_push_pull_output();  // Puerto A, Pin 1  
    // let mut led = sys.gpiob.pb1.into_push_pull_output();  // Puerto B, Pin 1
    let mut led = sys.gpiob.pb5.into_push_pull_output();     // Puerto B, Pin 5 (ACTUAL)
    // let mut led = sys.gpiof.pf0.into_push_pull_output();  // Puerto F, Pin 0
    // let mut led = sys.gpiof.pf1.into_push_pull_output();  // Puerto F, Pin 1
    
    // El resto del código es idéntico sin importar qué pin uses
    loop {
        led.set_high();
        
        // Demora (~0.5 segundo)
        for _ in 0..200000 {
            cortex_m::asm::nop();
        }
        
        led.set_low();
        
        // Demora (~0.5 segundo)
        for _ in 0..200000 {
            cortex_m::asm::nop();
        }
    }
}
            cortex_m::asm::nop();
        }
    }
}
