#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::pac;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // Habilitar clock para GPIOA
    p.RCC.iopenr.modify(|_, w| w.gpioaen().set_bit());
    
    // Configurar PA1 como salida
    p.GPIOA.moder.modify(|_, w| w.mode1().output());
    
    // APAGAR PA1 (ponerlo en LOW/0V)
    p.GPIOA.bsrr.write(|w| w.br1().set_bit());

    // Bucle infinito simple
    loop {
        cortex_m::asm::nop();
    }
}
