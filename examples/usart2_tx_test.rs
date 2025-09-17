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

    // 3 blinks de inicialización
    for _ in 0..3 {
        led.set_high();
        cortex_m::asm::delay(4_800_000); // ~200ms a 24MHz
        led.set_low();
        cortex_m::asm::delay(4_800_000);
    }

    // Configuración USART2 con PA1/PA2
    // PA1 como RX con AF9 para USART2
    // PA2 como TX con AF4 para USART2
    let rx = system.gpioa.pa1.into_alternate_af9();
    let tx = system.gpioa.pa2.into_alternate_af4();
    
    let mut serial = p.USART2.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // Señal de que el serial está configurado (5 blinks)
    for _ in 0..5 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low(); 
        cortex_m::asm::delay(1_200_000);
    }
    
    // Pausa antes de empezar
    cortex_m::asm::delay(2_400_000);
    
    // Mensajes informativos
    serial.write_str("=== USART2 TX Test ===\r\n").ok();
    serial.write_str("PA1: RX (AF9)\r\n").ok();
    serial.write_str("PA2: TX (AF4)\r\n").ok();
    serial.write_str("PA0: Debug LED\r\n").ok();
    serial.write_str("Velocidad: 115200 bps\r\n").ok();
    serial.write_str("Reloj: 24MHz\r\n").ok();
    serial.write_str("====================\r\n").ok();
    
    let mut counter = 0u32;
    
    loop {
        // LED encendido = transmitiendo
        led.set_high();
        
        // Mensaje con contador
        serial.write_str("USART2 funcionando - contador: ").ok();
        
        // Escribir el número (método simple)
        let mut num = counter;
        let mut digits = [0u8; 10];
        let mut idx = 0;
        
        if num == 0 {
            serial.write_str("0").ok();
        } else {
            // Convertir número a dígitos
            while num > 0 {
                digits[idx] = (num % 10) as u8 + b'0';
                num /= 10;
                idx += 1;
            }
            
            // Escribir dígitos en orden inverso
            for i in (0..idx).rev() {
                let digit_char = [digits[i]];
                serial.write_str(core::str::from_utf8(&digit_char).unwrap()).ok();
            }
        }
        
        serial.write_str("\r\n").ok();
        
        counter += 1;
        
        // LED apagado = pausa
        led.set_low();
        
        // Pausa de 1 segundo
        cortex_m::asm::delay(24_000_000); // ~1s a 24MHz
    }
}
