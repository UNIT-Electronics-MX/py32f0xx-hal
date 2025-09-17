# PY32F0xx HAL Documentation

Welcome to the PY32F0xx Hardware Abstraction Layer (HAL) documentation. This library provides a Rust-based hardware abstraction layer for the Puya Semiconductor PY32F0xx family of microcontrollers.

[![Crates.io](https://img.shields.io/crates/d/py32f0xx-hal.svg)](https://crates.io/crates/py32f0xx-hal)
[![Crates.io](https://img.shields.io/crates/v/py32f0xx-hal.svg)](https://crates.io/crates/py32f0xx-hal)
[![docs.rs](https://docs.rs/py32f0xx-hal/badge.svg)](https://docs.rs/py32f0xx-hal/)

## What is PY32F0xx?

The PY32F0xx family consists of low-cost ARM Cortex-M0+ based microcontrollers that offer an excellent alternative to STM32F0xx series. These MCUs are manufactured by Puya Semiconductor and provide:

- **Low Cost**: Extremely affordable microcontrollers
- **ARM Cortex-M0+**: Industry-standard 32-bit processor
- **Rich Peripherals**: USART, SPI, I2C, ADC, Timers, and more
- **Small Packages**: Available in compact packages like DFN8
- **Wide Voltage Range**: Typically 1.7V to 5.5V operation

## Supported Devices

This HAL supports the following PY32F0xx device families:

- **PY32F030** (16KB, 32KB, 48KB, 64KB Flash variants)
- **PY32F003** (16KB, 32KB, 64KB Flash variants) 
- **PY32F002A** (20KB Flash)
- **PY32F002B** (20KB Flash)

## Key Features

- ✅ **Type-safe GPIO** with compile-time pin configuration
- ✅ **Serial Communication** (USART) with working examples
- ✅ **ADC Support** for analog measurements
- ✅ **PWM and Timers** for precise timing control
- ✅ **SPI and I2C** for device communication
- ✅ **Real-time Clock** (RTC) support
- ✅ **DMA Support** (on supported devices)
- ✅ **Embedded HAL Compatibility** for ecosystem integration

## Working Examples

This documentation includes comprehensive examples that have been tested and verified on real hardware:

### Verified Serial Examples
- **Serial Echo** - Basic USART2 communication
- **Serial ADC** - Advanced serial + ADC with command interface

Both examples work reliably on **PY32F003I DFN8** package at 9600 bps.

## What's in This Documentation?

This book is organized into several sections to help you get started and become productive with PY32F0xx development:

- **[Getting Started](./getting-started/quick-start.md)** - Set up your development environment and create your first project
- **[Device Guides](./devices/py32f003-guide.md)** - Device-specific information and configuration guides  
- **[Examples](./examples/blinky.md)** - Step-by-step examples with working code
- **[Peripheral Drivers](./peripherals/gpio.md)** - Detailed documentation for each peripheral
- **[Troubleshooting](./troubleshooting/serial.md)** - Solutions to common issues
- **[Reference](./reference/pin-mapping.md)** - Technical reference materials

## Getting Help

If you encounter issues or have questions:

1. Check the **[Troubleshooting](./troubleshooting/serial.md)** section
2. Look for similar examples in the **[Examples](./examples/blinky.md)** section
3. Review the **[Peripheral Drivers](./peripherals/gpio.md)** documentation
4. Open an issue on [GitHub](https://github.com/UNIT-Electronics-MX/py32f0xx-hal)

## Contributing

We welcome contributions! Whether you're fixing bugs, adding examples, improving documentation, or testing on new devices, your help is appreciated. See our **[Contributing Guidelines](./contributing/how-to-contribute.md)** to get started.

---

Ready to begin? Start with the **[Quick Start Guide](./getting-started/quick-start.md)**!
