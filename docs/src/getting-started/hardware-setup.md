# Hardware Setup

This guide covers the hardware setup requirements for PY32F0xx development, including wiring, programmer connections, and common pin configurations.

## Development Hardware

### Minimum Requirements

1. **PY32F0xx Microcontroller** - Any supported variant
2. **SWD Programmer** - Multiprotocol Programmer, ST-Link v2, J-Link, or compatible
3. **Power Supply** - 3.3V (or 1.7V-5.5V depending on variant)
4. **Breadboard/PCB** - For prototyping connections

### Recommended Setup

- **Development Board** with built-in SWD programmer
- **Oscilloscope/Logic Analyzer** for debugging
- **Multimeter** for voltage verification
- **LED** and resistor for basic output testing

## SWD Programming Connections

All PY32F0xx devices support Serial Wire Debug (SWD) for programming and debugging.

### Standard SWD Pinout

| SWD Signal | PY32F0xx Pin | Description |
|------------|--------------|-------------|
| SWDIO      | PA13         | Data line   |
| SWDCK      | PA14         | Clock line  |
| NRST       | NRST         | Reset (optional) |
| GND        | GND          | Ground      |
| VCC        | VCC          | Power (3.3V) |

### Wiring Example (ST-Link v2)

```
Multiprotocol      PY32F0xx
-------------      --------
SWDIO        <--> PA13
SWDCK        <--> PA14  
GND          <--> GND
3V3          <--> VCC
RST          <--> NRST (optional)
```

## Package-Specific Pin Configurations

### DFN8 Package (PY32F003I)

The DFN8 is a compact 8-pin package commonly used for space-constrained applications.

```
            DFN8 Pinout (Top View):
               VCC  1 ┌─────┐ 8  PB5/LED
               PA0  2 │     │ 7  PA14-SWDCK/PB6
               PA1  3 │     │ 6  PA13-SWDIO/PA10
               PA2  4 └─────┘ 5  PB0/PF2-NRST

```

#### Verified DFN8 Configuration

This configuration is tested and working on PY32F003I DFN8:

| Pin | Function | Description |
|-----|----------|-------------|
| PA0 | USART2 TX (AF9) | Serial transmit |
| PA1 | USART2 RX (AF9) | Serial receive |  
| PA2 | ADC Channel 2   | Analog input |
| PB5 | GPIO Output     | Debug LED |
| PA13| SWDIO          | Programming |
| PA14| SWDCK          | Programming |


## Power Supply Considerations

### Voltage Requirements

- **PY32F002**: 1.7V - 5.5V
- **PY32F003**: 1.7V - 5.5V  
- **PY32F030**: 1.7V - 5.5V

### Power Connections

```
Power Rails:
VCC/VDD  --> Main power supply (3.3V recommended)
VDDA     --> Analog power (connect to VCC)
VSS/GND  --> Ground (0V)
VSSA     --> Analog ground (connect to GND)
```

### Decoupling

Always include decoupling capacitors:
- **100nF ceramic** close to each VCC pin
- **10μF tantalum/electrolytic** for bulk decoupling
- **100nF ceramic** between VDDA and VSSA

## Clock Configuration

### Internal Oscillators

PY32F0xx devices have built-in oscillators:
- **HSI**: 8MHz internal RC oscillator (default)
- **LSI**: ~40kHz internal low-speed oscillator

### External Crystals (Optional)

For applications requiring precise timing:
- **HSE**: 4-32MHz external crystal
- **LSE**: 32.768kHz external crystal for RTC

```
HSE Crystal Connections:
OSC_IN   --> Crystal + 12-22pF capacitor to GND
OSC_OUT  --> Crystal + 12-22pF capacitor to GND
```

## Common Test Circuits

### Basic LED Test

```
PB5 --> 330Ω resistor --> LED --> GND
```

### Serial Communication Test

```
PA0 (TX) --> RX of USB-to-Serial adapter
PA1 (RX) <-- TX of USB-to-Serial adapter
GND      --> GND of USB-to-Serial adapter
```

### ADC Input Test

```
PA2 --> 10kΩ potentiometer --> GND
        (center tap to PA2)
```

## Troubleshooting Hardware Issues

### Programming Issues

1. **Can't connect to device**
   - Check SWD wiring (SWDIO, SWDCK, GND)
   - Verify power supply (3.3V)
   - Try different SWD frequency

2. **Device not responding**
   - Check NRST connection
   - Verify crystal oscillator (if using HSE)
   - Try holding NRST low during connection

### Runtime Issues

1. **LED not blinking**
   - Check LED polarity and current-limiting resistor
   - Verify GPIO configuration in code
   - Measure voltage on GPIO pin

2. **Serial not working**
   - Check baud rate (9600 default)
   - Verify TX/RX wiring (not crossed)
   - Test with known-good USB-to-Serial adapter

3. **ADC readings incorrect**
   - Check VDDA connection
   - Verify input voltage range (0-VDDA)
   - Check reference voltage configuration

## Safety Guidelines

- Never exceed maximum voltage ratings
- Use current-limiting resistors with LEDs
- Double-check power supply polarity
- Avoid ESD damage - use anti-static precautions
- Keep connections short for high-frequency signals

## Next Steps

Once your hardware is set up:
1. Follow the [Development Environment](./development-environment.md) guide
2. Try the [First Program](./first-program.md) tutorial
3. Explore [Working Examples](../examples/blinky.md)
