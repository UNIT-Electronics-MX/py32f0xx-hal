#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;

use py32f0xx_hal::{
    pac, 
    prelude::*, 
    rcc::{RccExt, HSIFreq},
};

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    // Configure RCC for 24MHz
    let rcc = p.RCC
        .configure()
        .hsi(HSIFreq::Freq24mhz) 
        .sysclk(24.MHz())        
        .freeze(&mut p.FLASH);

    // Initialize GPIO
    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();

    // USART2 for debug: PA0=TX, PA1=RX @ 9600bps
    let tx = gpioa.pa0.into_alternate_af9();
    let rx = gpioa.pa1.into_alternate_af9();
    let mut serial = p.USART2.serial((tx, rx), 9_600.bps(), &rcc.clocks);

    // Debug LED on PB5
    let mut debug_pin = gpiob.pb5.into_push_pull_output();
    debug_pin.set_low();

    // Simple startup blink
    for i in 1..=3 {
        writeln!(serial, "Blink {}\r", i).unwrap();
        debug_pin.set_high();
        for _ in 0..600_000 { cortex_m::asm::nop(); }
        debug_pin.set_low(); 
        for _ in 0..600_000 { cortex_m::asm::nop(); }
    }

    writeln!(serial, "=== I2C SLAVE DEMO ===\r").unwrap();
    writeln!(serial, "Configuracion: PA10=SDA, PB6=SCL\r").unwrap();

    // Configure I2C pins: PA10=SDA, PB6=SCL with AF6
    let _sda = gpioa.pa10.into_alternate_af6();   
    let _scl = gpiob.pb6.into_alternate_af6();    
    
    // GPIO configuration: Open Drain + Pull-up
    unsafe {
        let gpioa = &(*pac::GPIOA::ptr());
        let gpiob = &(*pac::GPIOB::ptr());
        
        // PA10 (SDA): Open Drain + Pull-up
        gpioa.otyper.modify(|_, w| w.ot10().set_bit());     
        gpioa.pupdr.modify(|_, w| w.pupd10().pull_up());    
        gpioa.ospeedr.modify(|_, w| w.ospeed10().very_high_speed()); 
        
        // PB6 (SCL): Open Drain + Pull-up  
        gpiob.otyper.modify(|_, w| w.ot6().set_bit());      
        gpiob.pupdr.modify(|_, w| w.pupd6().pull_up());     
        gpiob.ospeedr.modify(|_, w| w.ospeed6().very_high_speed()); 
    }

    // Enable I2C clock and reset
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.apbenr1.modify(|_, w| w.i2cen().set_bit());  
        rcc.apbrstr1.modify(|_, w| w.i2crst().set_bit());   
        rcc.apbrstr1.modify(|_, w| w.i2crst().clear_bit()); 
    }
    
    // Configure I2C SLAVE - Address 0x50
    let slave_addr = 0x50_u8;
    unsafe {
        let i2c = &(*pac::I2C::ptr());
        
        i2c.cr1.modify(|_, w| w.pe().clear_bit());
        i2c.cr2.write(|w| w.freq().bits(24_u8));    
        i2c.oar1.write(|w| w.add().bits(slave_addr)); 
        i2c.cr1.write(|w| w.ack().set_bit().pe().set_bit());
        i2c.cr1.modify(|_, w| w.ack().set_bit());
    }
    
    writeln!(serial, "I2C SLAVE configurado en direccion 0x{:02X}\r", slave_addr).unwrap();
    writeln!(serial, "Esperando comunicacion I2C...\r\n").unwrap();
    
    // I2C Slave Loop - Demo simple
    let mut contact_count = 0u32;
    let mut byte_count = 0u32;
    
    loop {
        unsafe {
            let i2c = &(*pac::I2C::ptr());
            let sr1 = i2c.sr1.read();
            
            // Nueva transacción I2C iniciada
            if sr1.addr().bit_is_set() {
                contact_count += 1;
                writeln!(serial, "=== I2C Transaccion #{} ===\r", contact_count).unwrap();
                
                // Limpiar ADDR flag (necesario para ACK)
                let _sr2_clear = i2c.sr2.read();
                i2c.cr1.modify(|_, w| w.ack().set_bit());
            }
            
            // Datos recibidos
            if sr1.rxne().bit_is_set() {
                let data = i2c.dr.read().dr().bits();
                byte_count += 1;
                
                writeln!(serial, "Dato #{}: 0x{:02X} ({})", byte_count, data, data).unwrap();
                
                // Si es ASCII, mostrarlo
                if data >= 32 && data <= 126 {
                    writeln!(serial, "ASCII: '{}'\r", data as char).unwrap();
                }
            }
            
            // Fin de transacción
            if sr1.stopf().bit_is_set() {
                writeln!(serial, "Transaccion completa\r\n").unwrap();
                
                // Limpiar STOP flag
                i2c.cr1.modify(|r, w| w.bits(r.bits()));
                i2c.cr1.modify(|_, w| w.ack().set_bit());
            }
        }
        
        // Pequeña pausa
        for _ in 0..500 {
            cortex_m::asm::nop();
        }
    }
}
