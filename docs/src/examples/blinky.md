# Blinky LED Example

The classic \"Hello World\" of embedded programming - blinking an LED. This example demonstrates basic GPIO output control and timing.

## What It Does

This example:
1. Configures the system clock to 24MHz
2. Sets up a GPIO pin as output (typically PB5)
3. Blinks an LED on and off every 500ms
4. Runs indefinitely

## Hardware Requirements

### PY32F003I DFN8 Package
- **PB5** (Pin 1): LED output
- **GND** (Pin 2): Ground connection
- **VCC** (Pin 8): 3.3V power

### LED Circuit
```
PB5 → [330Ω resistor] → LED Anode
LED Cathode → GND
```

**Component Values:**
- LED: Standard 3mm or 5mm LED (any color)
- Resistor: 330Ω (current limiting)
- Current: ~7mA @ 3.3V

## Code Example

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
    // Get hardware peripherals
    let mut p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure 24MHz system clock
    let rcc = p.RCC
        .configure()
        .hsi(HSIFreq::Freq24mhz)
        .sysclk(24.MHz())
        .freeze(&mut p.FLASH);

    // Setup GPIO
    let gpiob = p.GPIOB.split();
    let mut led = gpiob.pb5.into_push_pull_output();

    // Create delay timer
    let mut delay = Delay::new(cp.SYST, &rcc.clocks);

    // Blink forever
    loop {
        led.set_high();
        delay.delay_ms(500_u16);
        led.set_low();
        delay.delay_ms(500_u16);
    }
}
```

## Building and Running

### Using Make

```bash
# Build the example
make build EXAMPLE=blinky MCU_TYPE=py32f003xx4

# Flash to device
make flash EXAMPLE=blinky MCU_TYPE=py32f003xx4
```

### Using Cargo

```bash
# Build for PY32F003
cargo build --release --example blinky --features py32f003xx4

# Flash with PyOCD
pyocd flash target/thumbv6m-none-eabi/release/examples/blinky --target py32f003xx4
```

## Expected Behavior

After flashing successfully:
- LED should turn ON for 500ms
- LED should turn OFF for 500ms  
- Pattern repeats indefinitely
- Total cycle time: 1 second

## Code Walkthrough

### System Initialization

```rust
let mut p = pac::Peripherals::take().unwrap();
let cp = cortex_m::Peripherals::take().unwrap();
```
- `pac::Peripherals` provides access to all microcontroller peripherals
- `cortex_m::Peripherals` provides access to ARM Cortex-M core peripherals
- `take()` ensures singleton access (only one instance)

### Clock Configuration

```rust
let rcc = p.RCC
    .configure()
    .hsi(HSIFreq::Freq24mhz)  // Internal 24MHz oscillator
    .sysclk(24.MHz())         // System clock = 24MHz
    .freeze(&mut p.FLASH);    // Apply and lock configuration
```

**Why 24MHz?**
- Stable and reliable frequency
- Good balance between performance and power
- Well-tested configuration
- Compatible with common baud rates

### GPIO Setup

```rust
let gpiob = p.GPIOB.split();
let mut led = gpiob.pb5.into_push_pull_output();
```

- `split()` converts raw GPIO peripheral to HAL-managed pins
- `into_push_pull_output()` configures pin as:
  - **Output direction** (not input)
  - **Push-pull mode** (can drive high and low)
  - **Default low** (LED starts off)

### Timing

```rust
let mut delay = Delay::new(cp.SYST, &rcc.clocks);
```
- Uses ARM SysTick timer for precise delays
- Automatically calibrated to system clock frequency
- `delay_ms()` provides millisecond-accurate delays

### Main Loop

```rust
loop {
    led.set_high();           // LED ON (3.3V output)
    delay.delay_ms(500_u16);  // Wait 500ms
    led.set_low();            // LED OFF (0V output)
    delay.delay_ms(500_u16);  // Wait 500ms
}
```

## Customizations

### Change Blink Pattern

**Fast Blink:**
```rust
led.set_high();
delay.delay_ms(100_u16);
led.set_low();
delay.delay_ms(100_u16);
```

**Slow Blink:**
```rust
led.set_high();
delay.delay_ms(2000_u16);  // 2 seconds
led.set_low();
delay.delay_ms(2000_u16);
```

**Heartbeat Pattern:**
```rust
// Quick double-blink, then pause
for _ in 0..2 {
    led.set_high();
    delay.delay_ms(100_u16);
    led.set_low();
    delay.delay_ms(100_u16);
}
delay.delay_ms(800_u16);  // Long pause
```

### Different LED Pins

**Use PA2 (DFN8 Pin 7):**
```rust
let gpioa = p.GPIOA.split();
let mut led = gpioa.pa2.into_push_pull_output();
```

**Multiple LEDs:**
```rust
let mut led1 = gpiob.pb5.into_push_pull_output();
let mut led2 = gpioa.pa2.into_push_pull_output();

