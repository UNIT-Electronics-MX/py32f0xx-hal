#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::pac;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // Habilitar clocks para GPIO A y B
    p.RCC.iopenr.modify(|_, w| w.gpioaen().set_bit().gpioben().set_bit());
    
    // Configurar PA1 como salida push-pull
    p.GPIOA.moder.modify(|_, w| w.mode1().output());
    p.GPIOA.otyper.modify(|_, w| w.ot1().push_pull());
    
    // Configurar PB5 como salida push-pull
    p.GPIOB.moder.modify(|_, w| w.mode5().output());  
    p.GPIOB.otyper.modify(|_, w| w.ot5().push_pull());

    let mut counter = 0u32;
    
    loop {
        // Parpadeo muy lento usando contador
        if counter < 1_000_000 {
            // LED encendido - ambos pines
            p.GPIOA.bsrr.write(|w| w.bs1().set_bit()); // PA1 HIGH
            p.GPIOB.bsrr.write(|w| w.bs5().set_bit()); // PB5 HIGH
        } else if counter < 2_000_000 {
            // LED apagado - ambos pines
            p.GPIOA.bsrr.write(|w| w.br1().set_bit()); // PA1 LOW
            p.GPIOB.bsrr.write(|w| w.br5().set_bit()); // PB5 LOW
        } else {
            // Reset contador
            counter = 0;
            continue;
        }
        
        counter += 1;
        
        // Pequeño delay para no saturar
        for _ in 0..10 {
            cortex_m::asm::nop();
        }
    }
}
