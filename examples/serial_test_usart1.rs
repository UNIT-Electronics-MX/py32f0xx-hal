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

    // PROBAR CONFIGURACIÓN DE USART1 con PA2/PA3 (configuración estándar)
    let tx1 = system.gpioa.pa2.into_alternate_af1();
    let rx1 = system.gpioa.pa3.into_alternate_af1();
    
    let mut serial1 = p.USART1.serial((tx1, rx1), 115_200.bps(), &system.rcc.clocks);
    
    // Indicar que USART1 está configurado con 5 blinks rápidos
    for _ in 0..5 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low();
        cortex_m::asm::delay(1_200_000);
    }
    
    serial1.write_str("=== USART1 TEST OK ===\r\n").ok();
    serial1.write_str("PA2: TX (AF1)\r\n").ok();
    serial1.write_str("PA3: RX (AF1)\r\n").ok();
    serial1.write_str("Velocidad: 115200 bps\r\n").ok();
    serial1.write_str("Escriba algo para test...\r\n").ok();
    
    loop {
        led.set_low();
        let received: u8 = nb::block!(serial1.read()).unwrap();
        led.set_high();
        nb::block!(serial1.write(received)).ok();
        cortex_m::asm::delay(240_000);
    }
}
