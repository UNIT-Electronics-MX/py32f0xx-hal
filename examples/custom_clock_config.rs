#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::system_init::{SystemInit, SystemClockConfig};
use crate::hal::rcc::HSIFreq;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // ========= CONFIGURACIÓN PERSONALIZADA DEL RELOJ =========
    
    // Método 1: Usar configuraciones predefinidas (más fácil)
    // let sys = SystemInit::init();          // 8MHz por defecto
    // let sys = SystemInit::init_24mhz();    // 24MHz máximo
    
    // Método 2: Configuración personalizada (más control)
    let custom_config = SystemClockConfig {
        hsi_freq: HSIFreq::Freq16mhz,  // Frecuencia del oscilador HSI
        sysclk_mhz: 16,               // Frecuencia del sistema en MHz
    };
    let sys = SystemInit::init_with_config(custom_config);
    
    // Método 3: Configuración manual completa (máximo control)
    // Si necesitas configuración muy específica, usar directamente el HAL:
    /*
    let mut p = pac::Peripherals::take().unwrap();
    let rcc = p.RCC
        .configure()
        .hsi(HSIFreq::Freq16mhz)      // Configurar HSI
        .sysclk(16.MHz())             // Configurar SYSCLK
        .pclk(8.MHz())                // Configurar PCLK (opcional)
        .hclk(16.MHz())               // Configurar HCLK (opcional)
        .freeze(&mut p.FLASH);        // Aplicar configuración
    */
    
    // Configurar LED
    let mut led = sys.gpiob.pb5.into_push_pull_output();
    
    // Delays ajustados para 16MHz
    let delay_16mhz = 400_000;
    
    loop {
        led.set_high();
        for _ in 0..delay_16mhz {
            cortex_m::asm::nop();
        }
        
        led.set_low();
        for _ in 0..delay_16mhz {
            cortex_m::asm::nop();
        }
    }
}
