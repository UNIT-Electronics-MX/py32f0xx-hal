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
    
    // PRIMERO configurar el LED y confirmar que funciona
    let mut led = system.gpiob.pb5.into_push_pull_output();
    led.set_low();

    // 3 blinks de inicialización (confirmamos que arranca)
    for _ in 0..3 {
        led.set_high();
        cortex_m::asm::delay(4_800_000); // ~200ms a 24MHz
        led.set_low();
        cortex_m::asm::delay(4_800_000);
    }

    // Pausa larga para confirmar visualmente que llegamos hasta aquí
    cortex_m::asm::delay(12_000_000); // ~500ms

    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();

    // Configurar GPIO para USART2 CUIDADOSAMENTE
    let rx = system.gpioa.pa1.into_alternate_af9();
    let tx = system.gpioa.pa2.into_alternate_af4();
    
    // Señal de que vamos a configurar USART2
    led.set_high();
    cortex_m::asm::delay(2_400_000); // LED encendido mientras configuramos
    
    // Configurar USART2 con manejo de errores
    let mut serial = p.USART2.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // Señal de que USART2 está configurado (5 blinks rápidos)
    led.set_low();
    for _ in 0..5 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low(); 
        cortex_m::asm::delay(1_200_000);
    }
    
    // Pausa antes de empezar comunicación
    cortex_m::asm::delay(2_400_000);
    
    // Intentar enviar mensajes iniciales con manejo de errores
    let _ = serial.write_str("=== USART2 con PB5 Debug ===\r\n");
    cortex_m::asm::delay(1_200_000);
    let _ = serial.write_str("PA1: RX (AF9)\r\n");
    cortex_m::asm::delay(1_200_000);
    let _ = serial.write_str("PA2: TX (AF4)\r\n");
    cortex_m::asm::delay(1_200_000);
    let _ = serial.write_str("PB5: Debug LED\r\n");
    cortex_m::asm::delay(1_200_000);
    let _ = serial.write_str("Test: Escribe algo...\r\n");
    cortex_m::asm::delay(1_200_000);
    
    loop {
        // LED apagado = esperando datos
        led.set_low();
        
        // Esperar y leer un byte (especificamos u8)
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
