#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::pac;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // === CONFIGURACIÓN BÁSICA PASO A PASO ===
    
    // 1. Verificar que el RCC funciona - habilitar clocks
    p.RCC.iopenr.write(|w| w.gpioaen().set_bit().gpioben().set_bit());
    
    // 2. Forzar configuración directa sin usar las APIs del HAL
    // Configurar PA1 como output, push-pull, speed low
    p.GPIOA.moder.write(|w| unsafe { w.bits(0x00000004) }); // PA1 = output (01), resto input
    p.GPIOA.otyper.write(|w| unsafe { w.bits(0x00000000) }); // Todos push-pull
    p.GPIOA.ospeedr.write(|w| unsafe { w.bits(0x00000000) }); // Todos low speed
    p.GPIOA.pupdr.write(|w| unsafe { w.bits(0x00000000) }); // Sin pull-up/down
    
    // 3. Configurar PB5 también
    p.GPIOB.moder.write(|w| unsafe { w.bits(0x00000400) }); // PB5 = output (01), resto input
    p.GPIOB.otyper.write(|w| unsafe { w.bits(0x00000000) }); // Todos push-pull
    p.GPIOB.ospeedr.write(|w| unsafe { w.bits(0x00000000) }); // Todos low speed
    p.GPIOB.pupdr.write(|w| unsafe { w.bits(0x00000000) }); // Sin pull-up/down
    
    // 4. Bucle muy simple de encendido/apagado
    let mut state = false;
    let mut delay_counter = 0u32;
    
    loop {
        // Alternar estado cada 4 millones de ciclos (muy lento)
        if delay_counter >= 4_000_000 {
            state = !state;
            delay_counter = 0;
            
            if state {
                // ENCENDER - escribir directamente al ODR
                p.GPIOA.odr.write(|w| unsafe { w.bits(0x00000002) }); // PA1 = 1
                p.GPIOB.odr.write(|w| unsafe { w.bits(0x00000020) }); // PB5 = 1
            } else {
                // APAGAR - escribir directamente al ODR
                p.GPIOA.odr.write(|w| unsafe { w.bits(0x00000000) }); // PA1 = 0
                p.GPIOB.odr.write(|w| unsafe { w.bits(0x00000000) }); // PB5 = 0
            }
        }
        
        delay_counter += 1;
        
        // NOP para evitar optimizaciones
        cortex_m::asm::nop();
    }
}
