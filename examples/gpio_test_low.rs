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
    
    // Pequeña demora para asegurar que el clock esté estable
    for _ in 0..1000 {
        cortex_m::asm::nop();
    }
    
    // Configurar PA1 como salida push-pull
    p.GPIOA.moder.modify(|_, w| w.mode1().output());
    p.GPIOA.otyper.modify(|_, w| w.ot1().push_pull());
    p.GPIOA.ospeedr.modify(|_, w| w.ospeed1().low_speed());
    p.GPIOA.pupdr.modify(|_, w| w.pupd1().floating());
    
    // Forzar PA1 a LOW usando ODR (Output Data Register)
    p.GPIOA.odr.modify(|_, w| w.od1().clear_bit());
    
    // También usar BSRR para asegurar que esté en LOW
    p.GPIOA.bsrr.write(|w| w.br1().set_bit());

    // Bucle infinito - mantener PA1 en LOW
    loop {
        // Forzar PA1 a LOW repetidamente
        p.GPIOA.odr.modify(|_, w| w.od1().clear_bit());
        
        // Pequeña demora
        for _ in 0..10000 {
            cortex_m::asm::nop();
        }
    }
}
