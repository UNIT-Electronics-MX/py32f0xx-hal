#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::pac;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    
    // Configurar el reloj del sistema step by step
    
    // 1. Habilitar HSI (High Speed Internal) 8MHz
    p.RCC.cr.modify(|_, w| w.hsion().set_bit());
    
    // 2. Esperar a que HSI esté listo
    while p.RCC.cr.read().hsirdy().bit_is_clear() {}
    
    // 3. Configurar HSI como fuente de reloj del sistema
    p.RCC.cfgr.modify(|_, w| unsafe { w.sw().bits(0) }); // 0 = HSI
    
    // 4. Esperar a que el switch sea efectivo
    while p.RCC.cfgr.read().sws().bits() != 0 {}
    
    // 5. Habilitar el reloj para GPIOA
    p.RCC.iopenr.modify(|_, w| w.gpioaen().set_bit());
    
    // 6. Esperar un poco para que el reloj se establezca
    for _ in 0..1000 {
        cortex_m::asm::nop();
    }
    
    // 7. Configurar PA1 como salida push-pull
    p.GPIOA.moder.modify(|_, w| w.mode1().output());
    p.GPIOA.otyper.modify(|_, w| w.ot1().push_pull());
    p.GPIOA.ospeedr.modify(|_, w| w.ospeed1().low_speed());
    p.GPIOA.pupdr.modify(|_, w| w.pupd1().floating());
    
    // 8. Test: alternar PA1 entre HIGH y LOW
    loop {
        // PA1 = HIGH (3.3V)
        p.GPIOA.bsrr.write(|w| w.bs1().set_bit());
        
        // Demora larga (~1 segundo)
        for _ in 0..400000 {
            cortex_m::asm::nop();
        }
        
        // PA1 = LOW (0V)  
        p.GPIOA.bsrr.write(|w| w.br1().set_bit());
        
        // Demora larga (~1 segundo)
        for _ in 0..400000 {
            cortex_m::asm::nop();
        }
    }
}
