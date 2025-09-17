#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::system_init::SystemInit;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // ========= CONFIGURACIÓN DE FRECUENCIA DE RELOJ =========
    // ¡Solo descomenta la línea de la frecuencia que quieres usar!
    
    // let sys = SystemInit::init_4mhz();     // 4MHz  - Bajo consumo
    // let sys = SystemInit::init();             // 8MHz  - Por defecto  
    // let sys = SystemInit::init_16mhz();    // 16MHz - Rendimiento medio
    let sys = SystemInit::init_24mhz();    // 24MHz - Máximo rendimiento
    
    // Configurar LED en PB5
    let mut led = sys.gpiob.pb5.into_push_pull_output();
    
    // ========= DELAYS CALIBRADOS PARA CADA FRECUENCIA =========
    // Los delays cambian según la frecuencia del reloj
    
    // Para 4MHz:   delay = 100,000  (más lento)
    // Para 8MHz:   delay = 200,000  (por defecto)
    // Para 16MHz:  delay = 400,000  (más rápido)  
    // Para 24MHz:  delay = 600,000  (máximo)
    
    let delay_cycles = 600_000; // Ajustado para 8MHz
    
    loop {
        // Encender LED
        led.set_high();
        
        // Delay calibrado para la frecuencia seleccionada
        for _ in 0..delay_cycles {
            cortex_m::asm::nop();
        }
        
        // Apagar LED
        led.set_low();
        
        // Delay calibrado para la frecuencia seleccionada
        for _ in 0..delay_cycles {
            cortex_m::asm::nop();
        }
    }
}
