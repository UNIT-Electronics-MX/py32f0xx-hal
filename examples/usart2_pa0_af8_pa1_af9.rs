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
    
    // Configurar PB5 como debug
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

    // CONFIGURACIÓN CORRECTA según la tabla:
    // PA0: USART2_TX con AF9 (naranja en la tabla)
    // PA1: USART2_RX con AF9 (amarillo en la tabla)
    let tx = system.gpioa.pa0.into_alternate_af9();  // PA0 TX AF9 ✓
    let rx = system.gpioa.pa1.into_alternate_af9();  // PA1 RX AF9 ✓
    
    // 2 blinks para confirmar GPIO configurado
    for _ in 0..2 {
        debug_pin.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        debug_pin.set_low();
        cortex_m::asm::delay(1_200_000);
    }
    cortex_m::asm::delay(2_400_000);

    // Configurar USART2 con la configuración correcta
    let mut serial = p.USART2.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // 5 blinks rápidos para confirmar USART2 configurado
    for _ in 0..5 {
        debug_pin.set_high();
        cortex_m::asm::delay(600_000); // 25ms
        debug_pin.set_low();
        cortex_m::asm::delay(600_000);
    }
    
    cortex_m::asm::delay(2_400_000);
    
    // ENVÍO CONTINUO de mensajes
    let mut counter = 0u32;
    
    loop {
        counter += 1;
        
        // Enviar mensaje cada ciclo
        serial.write_str("=== USART2 CORRECTO ===\r\n").ok();
        serial.write_str("PA0: TX (AF9) <- CORRECTO\r\n").ok();
        serial.write_str("PA1: RX (AF9) <- CORRECTO\r\n").ok();
        serial.write_str("PB5: Debug LED\r\n").ok();
        serial.write_str("Config: Segun tabla oficial\r\n").ok();
        
        // Contador simple
        serial.write_str("Mensaje #: ").ok();
        let mut temp = counter;
        let mut digits = [0u8; 10];
        let mut pos = 0;
        
        if temp == 0 {
            nb::block!(serial.write(b'0')).ok();
        } else {
            while temp > 0 && pos < 10 {
                digits[pos] = (temp % 10) as u8 + b'0';
                temp /= 10;
                pos += 1;
            }
            for i in (0..pos).rev() {
                nb::block!(serial.write(digits[i])).ok();
            }
        }
        
        serial.write_str("\r\n").ok();
        serial.write_str("Status: TX FUNCIONANDO!\r\n").ok();
        serial.write_str("========================\r\n").ok();
        
        // LED parpadea con cada mensaje
        debug_pin.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        debug_pin.set_low();
        
        // Pausa de 1 segundo
        cortex_m::asm::delay(24_000_000);
    }
}
