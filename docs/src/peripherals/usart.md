# USART/Serial Communication

The PY32F0xx family provides flexible USART (Universal Synchronous/Asynchronous Receiver/Transmitter) peripherals for serial communication.

## Available USART Peripherals

| Device | USART1 | USART2 | Notes |
|--------|--------|--------|-------|
| **PY32F002A** | ✅ | ❌ | Single USART |
| **PY32F002B** | ✅ | ❌ | Single USART |
| **PY32F003** | ✅ | ✅ | Dual USART |
| **PY32F030** | ✅ | ✅ | Dual USART |

## Pin Configurations

### USART1 Pin Mappings

| Pin | AF | Function | Package Availability |
|-----|----|-----------|--------------------|
| PA1 | AF1 | USART1_RX | All packages |
| PA2 | AF1 | USART1_TX | All packages |
| PA9 | AF1 | USART1_TX | TSSOP20+ only |
| PA10| AF1 | USART1_RX | TSSOP20+ only |

### USART2 Pin Mappings

| Pin | AF | Function | Package Availability |
|-----|----|-----------|--------------------|
| PA0 | AF9 | USART2_TX | All packages |
| PA1 | AF9 | USART2_RX | All packages |
| PA2 | AF4 | USART2_TX | All packages (alternative) |
| PA14| AF1 | USART2_TX | All packages (alternative) |

## Basic USART Setup

### Simple Configuration

```rust
use py32f0xx_hal::{
    pac,
    prelude::*,
    rcc::HSIFreq,
    serial::Serial,
};

// Get peripherals
let mut p = pac::Peripherals::take().unwrap();

// Configure clock
let rcc = p.RCC
    .configure()
    .hsi(HSIFreq::Freq24mhz)
    .sysclk(24.MHz())
    .freeze(&mut p.FLASH);

// Setup GPIO
let gpioa = p.GPIOA.split();

// Configure pins for USART2
let tx = gpioa.pa0.into_alternate_af9();
let rx = gpioa.pa1.into_alternate_af9();

// Create serial interface
let mut serial = Serial::usart2(
    p.USART2,
    (tx, rx),
    9600.bps(),
    rcc.clocks,
);
```

### Advanced Configuration

```rust
use py32f0xx_hal::serial::{Config, Parity, StopBits};

// Custom serial configuration
let config = Config {
    baudrate: 115_200.bps(),
    parity: Parity::ParityNone,
    stopbits: StopBits::STOP1,
    // Additional config options...
};

let mut serial = Serial::usart1(
    p.USART1,
    (tx, rx),
    config,
    rcc.clocks,
);
```

## Supported Baud Rates

The USART peripheral supports a wide range of baud rates, limited by the system clock:

### Common Baud Rates (24MHz System Clock)

| Baud Rate | Typical Use Case | Error Rate |
|-----------|------------------|------------|
| 2400 | Low-speed sensors | < 0.1% |
| 4800 | Legacy devices | < 0.1% |
| 9600 | General purpose | < 0.1% |
| 19200 | Faster communication | < 0.1% |
| 38400 | High-speed sensors | < 0.1% |
| 57600 | Fast data transfer | < 0.2% |
| 115200 | Maximum practical | < 0.5% |

### Calculating Baud Rate

```rust
// Baud rate = Clock / (16 * USARTDIV)
// For 24MHz clock and 9600 baud:
// 9600 = 24,000,000 / (16 * USARTDIV)
// USARTDIV = 156.25 ≈ 156

let serial = Serial::usart2(
    p.USART2,
    (tx, rx),
    9600.bps(),  // HAL calculates USARTDIV automatically
    rcc.clocks,
);
```

## Reading and Writing Data

### Basic I/O Operations

```rust
use embedded_hal_02::serial::{Read, Write};

// Write single character
serial.write(b'A').ok();

// Write string
use core::fmt::Write;
serial.write_str(\"Hello World!\\r\\n\").ok();

// Read single character
if let Ok(byte) = serial.read() {
    // Process received byte
    println!(\"Received: {}\", byte as char);
}
```

