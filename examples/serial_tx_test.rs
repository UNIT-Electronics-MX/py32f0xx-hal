#![no_main]
#![no_std]

use core::fmt::Write;
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
    // Inicialización del sistema
    let system = SystemInit::init_24mhz();
    
    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();

    // LED de debug en PA0
    let mut led = system.gpioa.pa0.into_push_pull_output();
    led.set_low();

    // 3 blinks de inicialización MUY LENTOS para debug
    for i in 1..=3 {
        led.set_high();
        cortex_m::asm::delay(12_000_000); // ~500ms a 24MHz
        led.set_low();
        cortex_m::asm::delay(12_000_000);
    }

    // Configuración USART1 con PA1(RX)/PA2(TX) AF1 para DFN8
    let rx = system.gpioa.pa1.into_alternate_af1();
    let tx = system.gpioa.pa2.into_alternate_af1();
    
    let mut serial = p.USART1.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // Señal de que el serial está listo (10 blinks muy rápidos)
    for _ in 0..10 {
        led.set_high();
        cortex_m::asm::delay(600_000); // ~25ms
        led.set_low(); 
        cortex_m::asm::delay(600_000);
    }
    
    // Pausa antes de empezar comunicación
    cortex_m::asm::delay(24_000_000); // 1 segundo
    
    let mut counter = 0u32;
    
    loop {
        // LED ON = enviando mensaje
        led.set_high();
        
        // Enviar mensaje cada ciclo
        serial.write_str("\r\n=== PY32F003I DFN8 Serial Test ===\r\n").ok();
        serial.write_str("Package: DFN8 (8 pins)\r\n").ok();
        serial.write_str("MCU: PY32F003x4\r\n").ok();
        serial.write_str("Config: USART1 PA1(RX)/PA2(TX) AF1\r\n").ok();
        serial.write_str("Clock: 24MHz\r\n").ok();
        serial.write_str("Baud: 115200\r\n").ok();
        
        // Contador para verificar que está funcionando
        counter += 1;
        write!(serial, "Counter: {}\r\n", counter).ok();
        serial.write_str("Status: TX Working!\r\n").ok();
        serial.write_str("============================\r\n\r\n").ok();
        
        led.set_low();
        
        // Esperar 2 segundos antes del siguiente mensaje
        cortex_m::asm::delay(48_000_000); // ~2s a 24MHz
    }
}
