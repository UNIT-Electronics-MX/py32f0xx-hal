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
git clone https://github.com/Cesarbautista10/py32f0xx-hal.git
cd py32f0xx-hal
```

## 2. Quick Setup

Run the setup script to configure your development environment:

```bash
./setup.sh
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

Let's build the classic "blinky" LED example:

```bash
# Build the blinky example for PY32F003
make build EXAMPLE=blinky MCU_TYPE=py32f003xx4
```

Available MCU types:
- `py32f030xx4` - PY32F030 with 16KB Flash
- `py32f030xx6` - PY32F030 with 32KB Flash
- `py32f030xx7` - PY32F030 with 48KB Flash  
- `py32f030xx8` - PY32F030 with 64KB Flash
- `py32f003xx4` - PY32F003 with 16KB Flash
- `py32f003xx6` - PY32F003 with 32KB Flash
- `py32f003xx8` - PY32F003 with 64KB Flash
- `py32f002ax5` - PY32F002A with 20KB Flash
- `py32f002bx5` - PY32F002B with 20KB Flash

## 4. Flash to Your Device

Connect your PY32F0xx board via SWD and flash the firmware:

```bash
# Flash the blinky example
make flash EXAMPLE=blinky MCU_TYPE=py32f003xx4
```

## 5. Verify It Works

If everything is set up correctly, you should see:
- The LED on your board blinking
- No compilation errors
- Successful flashing messages

## Next Steps

Now that you have a working setup, explore more examples:

```bash
# Try the serial echo example
make flash EXAMPLE=serial_echo MCU_TYPE=py32f003xx4

# Or the ADC example  
make flash EXAMPLE=serial_adc MCU_TYPE=py32f003xx4
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
- Open an issue on [GitHub](https://github.com/Cesarbautista10/py32f0xx-hal/issues)
