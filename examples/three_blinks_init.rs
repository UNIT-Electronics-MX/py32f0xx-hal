#![no_main]
#![no_std]

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    system_init::SystemInit,
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Inicialización del sistema
    let system = SystemInit::init_24mhz();
    
    // Configurar PA0 como LED de estado
    let mut led = system.gpioa.pa0.into_push_pull_output();
    
    // Inicializar LED apagado
    led.set_low();
    
    // Pequeña pausa inicial
    cortex_m::asm::delay(12_000_000); // ~500ms a 24MHz
    
    // 3 BLINKS DE INICIALIZACIÓN
    for _ in 0..3 {
        // LED encendido
        led.set_high();
        cortex_m::asm::delay(4_800_000); // ~200ms a 24MHz
        
        // LED apagado  
        led.set_low();
        cortex_m::asm::delay(4_800_000); // ~200ms a 24MHz
    }
    
    // Pausa más larga para indicar fin de inicialización
    cortex_m::asm::delay(12_000_000); // ~500ms
    
    // Loop principal - blink lento continuo para mostrar que está funcionando
    loop {
        // Blink lento (1 segundo encendido, 1 segundo apagado)
        led.set_high();
        cortex_m::asm::delay(24_000_000); // ~1s a 24MHz
        
        led.set_low();
        cortex_m::asm::delay(24_000_000); // ~1s a 24MHz
    }
}
