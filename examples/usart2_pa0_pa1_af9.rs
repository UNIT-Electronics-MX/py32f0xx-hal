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
    
    // PRIMERO: Confirmar que PB5 funciona como debug
    let mut debug_pin = system.gpiob.pb5.into_push_pull_output();
    debug_pin.set_low();
    
    // 3 BLINKS para confirmar que arranca
    for _ in 0..3 {
        debug_pin.set_high();
        cortex_m::asm::delay(4_800_000); // ~200ms
        debug_pin.set_low();
        cortex_m::asm::delay(4_800_000);
    }
    
    // Pausa
    cortex_m::asm::delay(12_000_000); // ~500ms
    
    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();

    // 1 blink largo para confirmar periféricos OK
    debug_pin.set_high();
    cortex_m::asm::delay(9_600_000); // ~400ms
    debug_pin.set_low();
    cortex_m::asm::delay(2_400_000);

    // Configurar USART2 con PA0=TX(AF9) y PA1=RX(AF9)
    let tx = system.gpioa.pa0.into_alternate_af9();  // PA0 TX AF9
    let rx = system.gpioa.pa1.into_alternate_af9();  // PA1 RX AF9
    
    // 2 blinks para confirmar GPIO configurado
    for _ in 0..2 {
        debug_pin.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        debug_pin.set_low();
        cortex_m::asm::delay(1_200_000);
    }
    cortex_m::asm::delay(2_400_000);

    // Configurar USART2
    let mut serial = p.USART2.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // 5 blinks rápidos para confirmar USART2 configurado
    for _ in 0..5 {
        debug_pin.set_high();
        cortex_m::asm::delay(600_000); // 25ms
        debug_pin.set_low();
        cortex_m::asm::delay(600_000);
    }
    
    cortex_m::asm::delay(2_400_000);
    
    // ENVÍO CONTINUO de mensajes para debug
    let mut counter = 0u32;
    
    loop {
        counter += 1;
        
        // Enviar mensaje informativo cada ciclo
        serial.write_str("=== USART2 PA0/PA1 AF9 ===\r\n").ok();
        serial.write_str("PA0: TX (AF9)\r\n").ok();
        serial.write_str("PA1: RX (AF9)\r\n").ok();
        serial.write_str("PB5: Debug LED\r\n").ok();
        
        // Contador para ver que sigue funcionando
        serial.write_str("Count: ").ok();
        
        // Convertir counter a string simple
        let mut num_str = [0u8; 10];
        let mut temp = counter;
        let mut pos = 0;
        
        if temp == 0 {
            num_str[0] = b'0';
            pos = 1;
        } else {
            while temp > 0 {
                num_str[pos] = (temp % 10) as u8 + b'0';
                temp /= 10;
                pos += 1;
            }
            // Reverse the digits
            for i in 0..pos/2 {
                let temp = num_str[i];
                num_str[i] = num_str[pos-1-i];
                num_str[pos-1-i] = temp;
            }
        }
        
        for i in 0..pos {
            nb::block!(serial.write(num_str[i])).ok();
        }
        
        serial.write_str("\r\n").ok();
        serial.write_str("Status: TX funcionando\r\n").ok();
        serial.write_str("=======================\r\n").ok();
        
        // LED parpadea con cada mensaje
        debug_pin.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        debug_pin.set_low();
        
        // Pausa de 1 segundo entre mensajes
        cortex_m::asm::delay(24_000_000); // ~1 segundo
    }
}