### Buffered Operations

```rust
// Write multiple bytes
let message = b\"Hello PY32F0xx!\";
for &byte in message {
    serial.write(byte).ok();
}

// Read with timeout handling
use cortex_m::interrupt;

let mut buffer = [0u8; 64];
let mut index = 0;

loop {
    match serial.read() {
        Ok(byte) => {
            buffer[index] = byte;
            index += 1;
            
            if byte == b'\\n' || index >= buffer.len() {
                // Process complete message
                break;
            }
        },
        Err(nb::Error::WouldBlock) => {
            // No data available, continue
        },
        Err(nb::Error::Other(_)) => {
            // Handle error
            break;
        }
    }
}
```

### Interrupt-Driven Communication

```rust
use py32f0xx_hal::{
    pac::interrupt,
    serial::{Event, Serial},
};

// Enable RX interrupt
serial.listen(Event::Rxne);

// In interrupt handler
#[interrupt]
fn USART2() {
    // Handle received data
    if let Ok(byte) = SERIAL.read() {
        // Process byte
    }
}
```

## Error Handling

### Common Errors

```rust
use py32f0xx_hal::serial::Error;

match serial.read() {
    Ok(byte) => {
        // Process byte
    },
    Err(nb::Error::WouldBlock) => {
        // No data available
    },
    Err(nb::Error::Other(Error::Overrun)) => {
        // Data overrun - clear error
        serial.clear_overrun_error();
    },
    Err(nb::Error::Other(Error::Noise)) => {
        // Noise detected on line
    },
    Err(nb::Error::Other(Error::Framing)) => {
        // Framing error - incorrect stop bit
    },
    Err(nb::Error::Other(Error::Parity)) => {
        // Parity error
    },
}
```

### Error Recovery

```rust
// Clear all errors
fn clear_serial_errors(serial: &mut Serial<USART2>) {
    // Read status register to clear flags
    let _ = serial.clear_overrun_error();
    let _ = serial.clear_noise_error();
    let _ = serial.clear_framing_error();
    let _ = serial.clear_parity_error();
}
```

## DMA Integration

For high-throughput applications, USART can be used with DMA:

```rust
use py32f0xx_hal::dma::{dma1, Transfer, W, R};

// Setup DMA for TX
let tx_channel = dma1.ch2;
let tx_transfer = Transfer::init_memory_to_peripheral(
    tx_channel,
    serial.tx(),
    tx_buffer,
    None,
);

// Start DMA transfer
tx_transfer.start(|serial_tx| {
    serial_tx.enable_dma_tx();
});

// Setup DMA for RX
let rx_channel = dma1.ch3;
let rx_transfer = Transfer::init_peripheral_to_memory(
    rx_channel,
    serial.rx(),
    rx_buffer,
    None,
);
```

## Flow Control

### Software Flow Control (XON/XOFF)

```rust
const XON: u8 = 0x11;   // Resume transmission
const XOFF: u8 = 0x13;  // Pause transmission

// Send flow control characters
serial.write(XOFF).ok(); // Pause sender
serial.write(XON).ok();  // Resume sender

// Handle received flow control
match serial.read() {
    Ok(XON) => {
        // Resume sending
        tx_enabled = true;
    },
    Ok(XOFF) => {
        // Pause sending
        tx_enabled = false;
    },
    Ok(byte) => {
        // Normal data
    },
    _ => {}
}
```

### Hardware Flow Control (RTS/CTS)

```rust
// Configure RTS/CTS pins (if available on package)
let rts = gpioa.pa12.into_alternate_af1(); // USART1_RTS
let cts = gpioa.pa11.into_alternate_af1(); // USART1_CTS

// Enable hardware flow control
let mut serial = Serial::usart1(
    p.USART1,
    (tx, rx),
    config.rts(rts).cts(cts),
    rcc.clocks,
);
```

