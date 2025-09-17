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
    
    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();

    // LED de debug en PA0
    let mut led = system.gpioa.pa0.into_push_pull_output();
    led.set_low();

    // 3 blinks de inicialización
    for _ in 0..3 {
        led.set_high();
        cortex_m::asm::delay(4_800_000); // ~200ms a 24MHz
        led.set_low();
        cortex_m::asm::delay(4_800_000);
    }

    // IMPORTANTE: Usar USART1 con PA1/PA2 
    // Esto es más probable que funcione en PY32F003x4
    
    // PA1 como RX (AF1 para USART1)
    // PA2 como TX (AF1 para USART1) 
    let rx = system.gpioa.pa1.into_alternate_af1();
    let tx = system.gpioa.pa2.into_alternate_af1();
    
    let mut serial = p.USART1.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // Señal de que el serial está configurado (5 blinks)
    for _ in 0..5 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low(); 
        cortex_m::asm::delay(1_200_000);
    }
    
    // Pausa antes de empezar
    cortex_m::asm::delay(2_400_000);
    
    serial.write_str("=== PY32F0xx Serial Test ===\r\n").ok();
    serial.write_str("USART1 configurado:\r\n").ok(); 
    serial.write_str("PA1: RX (AF1)\r\n").ok();
    serial.write_str("PA2: TX (AF1)\r\n").ok();
    serial.write_str("Velocidad: 115200 bps\r\n").ok();
    serial.write_str("Reloj: 24MHz\r\n").ok();
    serial.write_str("========================\r\n").ok();
    serial.write_str("Escriba algo y se repetira...\r\n").ok();
    
    loop {
        // LED apagado = esperando datos
        led.set_low();
        
        // Esperar y leer un byte
        let received: u8 = nb::block!(serial.read()).unwrap();
        
        // LED encendido = procesando
        led.set_high();
        
        // Enviar de vuelta el byte
        nb::block!(serial.write(received)).ok();
        
        // Si recibimos Enter, enviar nueva línea
        if received == b'\r' {
            nb::block!(serial.write(b'\n')).ok();
        }
        
        // Pequeña pausa para ver el LED
        cortex_m::asm::delay(240_000); // ~10ms
    }
}
