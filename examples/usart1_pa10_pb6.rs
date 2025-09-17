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
    let system = SystemInit::init_24mhz();
    
    // PRIMERO: Probar que PB5 funciona ANTES de tocar USART
    let mut debug_pin = system.gpiob.pb5.into_push_pull_output();
    debug_pin.set_low();
    
    // 3 BLINKS para confirmar que arranca
    for _ in 0..3 {
        debug_pin.set_high();
        cortex_m::asm::delay(4_800_000); // ~200ms
        debug_pin.set_low();
        cortex_m::asm::delay(4_800_000);
    }
    
    // Pausa larga
    cortex_m::asm::delay(12_000_000); // ~500ms
    
    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();

    // 1 blink largo para confirmar que obtuvimos periféricos
    debug_pin.set_high();
    cortex_m::asm::delay(9_600_000); // ~400ms
    debug_pin.set_low();
    cortex_m::asm::delay(2_400_000);

    // Configurar USART1 con tus pines específicos:
    // PA10 como RX con AF1 para USART1
    // PB6 como TX con AF0 para USART1
    let rx = system.gpioa.pa10.into_alternate_af1(); // USART1_RX en PA10 con AF1
    let tx = system.gpiob.pb6.into_alternate_af0();  // USART1_TX en PB6 con AF0

    // 2 blinks para confirmar GPIO configurado
    for _ in 0..2 {
        debug_pin.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        debug_pin.set_low();
        cortex_m::asm::delay(1_200_000);
    }
    cortex_m::asm::delay(2_400_000);

    // Configurar USART1
    let mut serial = p.USART1.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // 5 blinks rápidos para confirmar USART1 configurado
    for _ in 0..5 {
        debug_pin.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        debug_pin.set_low();
        cortex_m::asm::delay(1_200_000);
    }
    
    // Mensajes informativos
    serial.write_str("=== USART1 PA10/PB6 ===\r\n").ok();
    serial.write_str("PA10: RX (AF1)\r\n").ok();
    serial.write_str("PB6: TX (AF0)\r\n").ok();
    serial.write_str("PB5: Debug LED\r\n").ok();
    serial.write_str("Test: Escribe algo...\r\n").ok();
    serial.write_str("========================\r\n").ok();

    loop {
        // LED apagado = esperando datos
        debug_pin.set_low();
        
        // Wait for reception of a single byte
        let received: u8 = nb::block!(serial.read()).unwrap();

        // LED encendido = procesando datos
        debug_pin.set_high();
        
        // Send back previously received byte and wait for completion
        nb::block!(serial.write(received)).ok();
        
        // Si recibimos Enter, enviar nueva línea
        if received == b'\r' {
            nb::block!(serial.write(b'\n')).ok();
        }
        
        // Pequeña pausa para visualizar el pulso
        cortex_m::asm::delay(240_000); // ~10ms
    }
}
