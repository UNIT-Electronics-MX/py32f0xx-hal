# Quick Start Guide

This guide will get you up and running with PY32F0xx development using Rust in just a few minutes.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust toolchain** (1.70 or later)
- **Python 3.7+** for flashing tools
- **Git** for version control
- A **PY32F0xx development board** or chip

## 1. Clone the Repository

```bash
git clone https://github.com/UNIT-Electronics-MX/py32f0xx-hal.git
cd py32f0xx-hal
```

## 2. Quick Setup

Run the setup script to configure your development environment:

```bash
./scripts/setup.sh
```

This script automatically:
- Adds the `thumbv6m-none-eabi` Rust target
- Creates a Python virtual environment with PyOCD
- Verifies all required tools are available

### Manual Setup (Alternative)

If you prefer to set up manually:

```bash
# Add Rust target for ARM Cortex-M0+
rustup target add thumbv6m-none-eabi

# Create Python virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\\Scripts\\activate
pip install pyocd

# Or use the Makefile
make setup-venv
```

## 3. Build Your First Example

Let's build the classic "blinky" LED example using the simplified commands:

```bash
# Simple way - build blinky (default MCU: PY32F003x4)
make blinky

# Or specify MCU type explicitly
make EXAMPLE=blinky MCU_TYPE=PY32F003x4
```

**New Simplified Commands:**
- `make blinky` - Build blinky example
- `make serial_echo` - Build serial example
- `make pwm` - Build PWM example
- `make adc_values` - Build ADC example

Available MCU types:
- `PY32F003x4` - PY32F003 with 16KB Flash (default)
- `PY32F003x6` - PY32F003 with 32KB Flash
- `PY32F003x8` - PY32F003 with 64KB Flash
- `PY32F030x4` - PY32F030 with 16KB Flash
- `PY32F030x6` - PY32F030 with 32KB Flash
- `PY32F030x7` - PY32F030 with 48KB Flash  
- `PY32F030x8` - PY32F030 with 64KB Flash
- `PY32F002Ax5` - PY32F002A with 20KB Flash
- `PY32F002Bx5` - PY32F002B with 20KB Flash

## 4. Flash to Your Device

Connect your PY32F0xx board via SWD and flash the firmware:

```bash
# Super simple - build and flash blinky
make flash-blinky

# Or with explicit MCU type
make flash EXAMPLE=blinky MCU_TYPE=PY32F003x4

# Alternative syntax
make example=blinky flash
```

## 5. Verify It Works

If everything is set up correctly, you should see:
- The LED on your board blinking
- No compilation errors
- Successful flashing messages

## Quick Reference Commands

Get help anytime with:

```bash
make help          # Show quick command reference
make full-help     # Show complete help with all options
```

**Most Used Commands:**
```bash
make blinky        # Build blinky
make flash-blinky  # Build and flash blinky
make serial_echo   # Build serial example
make flash-serial_echo # Build and flash serial
make clean         # Clean build files
```

## Next Steps

Now that you have a working setup, explore more examples with the simplified commands:

```bash
# Try the serial echo example
make flash-serial_echo

# Or the ADC example  
make flash EXAMPLE=serial_adc

# PWM example
make flash-pwm

# List all available examples
make list-examples
```

## Common Issues

### Compilation Errors
If you get target-related errors:
```bash
rustup target add thumbv6m-none-eabi
```

### Flashing Errors
If PyOCD can't find your device:
1. Check SWD connections
2. Verify device is powered
3. Try: `pyocd list` to see connected devices

### Permission Issues (Linux)
Add yourself to the dialout group:
```bash
sudo usermod -a -G dialout $USER
# Log out and back in
```

## What's Next?

- Read the [Hardware Setup Guide](./hardware-setup.md) for wiring details
- Explore [Working Examples](../examples/blinky.md) for code samples
- Check [Device-Specific Guides](../devices/py32f003-guide.md) for your chip
- Learn about [Peripheral Drivers](../peripherals/gpio.md) available

## Need Help?

- Check the [Troubleshooting Section](../troubleshooting/build.md)
- Review [Verification Guide](../troubleshooting/verification.md) 
- Open an issue on [GitHub](https://github.com/UNIT-Electronics-MX/py32f0xx-hal/issues)
