# Multi-Protocol Programmer

<div align="center">
  <img src="https://img.shields.io/badge/version-1.0-blue.svg" />
  <img src="https://img.shields.io/badge/language-C-lightgrey.svg" />
  <img src="https://img.shields.io/badge/language-Python-lightgrey.svg" />
  <img src="https://img.shields.io/badge/license-MIT-green.svg" />
</div>

<div align="center">
  <img src="https://raw.githubusercontent.com/UNIT-Electronics-MX/unit_ch552_multiprotocol_programmer/refs/heads/main/hardware/resources/programmer.png" width="480" alt="Multi-Protocol Programmer" />
</div>

<div align="center">

## Resources

<table>
  <thead>
    <tr>
      <th>Resource</th>
      <th>Link</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Wiki</td>
      <td><a href="https://unit-electronics-mx.github.io/wiki_uelectronics/es/docs/Development_boards/devlab/multiprotocol/">Development Boards Wiki</a></td>
    </tr>
    <tr>
      <td>Documentation</td>
      <td><a href="https://unit-electronics-mx.github.io/unit_multiprotocol_programmer_platform/">unit_multiprotocol_programmer_platform</a></td>
    </tr>
    <tr>
      <td>Getting Started</td>
      <td><a href="https://unit-electronics-mx.github.io/unit_ch552_multiprotocol_programmer/index.html">Initial Setup</a></td>
    </tr>
    <tr>
      <td>Schematic & PCB</td>
      <td><a href="https://github.com/UNIT-Electronics-MX/unit_ch552_multiprotocol_programmer/tree/main/hardware">Hardware Files</a></td>
    </tr>
    <tr>
      <td>Firmware & SDK</td>
      <td><a href="https://github.com/UNIT-Electronics-MX/unit_ch55x_docker_sdk">SDK & Firmware</a></td>
    </tr>
    <tr>
      <td>Main Repository</td>
      <td><a href="https://github.com/UNIT-Electronics-MX/unit_ch552_multiprotocol_programmer">GitHub Repo</a></td>
    </tr>
  </tbody>
</table>

</div>

## Firmware Required

This programmer **requires specific firmware** depending on the protocol:

* **AVR**: USBasp & UPDI
* **ARM**: CMSIS-DAP (SWD/JTAG)
* **CPLD**: USB-Blaster (JTAG)

Load the correct `.bin` before use. Without it, the device won't function properly.

## Overview

The **Multi-Protocol Programmer** is a USB tool based on the **CH552** microcontroller. It supports flashing and debugging of:

* **AVR microcontrollers** (ATmega, ATtiny, AVR-DA)
* **ARM Cortex-M** devices (STM32, nRF52, SAM, etc.)
* **Intel/Altera MAX II CPLDs** (EPM240, EPM570, etc.)

### Features

* USB Full-Speed (CDC/HID)
* Voltage selector: 3.3V / 5V
* SWD / JTAG / UPDI / USBasp support
* Works with popular tools (avrdude, OpenOCD, Quartus, etc.)

## PY32F0xx Configuration

The Multi-Protocol Programmer is fully compatible with PY32F0xx microcontrollers when using the **CMSIS-DAP firmware**.

### Testing Status & Recommendations

#### **Tested Devices ✅**
- **PY32F003x4** - Fully verified with this programmer
- **PY32F003x8** - Fully verified with this programmer

#### **Recommended for Testing 🧪**
For the following devices that are supported in code but need hardware verification:
- **PY32F030** series (all variants)
- **PY32F002A** series  
- **PY32F002B** series

**This programmer is the recommended solution for testing these untested device variants.**

### Setup for PY32F0xx

1. **Flash CMSIS-DAP Firmware:**
   ```bash
   python3 tools/chprog.py firmware/cmsis_dap.bin
   ```

2. **Connect to PY32F0xx:**
   ```
   Programmer    PY32F0xx
   ----------    --------
   SWDIO      ←→ PA13
   SWDCK      ←→ PA14  
   GND        ←→ GND
   VTG        ←→ VCC (3.3V/5V)
   RST        ←→ NRST (optional)
   ```

3. **Set Correct Voltage:**
   - Use **3.3V** for most PY32F0xx devices
   - Check your specific device voltage requirements

### Using with PyOCD

The programmer works seamlessly with PyOCD for PY32F0xx development:

```bash
# List connected devices
pyocd list

# Flash PY32F003
pyocd flash firmware.bin --target py32f003xx4

# Start GDB server
pyocd gdbserver --target py32f003xx4

# Interactive debugging
pyocd commander --target py32f003xx4
```

### Using with OpenOCD

OpenOCD can also be used with the CMSIS-DAP firmware:

