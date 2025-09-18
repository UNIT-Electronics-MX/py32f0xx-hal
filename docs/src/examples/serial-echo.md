# Serial Echo Example

The `serial_echo.rs` example demonstrates basic USART2 serial communication on the PY32F0xx microcontroller. This is the simplest serial example and a great starting point for learning.

## What It Does

This example:
1. Configures USART2 on PA0 (TX) and PA1 (RX) at 9600 bps
2. Sets up a debug LED on PB5 for visual feedback
3. Echoes back any characters received over serial
4. Provides a simple interactive serial interface

## Hardware Requirements

### PY32F003I DFN8 Package
- **PA0**: USART2 TX (AF9) → Connect to serial adapter RX
- **PA1**: USART2 RX (AF9) → Connect to serial adapter TX  
- **PB5**: Debug LED → Connect LED + 330Ω resistor to GND
- **GND**: Ground → Connect to serial adapter GND

## Code Walkthrough

### Clock Configuration

The example uses the same reliable clock setup as the working blinky example:

```rust
let rcc = p.RCC
    .configure()
    .hsi(HSIFreq::Freq24mhz)  // Set HSI to 24MHz 
    .sysclk(24.MHz())         // Set system clock to 24MHz
    .freeze(&mut p.FLASH);
```

### GPIO Setup

Configure pins for USART2 with AF9 alternate function:

```rust
// Split GPIO ports
let gpioa = p.GPIOA.split();
let gpiob = p.GPIOB.split();

// Configure USART2 pins with AF9
let tx = gpioa.pa0.into_alternate_af9(); // PA0 as TX
let rx = gpioa.pa1.into_alternate_af9(); // PA1 as RX

// Debug LED
let mut debug_pin = gpiob.pb5.into_push_pull_output();
```

### Serial Interface

Create the USART2 serial interface:

```rust
let mut serial = p.USART2.serial((tx, rx), 9_600.bps(), &rcc.clocks);

// Send startup message
serial.write_str("=== USART2 PA0/PA1 AF9 WORKING - 9600 bps ===\\r\\n").ok();
```

### Main Echo Loop

The main loop continuously reads and echoes characters:

```rust
loop {
    // Try to read a character
    if let Ok(byte) = serial.read() {
        // Toggle debug LED on activity
        debug_pin.toggle();
        
        // Echo character back (with handling for special characters)
        match byte {
            b'\\r' => {
                serial.write(b'\\r').ok();
                serial.write(b'\\n').ok();
            },
            _ => {
                serial.write(byte).ok();
            }
        }
    }
}
```

## Building and Flashing

### Using Make

```bash
# Build the example
make build EXAMPLE=serial_echo MCU_TYPE=py32f003xx4

```bash
# Simple way
make flash-serial_echo

# Traditional way  
make flash EXAMPLE=serial_echo MCU_TYPE=PY32F003x4
```

### Using Cargo Directly

```bash
# Build for PY32F003
cargo build --release --example serial_echo --features py32f003xx4

# Flash with PyOCD
pyocd flash target/thumbv6m-none-eabi/release/examples/serial_echo --target py32f003xx4
```

## Testing the Example

### 1. Connect Hardware

Wire your USB-to-Serial adapter:
- **Adapter RX** → **PA0** (PY32F0xx TX)
- **Adapter TX** → **PA1** (PY32F0xx RX)  
- **Adapter GND** → **PY32F0xx GND**

### 2. Open Serial Terminal

```bash
# Using screen
screen /dev/ttyACM0 9600

# Using minicom
minicom -D /dev/ttyACM0 -b 9600

# Using PuTTY (Windows)
# Set COM port, 9600 baud, 8-N-1
```

### 3. Expected Output

Upon connecting, you should see:
```
=== USART2 PA0/PA1 AF9 WORKING - 9600 bps ===
PA0: TX (AF9) - Register configured
PA1: RX (AF9) - Register configured
PB5: Debug LED - Activity indicator
Ready to echo characters...
Type any character:
```

### 4. Test Functionality

- **Type any character** → Should echo back immediately
- **PB5 LED** → Should toggle with each character
- **Enter key** → Should produce proper line endings

## Expected Behavior

| Input | Output | LED |
|-------|--------|-----|
| `H` | `H` | Toggle |
| `Hello` | `Hello` | Toggle for each char |
| Enter | New line | Toggle |
| Any ASCII | Same character | Toggle |

## Troubleshooting

### No Serial Output

1. **Check wiring**:
   ```
   PY32F0xx    Adapter
   PA0 (TX) → RX
   PA1 (RX) ← TX
   GND      → GND
   ```

2. **Verify serial settings**: 9600-8-N-1

3. **Test serial adapter** with loopback (TX→RX)

### LED Not Working

1. **Check PB5 connection**
2. **Verify LED polarity** (anode to PB5, cathode via resistor to GND)
3. **Use 330Ω current-limiting resistor**

### Characters Corrupted

1. **Check baud rate** (must be 9600)
2. **Verify clock configuration** (24MHz HSI)
3. **Test different serial adapter**

### No Response

1. **Verify device is flashed** and running
2. **Check SWD connections** for programming
3. **Try reset** (cycle power or use NRST)

## Code Customization

### Change Baud Rate

```rust
// Change from 9600 to 115200
let mut serial = p.USART2.serial((tx, rx), 115_200.bps(), &rcc.clocks);
```

### Add More Functionality

```rust
// Process specific commands
match byte {
    b'h' | b'H' => {
        serial.write_str("Help: Type any character to echo\\r\\n").ok();
    },
    b'\\r' => {
        serial.write_str("\\r\\n").ok();
    },
    _ => {
        serial.write(byte).ok();
    }
}
```

### Use Different Pins

```rust
// Use different USART pins (check datasheet for AF mappings)
let tx = gpioa.pa9.into_alternate_af1();  // USART1 TX
let rx = gpioa.pa10.into_alternate_af1(); // USART1 RX
let mut serial = p.USART1.serial((tx, rx), 9_600.bps(), &rcc.clocks);
```

## Next Steps

Once you have the echo example working:

1. **Try [Serial ADC Example](./serial-adc.md)** for more advanced functionality
2. **Learn about [USART Peripheral](../peripherals/usart.md)** details
3. **Explore [GPIO Configuration](../peripherals/gpio.md)** for other pins
4. **Build custom applications** using serial communication

## Related Examples

- **[Serial ADC](./serial-adc.md)** - Combines serial with ADC readings
- **[Blinky LED](./blinky.md)** - Basic GPIO output example  
- **[USART Configuration](./usart2-config.md)** - Advanced USART setup
