#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::system_init::SystemInit;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Inicialización automática del sistema con configuración por defecto
    let sys = SystemInit::init();

    // Configurar PB5 como salida - cambiar fácilmente el pin aquí
    let mut led = sys.gpiob.pb5.into_push_pull_output();

    // Bucle principal - alternar LED
    loop {
        // Encender LED
        led.set_high();
        
        // Demora (~1 segundo)
        for _ in 0..400000 {
            cortex_m::asm::nop();
        }
        
        // Apagar LED
        led.set_low();

        // Demora (~1 segundo)
        for _ in 0..400000 {
            cortex_m::asm::nop();
        }
    }
}
