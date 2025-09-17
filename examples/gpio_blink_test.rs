#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::pac;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // Habilitar el reloj del sistema a HSI (8MHz)
    p.RCC.cr.modify(|_, w| w.hsion().set_bit());
    while p.RCC.cr.read().hsirdy().bit_is_clear() {}
    
    // Habilitar clock para GPIOA
    p.RCC.iopenr.modify(|_, w| w.gpioaen().set_bit());
    
    // Configurar PA1 como salida
    p.GPIOA.moder.modify(|_, w| w.mode1().output());
    p.GPIOA.otyper.modify(|_, w| w.ot1().push_pull());

    loop {
        // Encender PA1 (HIGH/3.3V)
        p.GPIOA.bsrr.write(|w| w.bs1().set_bit());
        
        // Demora
        for _ in 0..100000 {
            cortex_m::asm::nop();
        }
        
        // Apagar PA1 (LOW/0V)
        p.GPIOA.bsrr.write(|w| w.br1().set_bit());
        
        // Demora
        for _ in 0..100000 {
            cortex_m::asm::nop();
        }
    }
}
