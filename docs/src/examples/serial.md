# Serial Communication Examples

This section covers serial communication examples that have been tested and verified on PY32F0xx hardware.

## Overview

The PY32F0xx HAL includes two comprehensive serial communication examples:

1. **[Serial Echo](./serial-echo.md)** - Basic USART communication
2. **[Serial ADC](./serial-adc.md)** - Advanced serial + ADC integration

Both examples are confirmed working on **PY32F003I DFN8** package at 9600 bps.

## Key Features

### Verified Configuration
- **USART2**: PA0=TX(AF9), PA1=RX(AF9)
- **Baud Rate**: 9600 bps
- **Clock**: 24MHz HSI (reliable configuration)
- **Debug LED**: PB5 for visual feedback

### Hardware Tested
- **Device**: PY32F003I DFN8 package
- **Programmer**: PyOCD with ST-Link compatible
- **Serial**: USB-to-Serial adapters (multiple tested)

## Quick Start

### Build and Flash

```bash
# Serial echo example
make flash EXAMPLE=serial_echo MCU_TYPE=py32f003xx4

# Serial ADC example
make flash EXAMPLE=serial_adc MCU_TYPE=py32f003xx4
```

### Connect Serial Terminal

```bash
# Using screen
screen /dev/ttyACM0 9600

# Using minicom  
minicom -D /dev/ttyACM0 -b 9600

```

## Pin Configuration

### PY32F003I DFN8 Package



```
            DFN8 Pinout (Top View):
               VCC  1 ┌─────┐ 8  PB5/LED
               PA0  2 │     │ 7  PA14-SWDCK/PB6
               PA1  3 │     │ 6  PA13-SWDIO/PA10
               PA2  4 └─────┘ 5  PB0/PF2-NRST

```


### Connections

| Pin | Function | Connection |
|-----|----------|------------|
| PA0 | USART2 TX (AF9) | Serial adapter RX |
| PA1 | USART2 RX (AF9) | Serial adapter TX |
| PA2 | ADC Channel 2   | Analog input (0-3.3V) |
| PB5 | GPIO Output     | LED + 330Ω resistor |
| PA13| SWDIO          | SWD programmer |
| PA14| SWDCK          | SWD programmer |

## Technical Implementation

### Clock Configuration

Both examples use the same reliable clock setup:

```rust
let rcc = p.RCC
    .configure()
    .hsi(HSIFreq::Freq24mhz)  // 24MHz HSI
    .sysclk(24.MHz())         // System clock
    .freeze(&mut p.FLASH);
```

### USART Setup

The key to success is using AF9 alternate function:

```rust
let tx = gpioa.pa0.into_alternate_af9(); // TX
let rx = gpioa.pa1.into_alternate_af9(); // RX

let serial = Serial::usart2(
    p.USART2,
    (tx, rx),
    9600.bps(),
    clocks,
);
```

### Debug LED

Visual feedback with PB5:

```rust
let mut led = gpiob.pb5.into_push_pull_output();
led.set_high().ok(); // LED on for activity
```

## Example Comparison

| Feature | Serial Echo | Serial ADC |
|---------|-------------|------------|
| **Complexity** | Basic | Advanced |
| **USART** | ✅ Echo functionality | ✅ Command interface |
| **ADC** | ❌ Not used | ✅ PA2 analog input |
| **Commands** | ❌ Simple echo | ✅ Interactive (r/s/q/h) |
| **Streaming** | ❌ No | ✅ Continuous ADC |
| **Best for** | Learning basics | Real applications |

## Testing Tools

### Python Test Script

You can quickly verify serial communication using a Python script, for example, `test_serial.py`:


```python
import serial
import time

# Connect to device
ser = serial.Serial('/dev/ttyACM0', 9600, timeout=1)

# Test echo
ser.write(b'Hello PY32F0xx!\n')
response = ser.read(50)
print(f"Response: {response}")

# For ADC example
ser.write(b'h')  # Get help
ser.write(b'r')  # Read ADC
ser.write(b's')  # Start streaming
time.sleep(5)
ser.write(b'q')  # Stop streaming
```

### Manual Testing

```bash
# Connect to device
screen /dev/ttyACM0 9600

# For serial_echo.rs:
# Type any characters, they should echo back

# For serial_adc.rs:
# Type 'h' for help
# Type 'r' to read ADC once  
# Type 's' to start streaming
# Type 'q' to stop streaming
```

## Troubleshooting

### No Serial Output

1. **Check connections**:
   - PA0 (TX) → Serial adapter RX
   - PA1 (RX) → Serial adapter TX  
   - GND → GND

2. **Verify settings**:
   - Baud rate: 9600
   - Data bits: 8
   - Parity: None
   - Stop bits: 1

3. **Test adapter**:
   - Try different USB-to-Serial adapter
   - Check adapter drivers

### LED Not Working

1. **Check PB5 connection**
2. **Verify LED polarity**
3. **Use appropriate current-limiting resistor (330Ω)**

### ADC Issues

1. **Input range**: Ensure 0-3.3V on PA2
2. **Reference**: ADC uses VDD as reference
3. **Test with known voltage** (e.g., 1.5V battery)

## Next Steps

Once you have serial communication working:

1. **Explore [USART Peripheral](../peripherals/usart.md)** documentation
2. **Learn [ADC Configuration](../peripherals/adc.md)** details
3. **Try [Custom Applications](../examples/blinky.md)** using serial
4. **Check [Troubleshooting](../troubleshooting/serial.md)** for advanced issues

## Related Documentation

- [USART2 Configuration Guide](./usart2-config.md)
- [Serial Troubleshooting](../troubleshooting/serial.md)
- [PY32F003 Device Guide](../devices/py32f003-guide.md)
- [DFN8 Package Guide](../devices/dfn8-guide.md)
