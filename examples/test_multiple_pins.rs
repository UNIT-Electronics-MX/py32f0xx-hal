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
    
    // Configurar múltiples pines como salida para probar
    // PA0, PA1, PA2, PA3
    p.GPIOA.moder.modify(|_, w| 
        w.mode0().output()
         .mode1().output()
         .mode2().output()
         .mode3().output()
    );
    
    // PB0, PB1, PB4, PB5
    p.GPIOB.moder.modify(|_, w|
        w.mode0().output()
         .mode1().output()
         .mode4().output()
         .mode5().output()
    );

    let mut counter = 0u32;
    
    loop {
        if counter < 1_000_000 {
            // ENCENDER todos los pines
            p.GPIOA.bsrr.write(|w| w.bs0().set_bit().bs1().set_bit().bs2().set_bit().bs3().set_bit());
            p.GPIOB.bsrr.write(|w| w.bs0().set_bit().bs1().set_bit().bs4().set_bit().bs5().set_bit());
        } else if counter < 2_000_000 {
            // APAGAR todos los pines
            p.GPIOA.bsrr.write(|w| w.br0().set_bit().br1().set_bit().br2().set_bit().br3().set_bit());
            p.GPIOB.bsrr.write(|w| w.br0().set_bit().br1().set_bit().br4().set_bit().br5().set_bit());
        } else {
            counter = 0;
            continue;
        }
        
        counter += 1;
        cortex_m::asm::nop();
    }
}