## Power Management

### Low Power Operation

```rust
// Disable USART when not needed
serial.disable();

// Re-enable when needed
serial.enable();

// Use lower baud rates for better power efficiency
let serial = Serial::usart2(
    p.USART2,
    (tx, rx),
    2400.bps(),  // Lower baud = lower power
    rcc.clocks,
);
```

### Wake-up from Stop Mode

```rust
// Configure USART for wake-up
serial.enable_wakeup();
serial.set_wakeup_method(WakeupMethod::StartBit);

// Enter stop mode
cortex_m::asm::wfi();

// USART activity will wake the MCU
```

## Practical Examples

### Command Interface

```rust
struct CommandProcessor {
    buffer: [u8; 64],
    index: usize,
}

impl CommandProcessor {
    fn process_byte(&mut self, byte: u8, serial: &mut Serial<USART2>) {
        match byte {
            b'\\r' | b'\\n' => {
                // Process complete command
                let command = &self.buffer[..self.index];
                self.handle_command(command, serial);
                self.index = 0;
            },
            b => {
                if self.index < self.buffer.len() {
                    self.buffer[self.index] = b;
                    self.index += 1;
                }
            }
        }
    }
    
    fn handle_command(&self, cmd: &[u8], serial: &mut Serial<USART2>) {
        match cmd {
            b\"help\" => {
                serial.write_str(\"Available commands:\\r\\n\").ok();
                serial.write_str(\"  help - Show this help\\r\\n\").ok();
                serial.write_str(\"  status - Show status\\r\\n\").ok();
            },
            b\"status\" => {
                serial.write_str(\"System OK\\r\\n\").ok();
            },
            _ => {
                serial.write_str(\"Unknown command\\r\\n\").ok();
            }
        }
    }
}
```

### Data Logging

```rust
use heapless::Vec;

struct DataLogger {
    buffer: Vec<u8, 256>,
}

impl DataLogger {
    fn log_measurement(&mut self, value: f32, serial: &mut Serial<USART2>) {
        // Format measurement
        use core::fmt::Write;
        let mut formatted = heapless::String::<32>::new();
        write!(formatted, \"{:.2},{}\\r\\n\", value, timestamp()).ok();
        
        // Send over serial
        serial.write_str(&formatted).ok();
        
        // Store in buffer if needed
        self.buffer.extend_from_slice(formatted.as_bytes()).ok();
    }
}
```

## Debugging Serial Issues

### Signal Verification

```rust
// Toggle TX pin to verify GPIO configuration
let mut tx_pin = gpioa.pa0.into_push_pull_output();

loop {
    tx_pin.set_high();
    delay.delay_ms(500);
    tx_pin.set_low(); 
    delay.delay_ms(500);
}
```

### Loopback Testing

```rust
// Connect TX to RX externally for loopback test
let test_data = b\"ABCDEF123456\";

for &byte in test_data {
    serial.write(byte).ok();
    
    // Should receive same byte back
    if let Ok(received) = serial.read() {
        if received != byte {
            // Loopback failed
            panic!(\"Loopback test failed\");
        }
    }
}
```

## Best Practices

1. **Always use appropriate baud rates** for your application
2. **Handle errors gracefully** - serial lines can be noisy
3. **Use DMA for high-throughput** applications
4. **Implement flow control** for reliable communication
5. **Test with oscilloscope** for timing verification
6. **Use proven pin configurations** from working examples
7. **Add timeout handling** to prevent hanging
8. **Clear errors promptly** to maintain communication

## Related Documentation

- [Serial Examples](../examples/serial.md) - Working code examples
- [Serial Echo Example](../examples/serial-echo.md) - Basic implementation
- [Serial ADC Example](../examples/serial-adc.md) - Advanced usage
- [GPIO Configuration](./gpio.md) - Pin setup details
- [DMA Documentation](./dma.md) - High-performance transfers
