# Your First Program

This tutorial will guide you through creating and running your first PY32F0xx program using the HAL.

## Goal

By the end of this tutorial, you'll have:
- Created a simple LED blinking program
- Built and flashed it to your PY32F0xx device
- Understood the basic structure of a PY32F0xx Rust program

## Prerequisites

- [Development environment](./development-environment.md) is set up
- [Hardware is connected](./hardware-setup.md) and working
- PY32F0xx device with at least one LED connection

## Step 1: Understanding the Basic Structure

Every PY32F0xx Rust program follows this basic pattern:

```rust
#![no_main]     // We don't use Rust's standard main
#![no_std]      // No standard library (embedded environment)

use panic_halt as _;  // Panic handler for embedded

use py32f0xx_hal as hal;  // Import HAL
use crate::hal::{
    pac,           // Peripheral Access Crate
    prelude::*,    // Common traits and imports
    rcc::HSIFreq,  // Clock configuration
};

use cortex_m_rt::entry;  // Entry point macro

#[entry]  // This is our \"main\" function
fn main() -> ! {
    // Hardware setup code goes here
    
    loop {
        // Main program loop
    }
}
```

## Step 2: Create Your First Program

Let's create a simple LED blinky program. Create `examples/my_first_program.rs`:

```rust
#![no_main]
#![no_std]

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    rcc::HSIFreq,
    timer::delay::Delay,
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Take ownership of device peripherals
    let mut p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the system clock
    let rcc = p.RCC
        .configure()
        .hsi(HSIFreq::Freq24mhz)  // Use 24MHz internal oscillator
        .sysclk(24.MHz())         // Set system clock to 24MHz
        .freeze(&mut p.FLASH);    // Apply clock configuration

    // Initialize GPIO port B
    let gpiob = p.GPIOB.split();

    // Configure PB5 as push-pull output (LED pin)
    let mut led = gpiob.pb5.into_push_pull_output();

    // Create a delay timer using the system timer
    let mut delay = Delay::new(cp.SYST, &rcc.clocks);

    // Main loop - blink the LED
    loop {
        led.set_high();           // Turn LED on
        delay.delay_ms(500_u16);  // Wait 500ms
        led.set_low();            // Turn LED off
        delay.delay_ms(500_u16);  // Wait 500ms
    }
}
```

## Step 3: Build Your Program

```bash
# Build your program
make build EXAMPLE=my_first_program MCU_TYPE=py32f003xx4

# If successful, you should see:
# Compiling my_first_program v0.1.0
# Finished release [optimized] target(s)
```

## Step 4: Flash to Device

```bash
# Connect your PY32F0xx device via SWD
# Then flash the program
make flash EXAMPLE=my_first_program MCU_TYPE=py32f003xx4

# You should see PyOCD output:
# 0001735:INFO:board:Target type is py32f003xx4
# 0001736:INFO:flash_loader:Erasing chip...
# 0001750:INFO:flash_loader:Programming...
# 0001755:INFO:flash_loader:Programming completed
```

## Step 5: Verify It Works

After successful flashing:
1. **LED should start blinking** - On for 500ms, off for 500ms
2. **If using PB5** - Connect LED with 330Ω resistor to GND
3. **No errors in terminal** - Clean flash output indicates success

## Understanding the Code

### Memory Management

```rust
#![no_main]  // No standard main function
#![no_std]   // No heap allocation, stack-based only

use panic_halt as _;  // Simple panic handler - halts on panic
```

### Hardware Abstraction

```rust
let mut p = pac::Peripherals::take().unwrap();
// pac::Peripherals gives access to all hardware peripherals
// take() ensures only one instance exists (singleton pattern)
```

### Clock Configuration

```rust
let rcc = p.RCC
    .configure()
    .hsi(HSIFreq::Freq24mhz)  // Internal 24MHz oscillator
    .sysclk(24.MHz())         // System clock frequency
    .freeze(&mut p.FLASH);    // Lock in configuration
```

**Why 24MHz?** This is a proven, stable configuration that works reliably across PY32F0xx devices.

### GPIO Setup

