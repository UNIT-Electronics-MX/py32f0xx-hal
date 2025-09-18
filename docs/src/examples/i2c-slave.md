# I2C Slave Communication

The `i2c_slave_demo.rs` example demonstrates I2C slave functionality on the PY32F0xx microcontroller with serial debug output. This example shows how to configure the microcontroller as an I2C slave device that can receive data from an I2C master.

## Overview

This example configures:
- **I2C Slave**: Address 0x50, 100kHz communication
- **GPIO Pins**: PA10 (SDA), PB6 (SCL) with AF6 alternate function
- **Serial Debug**: USART2 on PA0 (TX), PA1 (RX) at 9600 bps
- **Status LED**: PB5 for visual feedback

## Hardware Setup

### Pin Configuration
- **PA10**: I2C SDA (Serial Data) - AF6, Open Drain + Pull-up
- **PB6**: I2C SCL (Serial Clock) - AF6, Open Drain + Pull-up  
- **PA0**: USART2 TX (Debug output) - AF9
- **PA1**: USART2 RX (Debug input) - AF9
- **PB5**: Status LED (Push-pull output)

### External Connections
Connect external I2C pull-up resistors (4.7kΩ recommended) between:
- SDA (PA10) and VCC
- SCL (PB6) and VCC

## Code Structure

### Initialization
```rust
// Configure I2C pins with AF6
let _sda = gpioa.pa10.into_alternate_af6();   
let _scl = gpiob.pb6.into_alternate_af6();    

// GPIO configuration: Open Drain + Pull-up
unsafe {
    let gpioa = &(*pac::GPIOA::ptr());
    let gpiob = &(*pac::GPIOB::ptr());
    
    // PA10 (SDA): Open Drain + Pull-up
    gpioa.otyper.modify(|_, w| w.ot10().set_bit());     
    gpioa.pupdr.modify(|_, w| w.pupd10().pull_up());    
    
    // PB6 (SCL): Open Drain + Pull-up  
    gpiob.otyper.modify(|_, w| w.ot6().set_bit());      
    gpiob.pupdr.modify(|_, w| w.pupd6().pull_up());     
}
```

### I2C Slave Configuration
```rust
// Configure I2C SLAVE - Address 0x50
let slave_addr = 0x50_u8;
unsafe {
    let i2c = &(*pac::I2C::ptr());
    
    i2c.cr1.modify(|_, w| w.pe().clear_bit());
    i2c.cr2.write(|w| w.freq().bits(24_u8));    
    i2c.oar1.write(|w| w.add().bits(slave_addr)); 
    i2c.cr1.write(|w| w.ack().set_bit().pe().set_bit());
}
```

### Main Loop - Event Handling
```rust
loop {
    unsafe {
        let i2c = &(*pac::I2C::ptr());
        let sr1 = i2c.sr1.read();
        
        // Address matched - start of transaction
        if sr1.addr().bit_is_set() {
            contact_count += 1;
            writeln!(serial, "=== I2C Transaction #{} ===\r", contact_count).unwrap();
            
            // Clear ADDR flag (required for ACK)
            let _sr2_clear = i2c.sr2.read();
            i2c.cr1.modify(|_, w| w.ack().set_bit());
        }
        
        // Data received
        if sr1.rxne().bit_is_set() {
            let data = i2c.dr.read().dr().bits();
            byte_count += 1;
            
            writeln!(serial, "Data #{}: 0x{:02X} ({})", byte_count, data, data).unwrap();
            
            // Show ASCII if printable
            if data >= 32 && data <= 126 {
                writeln!(serial, "ASCII: '{}'\r", data as char).unwrap();
            }
        }
        
        // End of transaction
        if sr1.stopf().bit_is_set() {
            writeln!(serial, "Transaction complete\r\n").unwrap();
            
            // Clear STOP flag
            i2c.cr1.modify(|r, w| w.bits(r.bits()));
            i2c.cr1.modify(|_, w| w.ack().set_bit());
        }
    }
}
```

## Building and Running

### Build the Example
```bash
make build EXAMPLE=i2c_slave_demo
```

### Flash to Microcontroller
```bash
make flash EXAMPLE=i2c_slave_demo
```

### Monitor Serial Output
```bash
make monitor
# or
picocom /dev/ttyACM0 -b 9600
```

## Expected Output

When an I2C master communicates with the slave, you'll see output like:
```
=== I2C SLAVE DEMO ===
Configuration: PA10=SDA, PB6=SCL
I2C SLAVE configured at address 0x50
Waiting for I2C communication...

=== I2C Transaction #1 ===
Data #1: 0x42 (66)
ASCII: 'B'
Transaction complete

=== I2C Transaction #2 ===
Data #2: 0x48 (72)
ASCII: 'H'
Data #3: 0x65 (101)
ASCII: 'e'
Data #4: 0x6C (108)
ASCII: 'l'
Data #5: 0x6C (108)
ASCII: 'l'
Data #6: 0x6F (111)
ASCII: 'o'
Transaction complete
```

## Testing with ESP32-H2

You can test the I2C slave using an ESP32-H2 as master with MicroPython:

```python
from machine import I2C, Pin

# Configure I2C master
i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)

# Scan for devices
devices = i2c.scan()
print(f"I2C devices found: {[hex(d) for d in devices]}")

# Send data to slave at address 0x50
if 0x50 in devices:
    # Send single byte
    i2c.writeto(0x50, b'A')
    
    # Send multiple bytes
    i2c.writeto(0x50, b'Hello')
    
    # Send custom data
    data = bytearray([0x01, 0x02, 0x03, 0xFF])
    i2c.writeto(0x50, data)
```

## Key Features

- **Slave Address**: Configurable (default 0x50)
- **Data Reception**: Handles single and multi-byte transactions
- **Serial Debug**: Detailed transaction logging with hex, decimal, and ASCII display
- **Transaction Counting**: Tracks number of I2C communications
- **ACK Generation**: Proper acknowledgment handling for reliable communication
- **Status Indication**: Visual feedback through LED

## Troubleshooting

### No I2C Communication
- Check pull-up resistors on SDA and SCL lines
- Verify correct pin connections (PA10=SDA, PB6=SCL)
- Ensure master and slave use same frequency (100kHz)
- Check slave address matches in master code (0x50)

### Incomplete Data Reception
- Monitor ACK flag handling in main loop
- Verify STOP condition detection
- Check for proper flag clearing sequence

### Serial Output Issues
- Confirm USART2 connections (PA0=TX, PA1=RX)
- Verify baud rate settings (9600 bps)
- Check terminal configuration

## Related Examples

- `serial_echo.rs` - Basic serial communication
- `blinky.rs` - Basic GPIO control
- `pwm.rs` - PWM signal generation

## Technical Details

- **MCU**: PY32F003x4 series
- **I2C Speed**: 100kHz (Standard mode)
- **Clock**: 24MHz HSI
- **Addressing**: 7-bit addressing mode
- **Communication**: Receive-only slave implementation
