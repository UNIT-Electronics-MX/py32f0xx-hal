# Development Environment Setup

This guide covers setting up your development environment for PY32F0xx embedded development using Rust.

## Prerequisites

### Required Software

1. **Rust Toolchain** (1.70.0 or later)
2. **Python 3.7+** (for PyOCD flashing tool)
3. **Git** (for version control)
4. **Text Editor/IDE** (VS Code recommended)

### Optional but Recommended

- **OpenOCD** (alternative to PyOCD)
- **GDB** (for debugging)
- **Logic Analyzer/Oscilloscope** (for hardware debugging)

## Installing Rust

### Linux/macOS

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart shell or source environment
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Windows

1. Download and run [rustup-init.exe](https://rustup.rs/)
2. Follow the installation wizard
3. Restart command prompt
4. Verify with `rustc --version`

### Add ARM Target

```bash
# Add Cortex-M0+ target (required for PY32F0xx)
rustup target add thumbv6m-none-eabi

# Verify target is installed
rustup target list | grep thumbv6m-none-eabi
```

## Installing PyOCD

PyOCD is the recommended flashing and debugging tool for PY32F0xx devices.

### Using pip (Recommended)

```bash
# Install PyOCD system-wide
pip install pyocd

# Or create virtual environment (recommended)
python3 -m venv py32-env
source py32-env/bin/activate  # Linux/macOS
# py32-env\\Scripts\\activate.bat  # Windows
pip install pyocd
```

### Verify Installation

```bash
# Check PyOCD version
pyocd --version

# List supported devices (should include py32f0xx)
pyocd list --targets | grep py32

# Test SWD connection (with device connected)
pyocd list
```

## Setting Up the Project

### Clone Repository

```bash
git clone https://github.com/UNIT-Electronics-MX/py32f0xx-hal.git
cd py32f0xx-hal
```

### Quick Setup Script

```bash
# Run the provided setup script
./setup.sh
```

This script will:
- Verify Rust installation
- Add ARM target if missing
- Create Python virtual environment
- Install PyOCD
- Verify all tools

### Manual Setup

If you prefer manual setup:

```bash
# Ensure ARM target is available
rustup target add thumbv6m-none-eabi

# Create and activate Python environment
python3 -m venv venv
source venv/bin/activate

# Install PyOCD
pip install pyocd

# Verify build works
make build EXAMPLE=blinky MCU_TYPE=py32f003xx4
```

## IDE Configuration

### VS Code (Recommended)

Install recommended extensions:

```bash
# Install VS Code extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension ms-vscode.cpptools
code --install-extension marus25.cortex-debug
```

**Recommended VS Code Settings** (`.vscode/settings.json`):

```json
{
    \"rust-analyzer.cargo.target\": \"thumbv6m-none-eabi\",
    \"rust-analyzer.checkOnSave.allTargets\": false,
    \"rust-analyzer.cargo.allFeatures\": false,
    \"rust-analyzer.cargo.features\": [\"py32f003xx4\"]
}
```

**Launch Configuration** (`.vscode/launch.json`):

```json
{
    \"version\": \"0.2.0\",
    \"configurations\": [
        {
            \"name\": \"Debug PY32F0xx\",
            \"type\": \"cortex-debug\",
            \"request\": \"launch\",
            \"servertype\": \"pyocd\",
            \"cwd\": \"${workspaceRoot}\",
            \"executable\": \"target/thumbv6m-none-eabi/debug/examples/blinky\",
            \"device\": \"py32f003xx4\",
            \"svdFile\": \"tools/Misc/SVD/py32f003xx.svd\"
        }
    ]
}
```

### Other IDEs

**CLion/IntelliJ IDEA:**
- Install Rust plugin
- Configure Rust toolchain
- Set target to `thumbv6m-none-eabi`

**Vim/Neovim:**
- Install rust-analyzer LSP
- Configure for embedded development

## Hardware Setup

### Programmer Connection

Connect your SWD programmer (ST-Link v2, J-Link, etc.):

```
Programmer    PY32F0xx
----------    --------
SWDIO      ←→ PA13
SWDCK      ←→ PA14  
GND        ←→ GND
3V3        ←→ VCC
RST        ←→ NRST (optional)
```

### Test Hardware Connection

```bash
# Activate Python environment if using one
source venv/bin/activate

# Check if device is detected
pyocd list

# Should show something like:
# 0 => ST-Link v2 [STM32F103C8T6]
```

## Building Your First Project

### Test Build

```bash
# Build the blinky example
make build EXAMPLE=blinky MCU_TYPE=py32f003xx4

# Should complete without errors
```

### Flash and Test

```bash
# Flash to device
make flash EXAMPLE=blinky MCU_TYPE=py32f003xx4

# LED should start blinking
```

## Development Workflow

### Typical Development Cycle

1. **Edit Code** - Modify examples or create new ones
2. **Build** - `make build EXAMPLE=your_example MCU_TYPE=py32f003xx4`
3. **Flash** - `make flash EXAMPLE=your_example MCU_TYPE=py32f003xx4`
4. **Test** - Verify functionality on hardware
5. **Debug** - Use GDB/PyOCD for troubleshooting

### Project Structure

```
py32f0xx-hal/
├── src/                 # HAL source code
├── examples/            # Example applications
├── docs/                # This documentation
├── tools/               # Development tools
├── Cargo.toml           # Rust dependencies
├── memory.x             # Memory layout
├── Makefile            # Build automation
└── setup.sh            # Environment setup
```

### Creating New Examples

```bash
# Copy existing example
cp examples/blinky.rs examples/my_project.rs

# Edit the new file
# Build and test
make build EXAMPLE=my_project MCU_TYPE=py32f003xx4
make flash EXAMPLE=my_project MCU_TYPE=py32f003xx4
```

## Debugging Setup

### GDB with PyOCD

Start PyOCD GDB server:

```bash
# Terminal 1 - Start GDB server
pyocd gdbserver --target py32f003xx4

# Terminal 2 - Connect GDB
arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/examples/blinky
(gdb) target remote localhost:3333
(gdb) load
(gdb) break main
(gdb) continue
```

### VS Code Debugging

1. Set breakpoints in code
2. Press F5 (Start Debugging)
3. VS Code will build, flash, and start debug session
4. Step through code, inspect variables

## Environment Variables

Useful environment variables for development:

```bash
# In your ~/.bashrc or ~/.zshrc
export PY32_TARGET=py32f003xx4
export PY32_PROGRAMMER=pyocd

# Use in commands
make build MCU_TYPE=${PY32_TARGET}
```

## Troubleshooting

### Common Issues

**\"thumbv6m-none-eabi\" target not found:**
```bash
rustup target add thumbv6m-none-eabi
```

**PyOCD not found:**
```bash
# Ensure Python environment is activated
source venv/bin/activate
pip install pyocd
```

**Permission denied on Linux:**
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER
# Log out and back in
```

**Build failures:**
```bash
# Clean and rebuild
make clean
make build EXAMPLE=blinky MCU_TYPE=py32f003xx4
```

### Getting Help

1. Check [Troubleshooting section](../troubleshooting/build.md)
2. Review [Hardware Setup](./hardware-setup.md)
3. Open issue on GitHub with:
   - OS and versions
   - Complete error messages
   - Steps to reproduce

## Next Steps

Once your environment is set up:

1. **Try [Your First Program](./first-program.md)** tutorial
2. **Explore [Examples](../examples/blinky.md)** to learn the HAL
3. **Read [Hardware Setup](./hardware-setup.md)** for wiring details
4. **Check [Peripheral Documentation](../peripherals/gpio.md)** for advanced usage

## Performance Tips

- **Use release builds** for final deployment: `cargo build --release`
- **Enable LTO** for smaller binaries in `Cargo.toml`
- **Profile memory usage** with `cargo bloat`
- **Optimize for size** with `opt-level = \"s\"` in `Cargo.toml`

Your development environment is now ready for PY32F0xx embedded development!
