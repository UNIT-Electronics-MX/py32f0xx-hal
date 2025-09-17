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
use embedded_hal_02::serial::{Read, Write as OtherWrite};

#[entry]
fn main() -> ! {
    // Inicialización del sistema usando nuestro módulo común
    let system = SystemInit::init_24mhz(); // 24MHz para comunicación serial estable
    
    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();

    // Configuración de pines para USART2 (CONFIGURACIÓN SOLICITADA)
    // PA1 como RX con AF9 para USART2
    // PA2 como TX con AF4 para USART2
    let rx = system.gpioa.pa1.into_alternate_af9();
    let tx = system.gpioa.pa2.into_alternate_af4();

    // PB5 como pin de debug (salida push-pull)
    let mut debug_pin = system.gpiob.pb5.into_push_pull_output();
    
    // Inicializar debug pin en LOW
    debug_pin.set_low();

    let mut serial = p.USART2.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    serial.write_str("=== USART2 Configurado ===\r\n").ok();
    serial.write_str("PA1: RX (AF9)\r\n").ok();
    serial.write_str("PA2: TX (AF4)\r\n").ok();
    serial.write_str("Debug/LED pin: PB5\r\n").ok();
    
    // 3 BLINKS DE INICIALIZACIÓN (más lentos y visibles)
    serial.write_str("Iniciando 3 blinks...\r\n").ok();
    
    // Blink 1
    serial.write_str("Blink 1\r\n").ok();
    debug_pin.set_high();
    cortex_m::asm::delay(2_400_000); // ~100ms a 24MHz - LED encendido
    debug_pin.set_low();
    cortex_m::asm::delay(2_400_000); // ~100ms a 24MHz - LED apagado
    
    // Blink 2
    serial.write_str("Blink 2\r\n").ok();
    debug_pin.set_high();
    cortex_m::asm::delay(2_400_000);
    debug_pin.set_low();
    cortex_m::asm::delay(2_400_000);
    
    // Blink 3
    serial.write_str("Blink 3\r\n").ok();
    debug_pin.set_high();
    cortex_m::asm::delay(2_400_000);
    debug_pin.set_low();
    cortex_m::asm::delay(2_400_000);
    
    serial.write_str("=== Sistema Listo! ===\r\n").ok();

    loop {
        // Indicar que estamos esperando datos (debug pin LOW)
        debug_pin.set_low();
        
        // Wait for reception of a single byte
        let received: u8 = nb::block!(serial.read()).unwrap();

        // Indicar que estamos procesando datos (debug pin HIGH)
        debug_pin.set_high();
        
        // Send back previously received byte and wait for completion
        nb::block!(serial.write(received)).ok();
        
        // Pequeña pausa para visualizar el pulso
        cortex_m::asm::delay(240_000); // ~10ms a 24MHz
    }
}
