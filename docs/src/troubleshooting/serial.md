# Serial Communication Troubleshooting

This guide helps you diagnose and fix common serial communication issues with PY32F0xx microcontrollers.

## Quick Diagnostic Checklist

Before diving into detailed troubleshooting, check these common issues:

- [ ] **Wiring**: TX↔RX, RX↔TX, GND↔GND
- [ ] **Baud Rate**: 9600 bps on both sides  
- [ ] **Power**: 3.3V to microcontroller
- [ ] **Programming**: Device successfully flashed
- [ ] **Serial Adapter**: Working and recognized by system

## Common Problems and Solutions

### 1. No Serial Output at All

#### Symptoms
- No characters appear in serial terminal
- LED may or may not blink (depending on example)
- Terminal connects but shows no data

#### Diagnostic Steps

**Check Hardware Connections**
```
Verify wiring:
PY32F0xx     Serial Adapter
PA0 (TX)  →  RX
PA1 (RX)  ←  TX  
GND       →  GND
```

**Verify Serial Settings**
- Baud rate: 9600
- Data bits: 8
- Parity: None  
- Stop bits: 1
- Flow control: None

**Test Serial Adapter**
```bash
# Test adapter loopback (connect TX to RX on adapter)
echo \"test\" > /dev/ttyACM0
cat /dev/ttyACM0  # Should see \"test\"
```

#### Solutions

1. **Fix Wiring Issues**
   - Double-check TX/RX connections (they cross over)
   - Ensure good connections (no loose wires)
   - Verify GND connection

2. **Correct Pin Configuration**
   ```rust
   // Ensure you're using the right alternate function
   let tx = gpioa.pa0.into_alternate_af9(); // USART2 TX
   let rx = gpioa.pa1.into_alternate_af9(); // USART2 RX
   ```

3. **Try Different Baud Rate**
   ```rust
   // Try lower baud rate
   let mut serial = p.USART2.serial((tx, rx), 9_600.bps(), &rcc.clocks);
   ```

### 2. Garbled/Corrupted Characters

#### Symptoms
- Characters appear but are wrong/corrupted
- Random symbols instead of expected text
- Intermittent correct characters

#### Causes and Solutions

**Baud Rate Mismatch**
```bash
# Ensure both sides use same baud rate
# PY32F0xx side (in code):
let mut serial = p.USART2.serial((tx, rx), 9_600.bps(), &rcc.clocks);

# Terminal side:
screen /dev/ttyACM0 9600  # Must match!
```

**Clock Configuration Issues**
```rust
// Use proven 24MHz HSI configuration
let rcc = p.RCC
    .configure()
    .hsi(HSIFreq::Freq24mhz)
    .sysclk(24.MHz())
    .freeze(&mut p.FLASH);
```

**Power Supply Problems**
- Ensure stable 3.3V power supply
- Check for voltage drops under load
- Add decoupling capacitors (100nF)

### 3. Serial Works But Intermittently

#### Symptoms  
- Sometimes works, sometimes doesn't
- Works after reset but fails later
- Occasional correct characters

#### Solutions

1. **Check Power Stability**
   ```bash
   # Measure supply voltage with multimeter
   # Should be stable 3.3V ± 0.1V
   ```

2. **Verify Clock Stability**
   ```rust
   // Use internal HSI (more stable than HSE)
   let rcc = p.RCC
       .configure()  
       .hsi(HSIFreq::Freq24mhz)  // Internal oscillator
       .sysclk(24.MHz())
       .freeze(&mut p.FLASH);
   ```

3. **Add Error Handling**
   ```rust
   // Handle serial errors gracefully
   match serial.read() {
       Ok(byte) => {
           // Process byte
           serial.write(byte).ok();
       },
       Err(_) => {
           // Handle error - maybe reset USART
       }
   }
   ```

### 4. Device Not Responding

#### Symptoms
- Serial terminal connects but no startup message
- No response to any input
- LED may still blink

#### Diagnostic Steps

1. **Verify Programming**
   ```bash
   # Reflash the device
   make flash EXAMPLE=serial_echo MCU_TYPE=py32f003xx4
   
   # Check flash was successful (no errors in output)
   ```

2. **Test with Different Example**
   ```bash
   # Flash and test blinky first
   make flash EXAMPLE=blinky MCU_TYPE=py32f003xx4
   # LED should blink - confirms basic functionality
   ```

