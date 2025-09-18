# PY32F003 Device Guide

The PY32F003 is a low-cost ARM Cortex-M0+ microcontroller particularly well-suited for cost-sensitive applications. This guide covers device-specific features and configurations.

## Device Overview

### PY32F003 Family Variants

| Device | Flash | RAM | Package Options | Key Features |
|--------|-------|-----|-----------------|--------------|
| PY32F003x4 | 16KB | 2KB | DFN8, TSSOP20 | Basic peripherals |
| PY32F003x6 | 32KB | 4KB | TSSOP20, QFN32 | More memory |
| PY32F003x8 | 64KB | 8KB | TSSOP20, QFN32 | Maximum memory |

### Key Specifications
- **Core**: ARM Cortex-M0+ @ up to 24MHz
- **Voltage**: 1.7V to 5.5V operation
- **Temperature**: -40°C to +85°C (industrial)
- **Power**: Very low power consumption
- **Packages**: DFN8, TSSOP20, QFN32

## Peripheral Availability

### PY32F003 Peripheral Map

| Peripheral | F003x4 | F003x6 | F003x8 | Notes |
|------------|--------|--------|--------|-------|
| **GPIO** | 6-27 pins | 15-27 pins | 15-27 pins | Package dependent |
| **USART** | USART1,2 | USART1,2 | USART1,2 | 2 channels |
| **SPI** | SPI1 | SPI1 | SPI1 | 1 channel |  
| **I2C** | I2C1 | I2C1 | I2C1 | 1 channel |
| **ADC** | 1x12-bit | 1x12-bit | 1x12-bit | Up to 8 channels |
| **Timers** | TIM1,3,14,16,17 | TIM1,3,14,16,17 | TIM1,3,14,16,17 | Advanced + basic |
| **RTC** | Yes | Yes | Yes | 32.768kHz |
| **IWDG** | Yes | Yes | Yes | Independent watchdog |
| **WWDG** | Yes | Yes | Yes | Window watchdog |
| **CRC** | Yes | Yes | Yes | Hardware CRC |

## Package-Specific Information

### DFN8 Package (PY32F003I)

**Ultra-compact 2x3mm package** - ideal for space-constrained designs.


```
            DFN8 Pinout (Top View):
               VCC  1 ┌─────┐ 8  PB5/LED
               PA0  2 │     │ 7  PA14-SWDCK/PB6
               PA1  3 │     │ 6  PA13-SWDIO/PA10
               PA2  4 └─────┘ 5  PB0/PF2-NRST

```


**Available Pins:**
- **Power**: VCC, VSS
- **Programming**: PA13 (SWDIO), PA14 (SWDCK), NRST
- **GPIO**: PA0, PA1, PA2, PB5
- **USART2**: PA0 (TX), PA1 (RX)
- **ADC**: PA2 (Channel 2)

**Recommended Usage:**
```rust
// Verified working configuration
let tx = gpioa.pa0.into_alternate_af9();  // USART2_TX
let rx = gpioa.pa1.into_alternate_af9();  // USART2_RX
let adc_pin = gpioa.pa2.into_analog();    // ADC input
let led = gpiob.pb5.into_push_pull_output(); // Debug LED
```


**Additional Features:**
- More GPIO pins available
- Separate analog power (VDDA/VSSA)
- More peripheral pin options
- Better power supply decoupling

## Memory Layout

### Flash Memory Organization

```
PY32F003 Flash Layout:
┌─────────────────┐ 0x0800FFFF (64KB variant)
│   User Flash    │ 
│   (Application) │
├─────────────────┤ 0x08007FFF (32KB variant)  
│                 │
├─────────────────┤ 0x08003FFF (16KB variant)
│   System Flash  │ 0x08003000
│   (Bootloader)  │
└─────────────────┘ 0x08000000
```

### RAM Memory Map

```
PY32F003 RAM Layout:
┌─────────────────┐ 0x20001FFF (8KB variant)
│      SRAM       │
│   (User Data)   │ 
├─────────────────┤ 0x20000FFF (4KB variant)
│                 │
├─────────────────┤ 0x200007FF (2KB variant)
│                 │
└─────────────────┘ 0x20000000
```

## Clock Configuration

### Clock Sources

**Internal Clocks:**
- **HSI**: 8MHz internal RC oscillator (can be scaled to 24MHz)
- **LSI**: ~40kHz low-speed internal oscillator

**External Clocks (optional):**
- **HSE**: 4-32MHz external crystal/oscillator
- **LSE**: 32.768kHz external crystal (for RTC)

### Recommended Clock Setup

