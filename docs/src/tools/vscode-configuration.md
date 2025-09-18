# VS Code Configuration

This page details the VS Code configuration included in the project for optimal PY32F0xx development.

## Required Extensions

Install these extensions for full functionality:

### Core Development
```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension ms-vscode.cpptools
```

### Debugging
```bash
code --install-extension marus25.cortex-debug
```

## Settings Configuration

The project includes `.vscode/settings.json` with optimized settings for embedded Rust development:

```json
{
    "rust-analyzer.cargo.target": "thumbv6m-none-eabi",
    "rust-analyzer.checkOnSave.allTargets": false,
    "rust-analyzer.cargo.allFeatures": false,
    "rust-analyzer.cargo.features": ["py32f003xx4"],
    "rust-analyzer.check.allTargets": false,
    "files.associations": {
        "*.svd": "xml"
    },
    "cortex-debug.variableUseNaturalFormat": true,
    "cortex-debug.showRTOS": true
}
```

### Settings Explanation

- **`rust-analyzer.cargo.target`**: Sets the target architecture for analysis
- **`checkOnSave.allTargets`**: Disables checking all targets (improves performance)
- **`cargo.allFeatures`**: Disables all features analysis for better performance  
- **`cargo.features`**: Specifies default MCU features (PY32F003x4)
- **`files.associations`**: Associates SVD files with XML syntax highlighting
- **`cortex-debug.*`**: Optimizes debugging display options

## Launch Configuration

The debugging configuration is in `.vscode/launch.json`:

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
            "configFiles": ["tools/Misc/pyocd.yaml"],
            "targetId": "py32f003x4",
            "runToEntryPoint": "main",
            "showDevDebugOutput": "raw"
        }
    ]
}
```

### Configuration Explanation

- **`servertype: "pyocd"`**: Uses PyOCD as debug server
- **`device: "py32f003x4"`**: Specifies target device
- **`svdFile`**: Points to SVD file for peripheral register view
- **`configFiles`**: Uses project PyOCD configuration
- **`runToEntryPoint`**: Stops at main function
- **`showDevDebugOutput`**: Shows detailed debug output

## Different MCU Types

To use different MCU variants, update both files:

### For PY32F003x6:
```json
// .vscode/settings.json
"rust-analyzer.cargo.features": ["py32f003xx6"]

// .vscode/launch.json  
"device": "py32f003x6"
```

### For PY32F030x6:
```json
// .vscode/settings.json
"rust-analyzer.cargo.features": ["py32f030xx6"]

// .vscode/launch.json
"device": "py32f030x6"
```

## Multiple Debug Configurations

You can add multiple configurations for different examples:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug Blinky",
            "type": "cortex-debug",
            "executable": "target/thumbv6m-none-eabi/debug/examples/blinky",
            // ... other settings
        },
        {
            "name": "Debug Serial Echo", 
            "type": "cortex-debug",
            "executable": "target/thumbv6m-none-eabi/debug/examples/serial_echo",
            // ... other settings
        }
    ]
}
```

## Workspace Recommendations

The project includes `.vscode/extensions.json` with recommended extensions:

```json
{
    "recommendations": [
        "rust-lang.rust-analyzer",
        "ms-vscode.cpptools", 
        "marus25.cortex-debug"
    ]
}
```

## Tasks Configuration

You can also add `.vscode/tasks.json` for build tasks:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Build Blinky",
            "type": "shell",
            "command": "make",
            "args": ["blinky"],
            "group": "build",
            "problemMatcher": "$rustc"
        },
        {
            "label": "Flash Blinky",
            "type": "shell", 
            "command": "make",
            "args": ["flash-blinky"],
            "group": "build",
            "dependsOn": "Build Blinky"
        },
        {
            "label": "Debug Build Blinky",
            "type": "shell",
            "command": "make", 
            "args": ["debug-blinky"],
            "group": "build"
        }
    ]
}
```

## Troubleshooting

### Rust Analyzer Issues

If Rust Analyzer is slow or not working:

1. **Restart Rust Analyzer**: `Ctrl+Shift+P` → "rust-analyzer: Restart server"
2. **Clear cache**: Delete `target/` directory and rebuild
3. **Check target**: Ensure `thumbv6m-none-eabi` is installed

### Debug Issues

If debugging doesn't work:

1. **Check extensions**: Ensure Cortex-Debug is installed
2. **Build debug**: Run `make debug-blinky` first
3. **Check hardware**: Verify SWD connection
4. **PyOCD test**: Run `make -f rust.mk debug` to test connection

### Performance Issues

If VS Code is slow:

1. **Disable unused extensions** in workspace
2. **Use specific features**: Don't enable all MCU features  
3. **Exclude build dirs**: Add to `.gitignore` and VS Code exclude

## Integration with Make Commands

The VS Code configuration works seamlessly with Make commands:

```bash
# Build for analysis
make blinky

# Build for debugging  
make debug-blinky

# Flash and test
make flash-blinky
```

## Advanced Configuration

### Custom Keybindings

Add to your `keybindings.json`:

```json
[
    {
        "key": "f6",
        "command": "workbench.action.tasks.runTask",
        "args": "Build Blinky"
    },
    {
        "key": "f7", 
        "command": "workbench.action.tasks.runTask",
        "args": "Flash Blinky"
    }
]
```

### Integrated Terminal

Set default shell for embedded development:

```json
{
    "terminal.integrated.defaultProfile.linux": "bash",
    "terminal.integrated.cwd": "${workspaceFolder}"
}
```

This configuration provides a complete, optimized VS Code experience for PY32F0xx development!
