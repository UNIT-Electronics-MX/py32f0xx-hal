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
    let system = SystemInit::init_16mhz(); // 16MHz para comunicación serial estable
    
    // Configuración de pines para USART1
    // PA1 como RX (AF1)
    // PA2 como TX (AF1)
    let rx = system.gpioa.pa1.into_alternate_af1();
    let tx = system.gpioa.pa2.into_alternate_af1();
    
    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();
    
    // Configurar USART1 con velocidad de 115200 bps
    let mut serial = p.USART1.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // Mensaje inicial
    serial.write_str("Serial PA1/PA2 configurado!\r\n").ok();
    serial.write_str("PA1: RX, PA2: TX\r\n").ok();
    serial.write_str("Velocidad: 115200 bps\r\n").ok();
    serial.write_str("Escribe algo y se repetirá...\r\n").ok();

    loop {
        // Esperar recepción de un byte
        let received: u8 = nb::block!(serial.read()).unwrap();

        // Eco del byte recibido
        nb::block!(serial.write(received)).ok();
        
        // Si es Enter, agregar nueva línea
        if received == b'\r' {
            nb::block!(serial.write(b'\n')).ok();
        }
    }
}