```rust
// Standard 24MHz configuration (proven stable)
let rcc = p.RCC
    .configure()
    .hsi(HSIFreq::Freq24mhz)  // Use internal 24MHz
    .sysclk(24.MHz())         // System clock = 24MHz
    .freeze(&mut p.FLASH);

// Alternative: Use external crystal
let rcc = p.RCC
    .configure()
    .hse(8.MHz())            // 8MHz external crystal
    .sysclk(24.MHz())        // PLL to 24MHz
    .freeze(&mut p.FLASH);
```

## Power Management

### Supply Requirements

- **VDD/VCC**: 1.7V to 5.5V (main supply)
- **VDDA**: Analog supply (same as VDD in most cases)  
- **VSS/VSSA**: Ground (0V)

### Power Consumption (Typical @ 3.3V)

| Mode | Current | Description |
|------|---------|-------------|
| **Run @ 24MHz** | ~3mA | Full speed operation |
| **Run @ 8MHz** | ~1.5mA | Reduced speed |
| **Sleep** | ~0.8mA | CPU stopped, peripherals active |
| **Stop** | ~2µA | Most peripherals stopped |
| **Standby** | ~1µA | Minimal power, RTC active |

### Low Power Programming

```rust
use py32f0xx_hal::pwr::{PowerMode, Pwr};

// Enter sleep mode
cortex_m::asm::wfi(); // Wait for interrupt

// Enter stop mode
let pwr = Pwr::new(p.PWR);
pwr.stop_mode();

// Configure wake-up sources
pwr.enable_wakeup_pin(WakeupPin::PA0);
```

## Development Tips

### DFN8-Specific Considerations

1. **Limited pins** - Plan pin usage carefully
2. **Single-layer PCB friendly** - Bottom pad can be difficult
3. **Hand soldering** - Requires fine-pitch soldering skills
4. **Debug access** - PA13 on bottom pad may need special connector

### Pin Multiplexing Strategy

```rust
// Example: Maximize utility of limited DFN8 pins
// PA0: USART2_TX (communication)
// PA1: USART2_RX (communication) 
// PA2: ADC input (sensing)
// PB5: GPIO output (LED/control)
// PA13: SWDIO (programming - bottom pad)
// PA14: SWDCK (programming)
```

### PCB Design Recommendations

**DFN8 Package:**
- **Thermal pad connection** to ground plane recommended
- **Decoupling**: 100nF close to VCC pin
- **Crystal placement**: Keep HSE crystal close if used
- **Debug connector**: Consider test points for PA13/PA14

**Power Supply:**
- **Linear regulator** for 3.3V from 5V if needed
- **Battery operation** possible with low-power modes
- **Brown-out detection** available in software

## Common Applications

### Sensor Nodes
```rust
// Temperature sensor with serial output
let temp_sensor = adc.convert(&adc_pin);
serial.write_str(&format!("Temp: {}°C\r\n", temp_to_celsius(temp_sensor)));
```

### Simple Controllers  
```rust
// Button-controlled LED
if button.is_high() {
    led.set_high();
} else {
    led.set_low();  
}
```

### Data Loggers
```rust
// Log sensor data periodically
rtc.set_alarm(60); // Every minute
loop {
    cortex_m::asm::wfi(); // Sleep until alarm
    let data = read_sensors();
    log_data(data);
}
```

## Debugging and Development

### SWD Programming
- **SWDIO**: PA13 (bottom pad on DFN8)
- **SWDCK**: PA14  
- **NRST**: Hardware reset (optional)
- **VCC/GND**: Power for programmer

### Serial Debugging
```rust
// Use USART2 for debug output
let mut debug_serial = Serial::usart2(
    p.USART2, 
    (tx, rx), 
    115200.bps(), 
    rcc.clocks
);

// Debug output
writeln!(debug_serial, "Debug: value = {}", value).ok();
```

## Migration Notes

### From Other STM32F0xx
- **Pin compatibility**: Check alternate functions  
- **Clock setup**: May need adjustment
- **Peripheral differences**: Some features may be missing

### Between PY32F003 Variants
- **Pin count differences**: DFN8 vs TSSOP20 vs QFN32
- **Memory sizes**: Check flash/RAM requirements
- **Package-specific features**: Some pins only on larger packages

## Next Steps

- **Try [Quick Start Guide](../getting-started/quick-start.md)** with PY32F003
- **Explore [Serial Examples](../examples/serial.md)** for communication
- **Learn [GPIO Control](../peripherals/gpio.md)** for I/O operations
- **Check [Hardware Setup](../getting-started/hardware-setup.md)** for wiring
