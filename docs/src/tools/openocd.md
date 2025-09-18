# OpenOCD Support

While PyOCD is the recommended and default programming interface for PY32F0xx development, OpenOCD is also supported for users who prefer it or have specific workflow requirements.

## Files Location

OpenOCD configuration files are located in the `tools/openocd/` directory:

- **`openocd.cfg`** - OpenOCD configuration file for PY32F0xx microcontrollers
- **`openocd_program.sh`** - Shell script for programming with OpenOCD
- **`README.md`** - Documentation for OpenOCD setup

## When to Use OpenOCD

Consider using OpenOCD if you:

- Have existing OpenOCD-based workflows
- Need specific OpenOCD features
- Are integrating with tools that expect OpenOCD
- Want to use OpenOCD-specific debugging features

## Installation

### Linux (Ubuntu/Debian)
```bash
sudo apt-get install openocd
```

### macOS
```bash
brew install openocd
```

### Windows
Download from the [OpenOCD releases page](https://github.com/openocd-org/openocd/releases).

## Usage

### Using the Configuration Files

1. Navigate to the OpenOCD directory:
   ```bash
   cd tools/openocd/
   ```

2. Run OpenOCD with the provided configuration:
   ```bash
   openocd -f openocd.cfg
   ```

3. In another terminal, use GDB to load your program:
   ```bash
   arm-none-eabi-gdb target/thumbv6m-none-eabi/release/examples/blinky
   (gdb) target remote localhost:3333
   (gdb) load
   (gdb) continue
   ```

### Using the Programming Script

The `openocd_program.sh` script provides a convenient way to program your device:

```bash
./tools/openocd/openocd_program.sh path/to/your/firmware.elf
```

## Default PyOCD Workflow

For most users, we recommend sticking with the default PyOCD workflow:

```bash
# Install PyOCD (done automatically with make setup-venv)
pip install pyocd

# Use the simplified Makefile commands
make blinky           # Build example
make flash-blinky     # Build and flash
```

## Troubleshooting

### Connection Issues

If you're having connection problems with OpenOCD:

1. Check your debugger connection
2. Verify the correct configuration file is being used
3. Make sure no other debugging session is active
4. Try resetting your development board

### Permission Issues (Linux)

You may need to add udev rules for your debugger:

```bash
sudo usermod -a -G dialout $USER
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Log out and log back in for changes to take effect.

## Configuration Details

The provided `openocd.cfg` is configured specifically for PY32F0xx microcontrollers. If you need to modify it for your specific setup, refer to the OpenOCD documentation for:

- Interface configuration (ST-Link, J-Link, etc.)
- Target-specific settings
- Memory map customization

## Switching Back to PyOCD

To switch back to the default PyOCD workflow:

```bash
make clean
make flash-blinky  # Uses PyOCD by default
```

The Makefile uses PyOCD by default, so no configuration changes are needed.