```rust
let gpiob = p.GPIOB.split();
// split() converts raw peripheral to HAL-managed GPIO port

let mut led = gpiob.pb5.into_push_pull_output();
// Configure specific pin as output
// push_pull = can source and sink current
```

### Timing

```rust
let mut delay = Delay::new(cp.SYST, &rcc.clocks);
// Uses ARM Cortex-M SysTick timer for delays
// Calibrated to the system clock frequency
```

## Customizing Your Program

### Change Blink Rate

```rust
// Faster blinking
led.set_high();
delay.delay_ms(100_u16);  // 100ms on
led.set_low();
delay.delay_ms(100_u16);  // 100ms off

// Slower blinking  
led.set_high();
delay.delay_ms(1000_u16);  // 1 second on
led.set_low();
delay.delay_ms(1000_u16);  // 1 second off
```

### Use Different LED Pin

```rust
// For PY32F003 DFN8 package - use PA2 instead
let gpioa = p.GPIOA.split();
let mut led = gpioa.pa2.into_push_pull_output();
```

### Add Multiple LEDs

```rust
let mut led1 = gpiob.pb5.into_push_pull_output();
let mut led2 = gpioa.pa2.into_push_pull_output();

loop {
    led1.set_high();
    led2.set_low();
    delay.delay_ms(250_u16);
    
    led1.set_low();  
    led2.set_high();
    delay.delay_ms(250_u16);
}
```

## Common Issues and Solutions

### LED Not Blinking

1. **Check wiring**:
   ```
   PB5 → [330Ω resistor] → LED anode
   LED cathode → GND
   ```

2. **Verify pin assignment**:
   - PY32F003I DFN8: Use PB5 (pin 1)
   - Check your device pinout

3. **Check power supply**:
   - Should be stable 3.3V
   - Measure with multimeter

### Build Errors

**\"error: target 'thumbv6m-none-eabi' not found\"**
```bash
rustup target add thumbv6m-none-eabi
```

**\"feature 'py32f003xx4' not found\"**
```bash
# Use correct feature for your device:
make build EXAMPLE=my_first_program MCU_TYPE=py32f030xx4  # for PY32F030
```

### Flash Errors

**\"No devices found\"**
```bash
# Check SWD connections
pyocd list  # Should show your programmer

# Verify wiring:
# SWDIO ↔ PA13
# SWDCK ↔ PA14  
# GND ↔ GND
```

**\"Flash failed\"**
```bash
# Try different SWD frequency
pyocd flash --frequency 1000000 target/thumbv6m-none-eabi/release/examples/my_first_program

# Or try mass erase first
pyocd erase --chip --target py32f003xx4
```

## Next Steps

Congratulations! You've successfully created, built, and flashed your first PY32F0xx program. Here's what to explore next:

### Learn More Peripherals

- **[Serial Communication](../examples/serial-echo.md)** - USART communication
- **[ADC Reading](../examples/serial-adc.md)** - Analog input measurement  
- **[PWM Output](../examples/pwm.md)** - Generate PWM signals
- **[Timers](../peripherals/timers.md)** - Precise timing control

### Explore Examples

```bash
# Try the serial echo example
make flash EXAMPLE=serial_echo MCU_TYPE=py32f003xx4

# Or the ADC example
make flash EXAMPLE=serial_adc MCU_TYPE=py32f003xx4
```

### Build Real Projects

- **Temperature monitor** with serial output
- **PWM motor controller**  
- **Data logger** with external sensors
- **Simple IoT device** with serial interface

### Advanced Topics

- **[DMA transfers](../peripherals/dma.md)** for high-performance I/O
- **[Interrupts](../peripherals/gpio.md)** for responsive systems
- **[Low power modes](../peripherals/rtc.md)** for battery applications
- **[Custom bootloaders](../contributing/guidelines.md)** for field updates

## Reference Links

- [HAL API Documentation](https://docs.rs/py32f0xx-hal/)
- [More Examples](../examples/blinky.md)  
- [Troubleshooting Guide](../troubleshooting/build.md)
- [Hardware Setup](./hardware-setup.md)

You're now ready to start building more complex PY32F0xx applications!
