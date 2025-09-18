# VS Code Debugging Setup

This guide shows how to set up debugging for PY32F0xx development in VS Code using the Cortex-Debug extension.

## Prerequisites

1. **Cortex-Debug Extension** - Install from VS Code marketplace:
   ```bash
   code --install-extension marus25.cortex-debug
   ```

2. **PyOCD installed and configured** - Should be done automatically by setup script:
   ```bash
   ./scripts/setup.sh
   ```

## Launch Configuration

The project includes a pre-configured `launch.json` for debugging:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug PY32F0xx",
            "type": "cortex-debug",
            "request": "launch",
            "servertype": "pyocd",
            "cwd": "${workspaceRoot}",
            "executable": "target/thumbv6m-none-eabi/debug/examples/blinky",
            "device": "py32f003x4",
            "svdFile": "tools/Misc/SVD/py32f003xx.svd",
            "configFiles": [
                "tools/Misc/pyocd.yaml"
            ],
            "targetId": "py32f003x4",
            "runToEntryPoint": "main",
            "showDevDebugOutput": "raw"
        }
    ]
}
```

## How to Debug

### 1. Build Debug Version

First, build your example in debug mode:

```bash
# Simple way - build debug version of blinky
make debug-blinky

# Build debug version of serial_echo
make debug-serial_echo

# Or for specific MCU (traditional way)
make debug-build EXAMPLE=blinky MCU_TYPE=PY32F003x4
```

### 2. Start Debugging

1. **Set Breakpoints** - Click in the gutter next to line numbers
2. **Press F5** or go to **Run > Start Debugging**
3. **Select "Debug PY32F0xx"** configuration

### 3. Debug Controls

- **F5** - Continue
- **F10** - Step Over
- **F11** - Step Into
- **Shift+F11** - Step Out
- **Ctrl+Shift+F5** - Restart
- **Shift+F5** - Stop

## Debugging Different Examples

To debug different examples, update the `executable` path in `launch.json`:

```json
"executable": "target/thumbv6m-none-eabi/debug/examples/YOUR_EXAMPLE"
```

**Available examples:**
- `blinky`
- `serial_echo`
- `serial_adc`
- `adc_values`
- `pwm`
- And more...

## Multiple Configurations

You can add multiple debug configurations for different examples:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug Blinky",
            "type": "cortex-debug",
            "request": "launch",
            "servertype": "pyocd",
            "executable": "target/thumbv6m-none-eabi/debug/examples/blinky",
            "device": "py32f003x4",
            "svdFile": "tools/Misc/SVD/py32f003xx.svd",
            "configFiles": ["tools/Misc/pyocd.yaml"]
        },
        {
            "name": "Debug Serial Echo",
            "type": "cortex-debug",
            "request": "launch",
            "servertype": "pyocd",
            "executable": "target/thumbv6m-none-eabi/debug/examples/serial_echo",
            "device": "py32f003x4",
            "svdFile": "tools/Misc/SVD/py32f003xx.svd",
            "configFiles": ["tools/Misc/pyocd.yaml"]
        }
    ]
}
```

## Different MCU Types

For different MCU variants, update the `device` field:

- **PY32F003x4**: `"device": "py32f003x4"`
- **PY32F003x6**: `"device": "py32f003x6"`
- **PY32F003x8**: `"device": "py32f003x8"`
- **PY32F030x6**: `"device": "py32f030x6"`
- **PY32F030x8**: `"device": "py32f030x8"`

## Troubleshooting

### "Could not connect to target" Error

1. **Check hardware connection**:
   - SWD pins connected correctly
   - Device powered
   - Debugger connected

2. **Verify PyOCD can see device**:
   ```bash
   cd tools/Misc
   ../../venv/bin/pyocd list
   ```

3. **Check device matches configuration**:
   ```bash
   ../../venv/bin/pyocd info --config pyocd.yaml
   ```

### "Executable not found" Error

Make sure you've built the debug version:

```bash
# Build debug version of your example
make debug-blinky
make debug-serial_echo
make debug-pwm

# Or specific example
make debug-build EXAMPLE=your_example
```

### Permission Issues (Linux)

Add user to dialout group:

```bash
sudo usermod -a -G dialout $USER
# Log out and back in
```

### SVD File Issues

If peripheral registers don't show properly, verify the SVD file path:
- File should be at: `tools/Misc/SVD/py32f003xx.svd`
- Check that the SVD file matches your MCU type

## Debug Output

The configuration includes `"showDevDebugOutput": "raw"` which shows:
- PyOCD connection messages
- Memory read/write operations
- Reset and halt operations

This helps diagnose connection issues.

## Advanced Features

### Memory View
- **View > Command Palette** → **Cortex-Debug: View Memory**
- Enter memory address (e.g., `0x20000000` for RAM)

### Peripheral Registers
- Available in **Debug** sidebar when debugging
- Shows all MCU peripherals with current values
- Requires correct SVD file

### Disassembly View
- **View > Command Palette** → **Cortex-Debug: View Disassembly**
- Shows assembly code with source correlation

## Tips

1. **Use debug build** - Optimized builds are harder to debug
2. **Set meaningful breakpoints** - Avoid breakpoints in tight loops
3. **Check variables** - Hover over variables to see values
4. **Use watch expressions** - Add variables to watch panel
5. **Single-step carefully** - Some operations might affect timing-sensitive code
