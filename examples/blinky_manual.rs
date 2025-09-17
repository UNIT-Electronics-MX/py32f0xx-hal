#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // Configuración manual del reloj HSI
    // Habilitar HSI y esperar a que esté listo
    p.RCC.cr.modify(|_, w| w.hsion().set_bit());
    while p.RCC.cr.read().hsirdy().bit_is_clear() {}

    // Configurar HSI a 24MHz
    p.RCC.icscr.modify(|_, w| w.hsi_fs().freq24mhz());

    // Configurar el divisor del sistema
    p.RCC.cfgr.modify(|_, w| w.sw().hsi());
    
    // Esperar a que el reloj del sistema cambie a HSI
    while !p.RCC.cfgr.read().sws().is_hsi() {}

    // Habilitar clock para GPIOA y GPIOB
    p.RCC.iopenr.modify(|_, w| w.iopaen().set_bit().iopben().set_bit());
    
    // Configurar PA1 como salida
    p.GPIOA.moder.modify(|_, w| w.moder1().output());
    p.GPIOA.otyper.modify(|_, w| w.ot1().push_pull());
    
    // Configurar PB5 como salida también
    p.GPIOB.moder.modify(|_, w| w.moder5().output());
    p.GPIOB.otyper.modify(|_, w| w.ot5().push_pull());

    let mut counter = 0u32;
    
    loop {
        // Alternar ambos LEDs
        if counter & 0x200000 != 0 {  // Bit 21 como divisor (~2.4M ciclos)
            // LED encendido
            p.GPIOA.bsrr.write(|w| w.bs1().set_bit()); // PA1 HIGH
            p.GPIOB.bsrr.write(|w| w.bs5().set_bit()); // PB5 HIGH
        } else {
            // LED apagado
            p.GPIOA.bsrr.write(|w| w.br1().set_bit()); // PA1 LOW
            p.GPIOB.bsrr.write(|w| w.br5().set_bit()); // PB5 LOW
        }
        
        counter = counter.wrapping_add(1);
        
        // Pequeño delay
        cortex_m::asm::nop();
    }
}
