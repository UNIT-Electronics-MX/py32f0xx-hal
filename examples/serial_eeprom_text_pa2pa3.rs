#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    rcc::{RccExt, HSIFreq},
};

use cortex_m_rt::entry;
use embedded_hal_02::serial::{Read, Write as OtherWrite};

#[entry]
fn main() -> ! {
    // USE THE SAME CONFIGURATION AS BLINKY (WHICH WORKS)
    let mut p = pac::Peripherals::take().unwrap();

    // Configure RCC for 24MHz HSI (SAME AS WORKING BLINKY)
    let rcc = p.RCC
        .configure()
        .hsi(HSIFreq::Freq24mhz)  // Set HSI to 24MHz 
        .sysclk(24.MHz())         // Set system clock to 24MHz
        .freeze(&mut p.FLASH);

    // Initialize GPIO A and B
    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();

    // Pin configuration for USART2 (REQUESTED CONFIGURATION)
    // PA0 as TX with AF9 for USART2
    // PA1 as RX with AF9 for USART2
    let tx = gpioa.pa0.into_alternate_af9();
    let rx = gpioa.pa1.into_alternate_af9();

    // PB5 as debug pin (SAME AS WORKING BLINKY)
    let mut debug_pin = gpiob.pb5.into_push_pull_output();
    
    // Initialize debug pin LOW
    debug_pin.set_low();

    // Configure I2C pins: PA3=SCL, PA2=SDA with AF12
    let _scl = gpioa.pa3.into_alternate_af12();
    let _sda = gpioa.pa2.into_alternate_af12();
    
    // Configure GPIO registers for I2C: Open Drain + Pull-up
    unsafe {
        let gpioa = &(*pac::GPIOA::ptr());
        
        // PA2 (SDA): Open Drain + Pull-up
        gpioa.otyper.modify(|_, w| w.ot2().set_bit());     // Open drain
        gpioa.pupdr.modify(|_, w| w.pupd2().pull_up());    // Pull-up
        gpioa.ospeedr.modify(|_, w| w.ospeed2().very_high_speed()); // High speed
        
        // PA3 (SCL): Open Drain + Pull-up  
        gpioa.otyper.modify(|_, w| w.ot3().set_bit());     // Open drain
        gpioa.pupdr.modify(|_, w| w.pupd3().pull_up());    // Pull-up
        gpioa.ospeedr.modify(|_, w| w.ospeed3().very_high_speed()); // High speed
    }
    
    // Enable I2C clock and reset (manual configuration)
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.apbenr1.modify(|_, w| w.i2cen().set_bit());    // Enable I2C clock
        rcc.apbrstr1.modify(|_, w| w.i2crst().set_bit());  // Reset I2C
        rcc.apbrstr1.modify(|_, w| w.i2crst().clear_bit()); // Release reset
    }
    
    // Configure I2C peripheral manually for master mode
    unsafe {
        let i2c = &(*pac::I2C::ptr());
        
        // Disable I2C before configuration
        i2c.cr1.modify(|_, w| w.pe().clear_bit());
        
        // Configure timing for 100kHz @ 24MHz
        // Using standard I2C timing calculation
        i2c.cr2.write(|w| w.freq().bits(24_u8)); // 24MHz clock
        
        // Standard mode 100kHz: CCR = Tclk / (2 * Ti2c) = 24MHz / (2 * 100kHz) = 120
        i2c.ccr.write(|w| w.ccr().bits(120_u16));
        
        // Trise = (Trise_max / Tclk) + 1 = (1000ns / 41.67ns) + 1 = 25
        i2c.trise.write(|w| w.trise().bits(25_u8));
        
        // Enable I2C
        i2c.cr1.write(|w| w.pe().set_bit());
    }
    
    // Note: Using manual I2C implementation instead of HAL

    let mut serial = p.USART2.serial((tx, rx), 9_600.bps(), &rcc.clocks);
    serial.write_str("=== USART2 + I2C FT24C32A DEMO PA2/PA3 - 9600 bps ===\r\n").ok();
    serial.write_str("PA0: TX (AF9) - USART2\r\n").ok();
    serial.write_str("PA1: RX (AF9) - USART2\r\n").ok();
    serial.write_str("PA2: SDA (AF12) - I2C FT24C32A\r\n").ok();
    serial.write_str("PA3: SCL (AF12) - I2C FT24C32A\r\n").ok();
    serial.write_str("PB5: Debug LED\r\n").ok();
    serial.write_str("Clock: 24MHz confirmed\r\n").ok();
    serial.write_str("I2C: 100kHz mode - FT24C32A (16-bit addressing)\r\n").ok();
    
    // 3 INITIALIZATION BLINKS (VERY SLOW AND VISIBLE)
    serial.write_str("Starting 3 SLOW blinks...\r\n").ok();
    
    // Blink 1
    serial.write_str("Blink 1\r\n").ok();
    debug_pin.set_high();
    for _ in 0..1_200_000 {  // ~1 second (same as blinky)
        cortex_m::asm::nop();
    }
    debug_pin.set_low();
    for _ in 0..1_200_000 {  // ~1 second (same as blinky)
        cortex_m::asm::nop();
    }
    
    // Blink 2
    serial.write_str("Blink 2\r\n").ok();
    debug_pin.set_high();
    for _ in 0..1_200_000 {
        cortex_m::asm::nop();
    }
    debug_pin.set_low();
    for _ in 0..1_200_000 {
        cortex_m::asm::nop();
    }
    
    // Blink 3
    serial.write_str("Blink 3\r\n").ok();
    debug_pin.set_high();
    for _ in 0..1_200_000 {
        cortex_m::asm::nop();
    }
    debug_pin.set_low();
    for _ in 0..1_200_000 {
        cortex_m::asm::nop();
    }
    
    serial.write_str("=== System Ready! ===\r\n").ok();
    // Test I2C lines first
    serial.write_str("Probando lineas I2C...\r\n").ok();
    unsafe {
        let gpiof = &(*pac::GPIOF::ptr());
        let input_reg = gpiof.idr.read();
        
        if input_reg.id0().bit_is_clear() {
            serial.write_str("ADVERTENCIA: PF0 (SDA) esta en LOW - posible corto a GND\r\n").ok();
        }
        if input_reg.id1().bit_is_clear() {
            serial.write_str("ADVERTENCIA: PF1 (SCL) esta en LOW - posible corto a GND\r\n").ok();
        }
        
        if input_reg.id0().bit_is_set() && input_reg.id1().bit_is_set() {
            serial.write_str("Lineas I2C OK - ambas en HIGH\r\n").ok();
        }
    }
    
    serial.write_str("Comandos disponibles (FT24C32A - 16-bit addressing):\r\n").ok();
    serial.write_str("'w' - Escribir 0xAA a address 0x0000\r\n").ok();
    serial.write_str("'x' - Escribir 0x55 a address 0x0000\r\n").ok();
    serial.write_str("'y' - Escribir 0x33 a address 0x0001\r\n").ok();
    serial.write_str("'r' - Leer address 0x0000\r\n").ok();
    serial.write_str("'z' - Leer address 0x0001\r\n").ok();
    serial.write_str("'s' - Scan I2C devices\r\n").ok();
    serial.write_str("'t' - Test lineas I2C\r\n").ok();
    serial.write_str("Otro - Echo del caracter\r\n").ok();

    // EEPROM address (configurada para 0x50)
    const EEPROM_ADDR: u8 = 0x50;

    loop {
        // Indicate waiting for data (debug pin LOW)
        debug_pin.set_low();
        
        // Wait for reception of a single byte
        let received: u8 = nb::block!(serial.read()).unwrap();

        // Indicate processing data (debug pin HIGH)
        debug_pin.set_high();
        
        match received {
            b'w' => {
                // Escribir a FT24C32A (direccionamiento 16-bit)
                serial.write_str("Escribiendo 0xAA a FT24C32A address 0x0000 (16-bit)...\r\n").ok();
                
                unsafe {
                    let i2c = &(*pac::I2C::ptr());
                    let mut success = false;
                    let mut step = 0u8;
                    
                    // Generate START condition
                    i2c.cr1.modify(|_, w| w.start().set_bit());
                    step = 1;
                    
                    // Wait for START condition
                    let mut timeout = 50000;
                    while !i2c.sr1.read().sb().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if timeout > 0 {
                        step = 2;
                        // Send EEPROM address with write bit
                        i2c.dr.write(|w| w.dr().bits(EEPROM_ADDR << 1));
                        
                        // Wait for address ACK
                        timeout = 50000;
                        while !i2c.sr1.read().addr().bit_is_set() && 
                              !i2c.sr1.read().af().bit_is_set() && 
                              timeout > 0 {
                            timeout -= 1;
                        }
                        
                        if i2c.sr1.read().addr().bit_is_set() {
                            step = 3;
                            // Clear ADDR flag
                            let _ = i2c.sr1.read();
                            let _ = i2c.sr2.read();
                            
                            // Send memory address HIGH byte (0x00)
                            i2c.dr.write(|w| w.dr().bits(0x00_u8));
                            
                            // Wait for TXE
                            timeout = 50000;
                            while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                timeout -= 1;
                            }
                            
                            if timeout > 0 {
                                step = 4;
                                // Send memory address LOW byte (0x00)
                                i2c.dr.write(|w| w.dr().bits(0x00_u8));
                                
                                // Wait for TXE
                                timeout = 50000;
                                while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                    timeout -= 1;
                                }
                                
                                if timeout > 0 {
                                    step = 5;
                                    // Send data (0xAA)
                                    i2c.dr.write(|w| w.dr().bits(0xAA_u8));
                                    
                                    // Wait for TXE and BTF (Byte Transfer Finished)
                                    timeout = 50000;
                                    while (!i2c.sr1.read().txe().bit_is_set() || 
                                           !i2c.sr1.read().btf().bit_is_set()) && timeout > 0 {
                                        timeout -= 1;
                                    }
                                    
                                    if timeout > 0 {
                                        step = 6;
                                        success = true;
                                    }
                                }
                            }
                        } else if i2c.sr1.read().af().bit_is_set() {
                            step = 10; // NACK received
                        }
                        
                        // Clear AF flag if set
                        if i2c.sr1.read().af().bit_is_set() {
                            i2c.sr1.modify(|_, w| w.af().clear_bit());
                        }
                    }
                    
                    // Generate STOP condition
                    i2c.cr1.modify(|_, w| w.stop().set_bit());
                    
                    // Wait for STOP to complete
                    timeout = 10000;
                    while i2c.cr1.read().stop().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if success {
                        write!(serial, "Escritura exitosa! (paso {})\r\n", step).ok();
                    } else {
                        write!(serial, "Error en escritura FT24C32A (paso {})\r\n", step).ok();
                    }
                }
                
                // Delay para write cycle de FT24C32A (hasta 10ms)
                serial.write_str("Esperando write cycle (10ms)...\r\n").ok();
                for _ in 0..480_000 {  // ~10ms
                    cortex_m::asm::nop();
                }
            },
            b'x' => {
                // Escribir 0x55 a FT24C32A (patrón alternativo para test)
                serial.write_str("Escribiendo 0x55 a FT24C32A address 0x0000 (16-bit)...\r\n").ok();
                
                unsafe {
                    let i2c = &(*pac::I2C::ptr());
                    let mut success = false;
                    let mut step = 0u8;
                    
                    // Generate START condition
                    i2c.cr1.modify(|_, w| w.start().set_bit());
                    step = 1;
                    
                    // Wait for START condition
                    let mut timeout = 50000;
                    while !i2c.sr1.read().sb().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if timeout > 0 {
                        step = 2;
                        // Send EEPROM address with write bit
                        i2c.dr.write(|w| w.dr().bits(EEPROM_ADDR << 1));
                        
                        // Wait for address ACK
                        timeout = 50000;
                        while !i2c.sr1.read().addr().bit_is_set() && 
                              !i2c.sr1.read().af().bit_is_set() && 
                              timeout > 0 {
                            timeout -= 1;
                        }
                        
                        if i2c.sr1.read().addr().bit_is_set() {
                            step = 3;
                            // Clear ADDR flag
                            let _ = i2c.sr1.read();
                            let _ = i2c.sr2.read();
                            
                            // Send memory address HIGH byte (0x00)
                            i2c.dr.write(|w| w.dr().bits(0x00_u8));
                            
                            // Wait for TXE
                            timeout = 50000;
                            while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                timeout -= 1;
                            }
                            
                            if timeout > 0 {
                                step = 4;
                                // Send memory address LOW byte (0x00)
                                i2c.dr.write(|w| w.dr().bits(0x00_u8));
                                
                                // Wait for TXE
                                timeout = 50000;
                                while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                    timeout -= 1;
                                }
                                
                                if timeout > 0 {
                                    step = 5;
                                    // Send data (0x55)
                                    i2c.dr.write(|w| w.dr().bits(0x55_u8));
                                
                                    // Wait for TXE and BTF (Byte Transfer Finished)
                                    timeout = 50000;
                                    while (!i2c.sr1.read().txe().bit_is_set() || 
                                           !i2c.sr1.read().btf().bit_is_set()) && timeout > 0 {
                                        timeout -= 1;
                                    }
                                    
                                    if timeout > 0 {
                                        step = 6;
                                        success = true;
                                    }
                                }
                            }
                        } else if i2c.sr1.read().af().bit_is_set() {
                            step = 10; // NACK received
                        }
                        
                        // Clear AF flag if set
                        if i2c.sr1.read().af().bit_is_set() {
                            i2c.sr1.modify(|_, w| w.af().clear_bit());
                        }
                    }
                    
                    // Generate STOP condition
                    i2c.cr1.modify(|_, w| w.stop().set_bit());
                    
                    // Wait for STOP to complete
                    timeout = 10000;
                    while i2c.cr1.read().stop().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if success {
                        write!(serial, "Escritura exitosa! (paso {})\r\n", step).ok();
                    } else {
                        write!(serial, "Error en escritura FT24C32A (paso {})\r\n", step).ok();
                    }
                }
                
                // Delay para write cycle de FT24C32A
                serial.write_str("Esperando write cycle (10ms)...\r\n").ok();
                for _ in 0..480_000 {  // ~10ms
                    cortex_m::asm::nop();
                }
            },
            b'r' => {
                // Leer de FT24C32A con direccionamiento 16-bit
                serial.write_str("Leyendo FT24C32A address 0x0000 (16-bit random read)...\r\n").ok();
                
                unsafe {
                    let i2c = &(*pac::I2C::ptr());
                    let mut read_data = 0u8;
                    let mut success = false;
                    let mut step = 0u8;
                    
                    // PASO 1: Escribir dirección de memoria 16-bit (Dummy Write)
                    // Generate START condition
                    i2c.cr1.modify(|_, w| w.start().set_bit());
                    step = 1;
                    
                    // Wait for START condition
                    let mut timeout = 50000;
                    while !i2c.sr1.read().sb().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if timeout > 0 {
                        step = 2;
                        // Send EEPROM address with WRITE bit
                        i2c.dr.write(|w| w.dr().bits(EEPROM_ADDR << 1));
                        
                        // Wait for address ACK
                        timeout = 50000;
                        while !i2c.sr1.read().addr().bit_is_set() && 
                              !i2c.sr1.read().af().bit_is_set() && 
                              timeout > 0 {
                            timeout -= 1;
                        }
                        
                        if i2c.sr1.read().addr().bit_is_set() {
                            step = 3;
                            // Clear ADDR flag
                            let _ = i2c.sr1.read();
                            let _ = i2c.sr2.read();
                            
                            // Send memory address HIGH byte (0x00)
                            i2c.dr.write(|w| w.dr().bits(0x00_u8));
                            
                            // Wait for TXE
                            timeout = 50000;
                            while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                timeout -= 1;
                            }
                            
                            if timeout > 0 {
                                step = 4;
                                // Send memory address LOW byte (0x00)
                                i2c.dr.write(|w| w.dr().bits(0x00_u8));
                                
                                // Wait for TXE
                                timeout = 50000;
                                while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                    timeout -= 1;
                                }
                                
                                if timeout > 0 {
                                    step = 5;
                                    
                                    // PASO 2: Repeated START para lectura
                                    // Generate repeated START
                                    i2c.cr1.modify(|_, w| w.start().set_bit());
                                    
                                    // Wait for START condition
                                    timeout = 50000;
                                    while !i2c.sr1.read().sb().bit_is_set() && timeout > 0 {
                                        timeout -= 1;
                                    }
                                    
                                    if timeout > 0 {
                                        step = 6;
                                        // Send EEPROM address with READ bit
                                        i2c.dr.write(|w| w.dr().bits((EEPROM_ADDR << 1) | 1));
                                        
                                        // Wait for address ACK
                                        timeout = 50000;
                                        while !i2c.sr1.read().addr().bit_is_set() && 
                                              !i2c.sr1.read().af().bit_is_set() && 
                                              timeout > 0 {
                                            timeout -= 1;
                                        }
                                        
                                        if i2c.sr1.read().addr().bit_is_set() {
                                            step = 7;
                                            // Disable ACK for single byte read
                                            i2c.cr1.modify(|_, w| w.ack().clear_bit());
                                            
                                            // Clear ADDR flag
                                            let _ = i2c.sr1.read();
                                            let _ = i2c.sr2.read();
                                            
                                            // Generate STOP condition
                                            i2c.cr1.modify(|_, w| w.stop().set_bit());
                                            
                                            // Wait for RXNE
                                            timeout = 50000;
                                            while !i2c.sr1.read().rxne().bit_is_set() && timeout > 0 {
                                                timeout -= 1;
                                            }
                                            
                                            if timeout > 0 {
                                                step = 8;
                                                // Read data
                                                read_data = i2c.dr.read().dr().bits();
                                                success = true;
                                            }
                                            
                                            // Re-enable ACK for future operations
                                            i2c.cr1.modify(|_, w| w.ack().set_bit());
                                        } else if i2c.sr1.read().af().bit_is_set() {
                                            step = 20; // NACK en read
                                        }
                                    }
                                }
                            }
                        } else if i2c.sr1.read().af().bit_is_set() {
                            step = 10; // NACK en write
                        }
                        
                        // Clear AF flag if set
                        if i2c.sr1.read().af().bit_is_set() {
                            i2c.sr1.modify(|_, w| w.af().clear_bit());
                        }
                    }
                    
                    // Ensure STOP is generated
                    i2c.cr1.modify(|_, w| w.stop().set_bit());
                    
                    // Wait for STOP to complete
                    timeout = 10000;
                    while i2c.cr1.read().stop().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if success {
                        write!(serial, "Dato leido: 0x{:02X} (paso {})\r\n", read_data, step).ok();
                    } else {
                        write!(serial, "Error en lectura FT24C32A (paso {})\r\n", step).ok();
                    }
                }
            },
            b'y' => {
                // Escribir 0x33 a dirección 0x0001 (para test de múltiples direcciones)
                serial.write_str("Escribiendo 0x33 a FT24C32A address 0x0001 (16-bit)...\r\n").ok();
                
                unsafe {
                    let i2c = &(*pac::I2C::ptr());
                    let mut success = false;
                    let mut step = 0u8;
                    
                    // Generate START condition
                    i2c.cr1.modify(|_, w| w.start().set_bit());
                    step = 1;
                    
                    // Wait for START condition
                    let mut timeout = 50000;
                    while !i2c.sr1.read().sb().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if timeout > 0 {
                        step = 2;
                        // Send EEPROM address with write bit
                        i2c.dr.write(|w| w.dr().bits(EEPROM_ADDR << 1));
                        
                        // Wait for address ACK
                        timeout = 50000;
                        while !i2c.sr1.read().addr().bit_is_set() && 
                              !i2c.sr1.read().af().bit_is_set() && 
                              timeout > 0 {
                            timeout -= 1;
                        }
                        
                        if i2c.sr1.read().addr().bit_is_set() {
                            step = 3;
                            // Clear ADDR flag
                            let _ = i2c.sr1.read();
                            let _ = i2c.sr2.read();
                            
                            // Send memory address HIGH byte (0x00)
                            i2c.dr.write(|w| w.dr().bits(0x00_u8));
                            
                            // Wait for TXE
                            timeout = 50000;
                            while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                timeout -= 1;
                            }
                            
                            if timeout > 0 {
                                step = 4;
                                // Send memory address LOW byte (0x01)
                                i2c.dr.write(|w| w.dr().bits(0x01_u8));
                                
                                // Wait for TXE
                                timeout = 50000;
                                while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                    timeout -= 1;
                                }
                                
                                if timeout > 0 {
                                    step = 5;
                                    // Send data (0x33)
                                    i2c.dr.write(|w| w.dr().bits(0x33_u8));
                                
                                    // Wait for TXE and BTF
                                    timeout = 50000;
                                    while (!i2c.sr1.read().txe().bit_is_set() || 
                                           !i2c.sr1.read().btf().bit_is_set()) && timeout > 0 {
                                        timeout -= 1;
                                    }
                                    
                                    if timeout > 0 {
                                        step = 6;
                                        success = true;
                                    }
                                }
                            }
                        } else if i2c.sr1.read().af().bit_is_set() {
                            step = 10; // NACK received
                        }
                        
                        // Clear AF flag if set
                        if i2c.sr1.read().af().bit_is_set() {
                            i2c.sr1.modify(|_, w| w.af().clear_bit());
                        }
                    }
                    
                    // Generate STOP condition
                    i2c.cr1.modify(|_, w| w.stop().set_bit());
                    
                    // Wait for STOP to complete
                    timeout = 10000;
                    while i2c.cr1.read().stop().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if success {
                        write!(serial, "Escritura exitosa! (paso {})\r\n", step).ok();
                    } else {
                        write!(serial, "Error en escritura FT24C32A (paso {})\r\n", step).ok();
                    }
                }
                
                // Delay para write cycle
                serial.write_str("Esperando write cycle (10ms)...\r\n").ok();
                for _ in 0..480_000 {
                    cortex_m::asm::nop();
                }
            },
            b'z' => {
                // Leer dirección 0x0001
                serial.write_str("Leyendo FT24C32A address 0x0001 (16-bit random read)...\r\n").ok();
                
                unsafe {
                    let i2c = &(*pac::I2C::ptr());
                    let mut read_data = 0u8;
                    let mut success = false;
                    let mut step = 0u8;
                    
                    // PASO 1: Escribir dirección de memoria (0x01)
                    // Generate START condition
                    i2c.cr1.modify(|_, w| w.start().set_bit());
                    step = 1;
                    
                    // Wait for START condition
                    let mut timeout = 50000;
                    while !i2c.sr1.read().sb().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if timeout > 0 {
                        step = 2;
                        // Send EEPROM address with WRITE bit
                        i2c.dr.write(|w| w.dr().bits(EEPROM_ADDR << 1));
                        
                        // Wait for address ACK
                        timeout = 50000;
                        while !i2c.sr1.read().addr().bit_is_set() && 
                              !i2c.sr1.read().af().bit_is_set() && 
                              timeout > 0 {
                            timeout -= 1;
                        }
                        
                        if i2c.sr1.read().addr().bit_is_set() {
                            step = 3;
                            // Clear ADDR flag
                            let _ = i2c.sr1.read();
                            let _ = i2c.sr2.read();
                            
                            // Send memory address HIGH byte (0x00)
                            i2c.dr.write(|w| w.dr().bits(0x00_u8));
                            
                            // Wait for TXE
                            timeout = 50000;
                            while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                timeout -= 1;
                            }
                            
                            if timeout > 0 {
                                step = 4;
                                // Send memory address LOW byte (0x01)
                                i2c.dr.write(|w| w.dr().bits(0x01_u8));
                                
                                // Wait for TXE
                                timeout = 50000;
                                while !i2c.sr1.read().txe().bit_is_set() && timeout > 0 {
                                    timeout -= 1;
                                }
                                
                                if timeout > 0 {
                                    step = 5;
                                    
                                    // PASO 2: Repeated START para lectura
                                    // Generate repeated START
                                    i2c.cr1.modify(|_, w| w.start().set_bit());
                                    
                                    // Wait for START condition
                                    timeout = 50000;
                                    while !i2c.sr1.read().sb().bit_is_set() && timeout > 0 {
                                        timeout -= 1;
                                    }
                                    
                                    if timeout > 0 {
                                        step = 6;
                                    // Send EEPROM address with READ bit
                                    i2c.dr.write(|w| w.dr().bits((EEPROM_ADDR << 1) | 1));
                                    
                                    // Wait for address ACK
                                    timeout = 50000;
                                    while !i2c.sr1.read().addr().bit_is_set() && 
                                          !i2c.sr1.read().af().bit_is_set() && 
                                          timeout > 0 {
                                        timeout -= 1;
                                    }
                                    
                                        if i2c.sr1.read().addr().bit_is_set() {
                                            step = 7;
                                            // Disable ACK for single byte read
                                            i2c.cr1.modify(|_, w| w.ack().clear_bit());                                        // Clear ADDR flag
                                        let _ = i2c.sr1.read();
                                        let _ = i2c.sr2.read();
                                        
                                        // Generate STOP condition
                                        i2c.cr1.modify(|_, w| w.stop().set_bit());
                                        
                                        // Wait for RXNE
                                        timeout = 50000;
                                        while !i2c.sr1.read().rxne().bit_is_set() && timeout > 0 {
                                            timeout -= 1;
                                        }
                                        
                                        if timeout > 0 {
                                            step = 7;
                                            // Read data
                                            read_data = i2c.dr.read().dr().bits();
                                            success = true;
                                        }
                                        
                                        // Re-enable ACK for future operations
                                        i2c.cr1.modify(|_, w| w.ack().set_bit());
                                    }
                                }
                            }
                        }
                        }
                        
                        // Clear AF flag if set
                        if i2c.sr1.read().af().bit_is_set() {
                            i2c.sr1.modify(|_, w| w.af().clear_bit());
                        }
                    }
                    
                    // Ensure STOP is generated
                    i2c.cr1.modify(|_, w| w.stop().set_bit());
                    
                    // Wait for STOP to complete
                    timeout = 10000;
                    while i2c.cr1.read().stop().bit_is_set() && timeout > 0 {
                        timeout -= 1;
                    }
                    
                    if success {
                        write!(serial, "Dato leido: 0x{:02X} (paso {})\r\n", read_data, step).ok();
                    } else {
                        write!(serial, "Error en lectura EEPROM (paso {})\r\n", step).ok();
                    }
                }
            },
            b's' => {
                // Scan I2C devices with manual implementation
                serial.write_str("Escaneando dispositivos I2C (manual)...\r\n").ok();
                let mut found_devices = 0;
                
                unsafe {
                    let i2c = &(*pac::I2C::ptr());
                    
                    for addr in 0x08..=0x77 {
                        // Generate START condition
                        i2c.cr1.modify(|_, w| w.start().set_bit());
                        
                        // Wait for START condition to be sent
                        let mut timeout = 10000;
                        while !i2c.sr1.read().sb().bit_is_set() && timeout > 0 {
                            timeout -= 1;
                        }
                        
                        if timeout > 0 {
                            // Send address with write bit
                            i2c.dr.write(|w| w.dr().bits(addr << 1));
                            
                            // Wait for address to be sent or NACK
                            timeout = 10000;
                            while !i2c.sr1.read().addr().bit_is_set() && 
                                  !i2c.sr1.read().af().bit_is_set() && 
                                  timeout > 0 {
                                timeout -= 1;
                            }
                            
                            if i2c.sr1.read().addr().bit_is_set() {
                                // Device responded - clear ADDR flag
                                let _ = i2c.sr1.read();
                                let _ = i2c.sr2.read();
                                
                                write!(serial, "Dispositivo encontrado en 0x{:02X}\r\n", addr).ok();
                                found_devices += 1;
                            }
                            
                            // Clear AF flag if set
                            if i2c.sr1.read().af().bit_is_set() {
                                i2c.sr1.modify(|_, w| w.af().clear_bit());
                            }
                        }
                        
                        // Generate STOP condition
                        i2c.cr1.modify(|_, w| w.stop().set_bit());
                        
                        // Small delay between addresses
                        for _ in 0..1000 {
                            cortex_m::asm::nop();
                        }
                    }
                }
                
                write!(serial, "Scan completo. {} dispositivos encontrados.\r\n", found_devices).ok();
            },
            b't' => {
                // Test I2C lines
                serial.write_str("Test de lineas I2C:\r\n").ok();
                unsafe {
                    let gpiof = &(*pac::GPIOF::ptr());
                    let input_reg = gpiof.idr.read();
                    
                    write!(serial, "PF0 (SDA): {}\r\n", 
                          if input_reg.id0().bit_is_set() { "HIGH" } else { "LOW" }).ok();
                    write!(serial, "PF1 (SCL): {}\r\n", 
                          if input_reg.id1().bit_is_set() { "HIGH" } else { "LOW" }).ok();
                    
                    if input_reg.id0().bit_is_set() && input_reg.id1().bit_is_set() {
                        serial.write_str("Estado: OK - Listas para I2C\r\n").ok();
                    } else {
                        serial.write_str("Estado: ERROR - Lineas no estan en HIGH\r\n").ok();
                        serial.write_str("Verificar:\r\n").ok();
                        serial.write_str("- Resistencias pull-up 4.7k a 3.3V\r\n").ok();
                        serial.write_str("- No cortocircuitos a GND\r\n").ok();
                    }
                }
            },
            _ => {
                // Echo del carácter recibido
                nb::block!(serial.write(received)).ok();
                serial.write_str(" [echo]\r\n").ok();
            }
        }
        
        // Small pause to visualize the pulse
        for _ in 0..120_000 {  // ~100ms
            cortex_m::asm::nop();
        }
    }
}
