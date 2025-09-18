# OpenOCD Configuration Files

This directory contains OpenOCD configuration files for PY32F0xx development. These files are provided for users who prefer OpenOCD over PyOCD for programming and debugging.

## Files

- **`openocd.cfg`** - OpenOCD configuration file for PY32F0xx microcontrollers
- **`openocd_program.sh`** - Shell script for programming with OpenOCD

## Usage

If you want to use OpenOCD instead of PyOCD (which is the default), you can:

1. Install OpenOCD on your system
2. Use the configuration files in this directory
3. Run the programming script: `./tools/openocd/openocd_program.sh`

## Note

The main project uses **PyOCD** by default as it provides better support for PY32F0xx microcontrollers. These OpenOCD files are kept for compatibility and alternative workflows.

For the standard PyOCD workflow, use the main Makefile commands:
```bash
make flash EXAMPLE=blinky
# or the shorthand:
make flash-blinky
```
