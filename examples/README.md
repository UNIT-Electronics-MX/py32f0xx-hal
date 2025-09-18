# 📁 PY32F0xx-HAL Project Examples

## 🚀 Main Examples (Use These)

### **Basic**
- **`blinky_hal_simple.rs`** - Basic blinking LED example with automatic initialization
- **`gpio_easy_swap.rs`** - Easy GPIO port swapping demonstration  
- **`gpio_multi_port_demo.rs`** - Advanced demo with multiple ports at once

### **Original HAL Examples**
- **`blinky.rs`** - Original blinky example from the project
- **`blinky_delay.rs`** - Blinky with precise delays
- **`blinky_timer.rs`** - Blinky using timers

### **Peripherals**
- **`adc_values.rs`** - Reading ADC values
- **`serial_echo.rs`** - Serial echo
- **`i2c_slave_demo.rs`** - I2C slave communication with serial debug
- **`i2c_master/`** - I2C master test scripts for ESP32-H2
- **`spi_hal_apa102c.rs`** - APA102C LED control via SPI
- **`pwm.rs`** - PWM generation

## 🧪 Test and Development Files (`testing/`)

The following files are in `examples/testing/` to keep the main directory clean:

- `blinky_pa1.rs` - Specific test for PA1
- `blinky_working.rs` - Working version during development  
- `clock_gpio_test.rs` - Clock configuration tests
- `diagnostic_direct.rs` - Direct hardware diagnostics
- `direct_gpio_*.rs` - Direct GPIO register manipulation
- `gpio_test_*.rs` - Various GPIO tests
- `led_on.rs` - Simple LED on test
- `test_multiple_pins.rs` - Tests with multiple pins

## 🛠️ Generalized Initialization System

All new examples use the automatic initialization system:

```rust
use py32f0xx_hal::system_init::SystemInit;

// One line initializes the whole system
let sys = SystemInit::init();

// Swapping ports is easy - just change this line:
let mut pin = sys.gpiob.pb5.into_push_pull_output();
```

## 📋 Useful Commands

```bash
# Build example
make build EXAMPLE=blinky_hal_simple

# Build and flash (with automatic reset)  
make flash EXAMPLE=gpio_easy_swap

# Software reset only
make reset

# Clean build files
make clean

# View current configuration
make info
```

## 🎯 Automatic Configuration Included

✅ System clock (HSI 8MHz by default, 24MHz available)  
✅ All GPIO ports initialized (A, B, F)  
✅ Automatic reset after flashing  
✅ PyOCD from virtual environment  
✅ Correct configuration for PY32F003x4

