#![no_main]
#![no_std]

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    system_init::SystemInit,
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Inicialización del sistema usando nuestro módulo común
    let system = SystemInit::init_24mhz(); // 24MHz para comunicación serial estable
    
    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();

    // PB5 como pin de debug (salida push-pull) - disponible en DFN8
    let mut debug_pin = system.gpiob.pb5.into_push_pull_output();
    
    // TEST BÁSICO: Solo parpadear PB5 para confirmar que el sistema funciona
    loop {
        // LED encendido por 1 segundo
        debug_pin.set_high();
        cortex_m::asm::delay(24_000_000); // ~1 segundo a 24MHz
        
        // LED apagado por 1 segundo  
        debug_pin.set_low();
        cortex_m::asm::delay(24_000_000); // ~1 segundo a 24MHz
    }
}
