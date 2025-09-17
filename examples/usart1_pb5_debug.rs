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
    // Inicialización del sistema
    let system = SystemInit::init_24mhz();
    
    // Configurar PB5 como LED de debug
    let mut led = system.gpiob.pb5.into_push_pull_output();
    led.set_low();

    // 3 blinks de inicialización
    for _ in 0..3 {
        led.set_high();
        cortex_m::asm::delay(4_800_000); // ~200ms a 24MHz
        led.set_low();
        cortex_m::asm::delay(4_800_000);
    }

    // Pausa
    cortex_m::asm::delay(12_000_000);

    // Usar USART1 en lugar de USART2 con los MISMOS pines
    // PA1 y PA2 también funcionan con USART1
    let rx = system.gpioa.pa1.into_alternate_af1(); // USART1_RX en PA1
    let tx = system.gpioa.pa2.into_alternate_af1(); // USART1_TX en PA2
    
    // Obtener periféricos sin conflictos
    let p = pac::Peripherals::take().unwrap();
    
    // Parpadeo para indicar que obtuvimos periféricos
    led.set_high();
    cortex_m::asm::delay(9_600_000); // 400ms
    led.set_low();
    cortex_m::asm::delay(2_400_000);
    
    // Configurar USART1 (más estable que USART2)
    let mut serial = p.USART1.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // 5 parpadeos para indicar USART1 configurado
    for _ in 0..5 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low(); 
        cortex_m::asm::delay(1_200_000);
    }
    
    // Pausa
    cortex_m::asm::delay(2_400_000);
    
    // Mensajes iniciales
    serial.write_str("=== USART1 con PB5 Debug ===\r\n").ok();
    serial.write_str("PA1: RX (AF1 - USART1)\r\n").ok();
    serial.write_str("PA2: TX (AF1 - USART1)\r\n").ok();
    serial.write_str("PB5: Debug LED\r\n").ok();
    serial.write_str("Test: Escribe algo...\r\n").ok();
    serial.write_str("============================\r\n").ok();
    
    loop {
        // LED apagado = esperando datos
        led.set_low();
        
        // Leer byte
        let received: u8 = nb::block!(serial.read()).unwrap();
        
        // LED encendido = procesando
        led.set_high();
        
        // Echo del byte
        nb::block!(serial.write(received)).ok();
        
        // Nueva línea si es Enter
        if received == b'\r' {
            nb::block!(serial.write(b'\n')).ok();
        }
        
        // Pequeña pausa para ver el LED
        cortex_m::asm::delay(240_000); // ~10ms
    }
}
