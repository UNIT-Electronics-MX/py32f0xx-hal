#![no_main]
#![no_std]

use panic_halt as _;
use py32f0xx_hal as hal;
use crate::hal::pac;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    
    unsafe {
        // Habilitar clock para GPIOA usando direcciones directas
        let rcc_iopenr = 0x4002_1034 as *mut u32;
        let mut iopenr = core::ptr::read_volatile(rcc_iopenr);
        iopenr |= 1 << 0; // GPIOAEN
        core::ptr::write_volatile(rcc_iopenr, iopenr);
        
        // Configurar PA1 como salida usando direcciones directas
        let gpioa_moder = 0x4800_0000 as *mut u32;
        let mut moder = core::ptr::read_volatile(gpioa_moder);
        moder &= !(0b11 << 2); // Limpiar bits 3:2
        moder |= 0b01 << 2;    // Modo salida para PA1
        core::ptr::write_volatile(gpioa_moder, moder);
        
        // Poner PA1 en LOW (0V)
        let gpioa_odr = 0x4800_0014 as *mut u32;
        let mut odr = core::ptr::read_volatile(gpioa_odr);
        odr &= !(1 << 1); // PA1 = 0
        core::ptr::write_volatile(gpioa_odr, odr);
    }

    // Bucle infinito
    loop {
        cortex_m::asm::nop();
    }
}
