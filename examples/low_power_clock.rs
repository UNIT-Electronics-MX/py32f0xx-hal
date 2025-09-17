#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::system_init::SystemInit;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // ========= CONFIGURACIÓN PARA BAJO CONSUMO =========
    // Usar 4MHz para minimizar el consumo de energía
    let sys = SystemInit::init_4mhz();
    
    // Configurar LED
    let mut led = sys.gpiob.pb5.into_push_pull_output();
    
    // Delays más lentos para 4MHz (consume menos energía)
    let delay_4mhz = 100_000;
    
    loop {
        // Encender LED por tiempo corto
        led.set_high();
        for _ in 0..delay_4mhz {
            cortex_m::asm::nop();
        }
        
        // Apagar LED por tiempo largo (ahorra energía)
        led.set_low();
        for _ in 0..(delay_4mhz * 5) {  // 5x más tiempo apagado
            cortex_m::asm::nop();
        }
        
        // Nota: Para bajo consumo real, también puedes usar:
        // - Sleep modes (WFI/WFE)
        // - Desactivar periféricos no utilizados
        // - Configurar pines no utilizados como analog input
    }
}