```bash
# Flash firmware
openocd -f interface/cmsis-dap.cfg -f target/py32f0xx.cfg -c "program firmware.bin verify reset exit"

# Start GDB server
openocd -f interface/cmsis-dap.cfg -f target/py32f0xx.cfg
```

### Makefile Integration

Update your project Makefile to use the Multi-Protocol Programmer:

```makefile
# Use CMSIS-DAP interface
PROGRAMMER = cmsis-dap
PROGRAMMER_ARGS = --target py32f003xx4

flash: $(BUILD_DIR)/$(EXAMPLE).bin
	pyocd flash $(BUILD_DIR)/$(EXAMPLE).bin $(PROGRAMMER_ARGS)

debug: $(BUILD_DIR)/$(EXAMPLE).elf
	pyocd gdbserver $(PROGRAMMER_ARGS) &
	arm-none-eabi-gdb $(BUILD_DIR)/$(EXAMPLE).elf
```

## Supported Protocols

| Firmware      | Protocols    | Target Devices         | Interface | Tools                |
| ------------- | ------------ | ---------------------- | --------- | -------------------- |
| **AVR**       | USBasp, UPDI | ATmega, ATtiny         | CDC/HID   | avrdude, Arduino IDE |
| **CMSIS-DAP** | SWD, JTAG    | STM32, nRF52, PY32F0xx | HID+CDC   | OpenOCD, PyOCD, Keil |
| **CPLD**      | USB-Blaster  | EPM240, EPM570, MAX II | HID       | Quartus Prime        |

## Flashing Firmware

1. **Enter Bootloader Mode:**
   * Hold `BOOT`, plug USB, release.

2. **Flash Firmware:**
   ```bash
   python3 tools/chprog.py firmware/firmware_name.bin
   ```
   Or use WCHISPTool on Windows.

## Install Requirements

```bash
# Linux (Debian/Ubuntu)
sudo apt install build-essential sdcc python3-pip git
pip3 install pyusb pyocd

# Add user to dialout group for device access
sudo usermod -a -G dialout $USER
# Log out and back in for changes to take effect
```

For Windows:
Download [SDCC](https://sdcc.sourceforge.net/), [Python 3](https://python.org/), and [Git](https://git-scm.com/).

## Troubleshooting

### Common Issues

* **Device not recognized?**
  ➤ Check firmware & USB drivers (use Zadig on Windows for CMSIS-DAP)

* **Programming error?**
  ➤ Verify voltage level (3.3V/5V), connections & cable quality

* **Slow upload?**
  ➤ Reduce SWD/JTAG frequency or use shorter cables

### PY32F0xx Specific

* **PyOCD can't find device?**
  ```bash
  # Check if programmer is detected
  pyocd list
  
  # Try different target specification
  pyocd flash firmware.bin --target py32f030xx4
  ```

* **SWD connection issues?**
  - Verify SWDIO/SWDCK connections
  - Ensure stable power supply
  - Try lower SWD frequency: `--frequency 1000000`

* **Flashing fails?**
  ```bash
  # Try mass erase first
  pyocd erase --chip --target py32f003xx4
  
  # Then flash
  pyocd flash firmware.bin --target py32f003xx4
  ```

### Testing Contributions

**Help expand device support!** If you test py32f0xx-hal with untested devices using this programmer:

1. **Test procedure:**
   - Flash basic examples (`blinky`, `serial_echo`)
   - Verify peripheral functionality
   - Document any issues or successes

2. **Report results:**
   - Open GitHub issue with test results
   - Include device model, programmer setup, and example outcomes
   - Help us update the compatibility matrix

## Firmware Selection Guide

For **PY32F0xx development**, use the **CMSIS-DAP firmware**:

| Target Family | Recommended Firmware | Interface | Tools |
|---------------|---------------------|-----------|--------|
| **PY32F0xx** | CMSIS-DAP | SWD | PyOCD, OpenOCD |
| **STM32F0xx** | CMSIS-DAP | SWD | PyOCD, OpenOCD |
| **AVR** | USBasp/UPDI | ISP/UPDI | avrdude |
| **MAX II CPLD** | USB-Blaster | JTAG | Quartus |

## Performance Notes

### SWD Frequency Settings

For reliable PY32F0xx programming:

- **High-speed**: `--frequency 10000000` (10MHz) - for short cables
- **Standard**: `--frequency 1000000` (1MHz) - recommended default
- **Low-speed**: `--frequency 400000` (400kHz) - for problematic connections

### Cable Quality

- **Use short cables** (< 15cm) for high-speed SWD
- **Twisted pair** for SWDIO/SWDCK reduces noise
- **Good ground connection** essential for reliable operation

## License

* Hardware: CC BY-SA 4.0
* Firmware & Software: MIT License  
* Third-party components: see individual `LICENSE` files
