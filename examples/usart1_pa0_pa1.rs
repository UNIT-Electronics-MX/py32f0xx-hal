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
    cortex_m::asm::delay(12_000_000); // ~500ms

    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();

    // Señal de que obtuvimos periféricos (1 blink largo)
    led.set_high();
    cortex_m::asm::delay(9_600_000); // 400ms
    led.set_low();
    cortex_m::asm::delay(2_400_000);

    // Configurar USART1 con PA0=TX(AF9) y PA1=RX(AF9)
    let tx = system.gpioa.pa0.into_alternate_af9();  // PA0 TX AF9
    let rx = system.gpioa.pa1.into_alternate_af9();  // PA1 RX AF9
    
    // Señal de que configuramos GPIO (2 blinks)
    for _ in 0..2 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low();
        cortex_m::asm::delay(1_200_000);
    }
    cortex_m::asm::delay(2_400_000);
    
    // Configurar USART1 (más estable que USART2)
    let mut serial = p.USART1.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // Señal de que USART1 está configurado (5 blinks rápidos)
    for _ in 0..5 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low(); 
        cortex_m::asm::delay(1_200_000);
    }
    
    // Pausa antes de empezar
    cortex_m::asm::delay(2_400_000);
    
    // Mensajes informativos
    serial.write_str("=== USART1 con PB5 Debug ===\r\n").ok();
    serial.write_str("PA0: TX (AF9)\r\n").ok();
    serial.write_str("PA1: RX (AF9)\r\n").ok();
    serial.write_str("PB5: Debug LED\r\n").ok();
    serial.write_str("Test: Escribe algo...\r\n").ok();
    serial.write_str("============================\r\n").ok();
    
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