3. **Check Reset**
   ```bash
   # Try hardware reset
   # Briefly pull NRST low or cycle power
   ```

#### Solutions

1. **Reflash Firmware**
2. **Check SWD Programming Connections**
3. **Verify Device is Running** (check LED blinks)

### 5. Wrong USART Configuration

#### Common USART/Pin Combinations

**For PY32F003I DFN8:**
```rust
// Working configuration (tested):
// USART2 with AF9
let tx = gpioa.pa0.into_alternate_af9(); // Pin 3 → TX
let rx = gpioa.pa1.into_alternate_af9(); // Pin 6 → RX
let mut serial = p.USART2.serial((tx, rx), 9_600.bps(), &rcc.clocks);
```

**Alternative USART1 configuration:**
```rust
// USART1 with AF1 (different pins)
let tx = gpioa.pa9.into_alternate_af1();  
let rx = gpioa.pa10.into_alternate_af1();
let mut serial = p.USART1.serial((tx, rx), 9_600.bps(), &rcc.clocks);
```

## Advanced Diagnostics

### Logic Analyzer Testing

If you have a logic analyzer:

1. **Capture TX Line**
   - Set trigger on PA0 (TX)
   - Look for correct 9600 baud timing
   - Verify start bit, data bits, stop bit

2. **Check Clock Signals**
   - Verify system clock is 24MHz
   - Check USART clock enabling

### Oscilloscope Testing

1. **Measure TX Signal**
   - Should be 3.3V idle (high)
   - Should go to 0V for start bits
   - Timing: ~104µs per bit at 9600 baud

2. **Check Power Rails**
   - Stable 3.3V on VDD
   - No significant ripple or noise

## Testing Tools and Scripts

### Python Test Script

```python
#!/usr/bin/env python3
import serial
import time
import sys

def test_serial(port='/dev/ttyACM0', baud=9600):
    try:
        ser = serial.Serial(port, baud, timeout=1)
        print(f\"Connected to {port} at {baud} baud\")
        
        # Test echo
        test_msg = \"Hello PY32F0xx!\"
        ser.write(test_msg.encode())
        time.sleep(0.1)
        
        response = ser.read(len(test_msg))
        print(f\"Sent: '{test_msg}'\")
        print(f\"Received: '{response.decode()}'\")
        
        if response.decode() == test_msg:
            print(\"✅ Echo test PASSED\")
        else:
            print(\"❌ Echo test FAILED\")
            
        ser.close()
        
    except Exception as e:
        print(f\"Error: {e}\")

if __name__ == \"__main__\":
    port = sys.argv[1] if len(sys.argv) > 1 else '/dev/ttyACM0'
    test_serial(port)
```

### Manual Terminal Testing

```bash
# Test with different terminals
screen /dev/ttyACM0 9600
# or
minicom -D /dev/ttyACM0 -b 9600  
# or
picocom -b 9600 /dev/ttyACM0
```

## Device-Specific Issues

### PY32F003I DFN8

**Pin Limitations:**
- Only 8 pins available
- PA0/PA1 are best choice for USART2
- PB5 available for debug LED

**Known Working Configuration:**
```rust
// Verified working on real hardware
let tx = gpioa.pa0.into_alternate_af9();
let rx = gpioa.pa1.into_alternate_af9(); 
let mut serial = p.USART2.serial((tx, rx), 9_600.bps(), &rcc.clocks);
```

### PY32F030 TSSOP20

**More Pin Options:**
- Multiple USART pin combinations available
- Can use USART1 or USART2
- More GPIO pins for debugging

## Getting Help

If none of these solutions work:

1. **Check [Examples Documentation](../examples/serial-echo.md)** for working code
2. **Review [Hardware Setup](../getting-started/hardware-setup.md)** guide  
3. **Compare with [Working Configuration](../examples/serial.md)**
4. **Open GitHub Issue** with:
   - Hardware details (device, package, programmer)
   - Code that's not working
   - Error messages
   - What you've already tried

## Prevention Tips

To avoid serial communication issues:

1. **Start with working examples** before modifying
2. **Use proven clock configurations** (24MHz HSI)
3. **Verify hardware before coding** (power, connections)
4. **Test serial adapter separately** before connecting MCU
5. **Use consistent naming** for TX/RX pins to avoid confusion
6. **Add error handling** in production code
7. **Document your pin choices** for future reference