loop {
    // Alternating blink
    led1.set_high();
    led2.set_low();
    delay.delay_ms(250_u16);
    
    led1.set_low();
    led2.set_high();
    delay.delay_ms(250_u16);
}
```

### Toggle Method

```rust
// More efficient toggling
loop {
    led.toggle();
    delay.delay_ms(500_u16);
}
```

## Troubleshooting

### LED Not Blinking

**Check Hardware:**
1. **LED polarity** - Long leg (anode) to resistor, short leg (cathode) to GND
2. **Resistor value** - Use 330Ω to 1kΩ 
3. **Connections** - Ensure solid connections
4. **Power supply** - Verify 3.3V on VCC

**Check Software:**
1. **Correct pin** - Verify PB5 for DFN8 package
2. **Successful flash** - Look for \"Programming completed\" message
3. **Device running** - Try different delay values

### LED Always On/Off

**Always On:**
```rust
// Check if set_low() is being called
led.set_low();   // Should turn LED off
led.set_high();  // Should turn LED on
```

**Always Off:**
- Check LED polarity (try reversing)
- Verify power supply voltage
- Test with multimeter on PB5 pin

### Build Errors

**Missing target:**
```bash
rustup target add thumbv6m-none-eabi
```

**Wrong feature:**
```bash
# Use correct device feature
cargo build --example blinky --features py32f030xx4  # For PY32F030
cargo build --example blinky --features py32f003xx4  # For PY32F003
```

### Flash Errors

**Device not found:**
```bash
# Check SWD connections
pyocd list

# Should show your programmer
```

**Programming failed:**
```bash
# Try erasing first
pyocd erase --chip --target py32f003xx4
make flash EXAMPLE=blinky MCU_TYPE=py32f003xx4
```

## Advanced Variations

### PWM Breathing LED

```rust
use py32f0xx_hal::pwm::*;

// Setup PWM on timer
let pwm = p.TIM1.pwm(
    gpiob.pb5.into_alternate_af2(),
    1.kHz(),
    &rcc.clocks
);

let mut pwm_ch = pwm.split();

// Breathing effect
loop {
    // Fade in
    for duty in 0..100 {
        pwm_ch.set_duty_cycle_percent(duty);
        delay.delay_ms(10_u16);
    }
    
    // Fade out
    for duty in (0..100).rev() {
        pwm_ch.set_duty_cycle_percent(duty);
        delay.delay_ms(10_u16);
    }
}
```

### Interrupt-Driven Blink

```rust
use py32f0xx_hal::timer::{Event, Timer};
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;

// Global LED reference
static LED: Mutex<RefCell<Option<gpio::gpiob::PB5<gpio::Output<gpio::PushPull>>>>> = 
    Mutex::new(RefCell::new(None));

#[entry] 
fn main() -> ! {
    // Setup timer interrupt
    let mut timer = Timer::tim1(p.TIM1, 1.Hz(), &rcc.clocks);
    timer.listen(Event::TimeOut);
    
    // Store LED globally
    cortex_m::interrupt::free(|cs| {
        LED.borrow(cs).replace(Some(led));
    });
    
    // Enable timer interrupt
    unsafe { cortex_m::peripheral::NVIC::unmask(pac::Interrupt::TIM1_UP_TIM16) };
    
    loop {
        cortex_m::asm::wfi(); // Sleep until interrupt
    }
}

#[interrupt]
fn TIM1_UP_TIM16() {
    // Toggle LED in interrupt
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut led) = LED.borrow(cs).borrow_mut().as_mut() {
            led.toggle();
        }
    });
}
```

## Related Examples

- **[Serial Echo](./serial-echo.md)** - Add serial communication to LED control
- **[PWM Examples](./pwm.md)** - Generate PWM signals for LED brightness control
- **[Timer Examples](./timers.md)** - Advanced timing and interrupts

## Next Steps

Once you have blinky working:

1. **Try [Your First Program](../getting-started/first-program.md)** to understand the code better
2. **Explore [GPIO Documentation](../peripherals/gpio.md)** for advanced pin control  
3. **Add [Serial Communication](./serial-echo.md)** for debugging output
4. **Learn [PWM Control](./pwm.md)** for variable brightness

The humble blinky example is the foundation for all embedded development - once you master GPIO control, you can interface with any digital device!
