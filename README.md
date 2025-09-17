# PY32F0xx HAL - Rust Hardware Abstraction Layer

> 🦀 **Hardware Abstraction Layer (HAL) for PY32F0xx microcontrollers in Rust**

[![Crates.io](https://img.shields.io/crates/d/py32f0xx-hal.svg)](https://crates.io/crates/py32f0xx-hal)
[![Crates.io](https://img.shields.io/crates/v/py32f0xx-hal.svg)](https://crates.io/crates/py32f0xx-hal)
[![docs.rs](https://docs.rs/py32f0xx-hal/badge.svg)](https://docs.rs/py32f0xx-hal/)
[![Deploy Docs](https://github.com/UNIT-Electronics-MX/py32f0xx-hal/actions/workflows/deploy-docs.yml/badge.svg)](https://github.com/UNIT-Electronics-MX/py32f0xx-hal/actions/workflows/deploy-docs.yml)
[![Documentation](https://img.shields.io/badge/docs-live-blue)](https://unit-electronics-mx.github.io/py32f0xx-hal/)

## 🎯 What is this?

This crate provides a **Hardware Abstraction Layer (HAL)** for the PY32F0xx family of microcontrollers from Puya Semiconductor. These are **low-cost ARM Cortex-M0+ based MCUs** that offer an excellent alternative to STM32F0xx series, perfect for:

- � **LED lighting projects**
- 🤖 **IoT sensors and controllers** 
- 🔌 **Simple automation projects**
- 📟 **Learning embedded Rust**
- 💰 **Cost-sensitive applications**

## 🚀 Quick Start

### 1. Add to your `Cargo.toml`

```toml
[dependencies]
py32f0xx-hal = "0.4"
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"

[dependencies.py32f0xx-hal]
version = "0.4"
features = ["py32f003xx4", "rt"]  # Choose your specific chip
```

### 2. Blink an LED Example

```rust
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use py32f0xx_hal::{pac, prelude::*, gpio::*};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    
    let mut rcc = p.RCC.constrain();
    let gpioa = p.GPIOA.split(&mut rcc.ahb);
    
    let mut led = gpioa.pa5.into_push_pull_output();
    
    loop {
        led.set_high().unwrap();
        cortex_m::asm::delay(1_000_000);
        led.set_low().unwrap();
        cortex_m::asm::delay(1_000_000);
    }
}
```

## 📚 Documentation

📖 **[Complete Documentation & Examples](https://unit-electronics-mx.github.io/py32f0xx-hal/)**

Our documentation includes:
- 🎓 **Getting Started Guide**
- 🛠️ **Hardware Setup**
- 📝 **Code Examples**
- 🔧 **Peripheral Usage**
- 🐛 **Troubleshooting**

## 🎯 Supported Devices

| Family | Variants | Flash | RAM | Features |
|--------|----------|-------|-----|----------|
| **PY32F030** | xx4, xx6, xx7, xx8 | 16-64KB | 4-8KB | PLL, More timers |
| **PY32F003** | xx4, xx6, xx8 | 16-64KB | 4-8KB | Standard features |
| **PY32F002A** | x5 | 20KB | 3KB | Ultra low-cost |
| **PY32F002B** | x5 | 24KB | 3KB | Enhanced F002A |

### ✅ **Tested & Verified**
- **PY32F003x4** - Fully tested and working
- **PY32F003x8** - Fully tested and working

### 🧪 **Code Ready, Needs Testing**
- **PY32F030** series (all variants)
- **PY32F002A** series  
- **PY32F002B** series

> 💡 **Want to help test?** We provide hardware testing support with the [UNIT Electronics CH552 Programmer](https://github.com/UNIT-Electronics-MX/unit_ch552_multiprotocol_programmer)

## 🛠️ Features & Peripherals

### ✅ **Currently Supported**
- 🔌 **GPIO** - Digital I/O, interrupts, alternate functions
- ⏰ **Timers** - PWM, input capture, one-pulse mode
- 📡 **USART** - Serial communication
- 🔄 **I2C** - I2C master mode
- 📊 **ADC** - Analog-to-digital conversion
- ⚡ **RCC** - Clock configuration and power management

### 🚧 **Coming Soon**
- 📶 **SPI** - Serial peripheral interface
- 💾 **Flash** - Internal flash programming
- 🔋 **Low Power** - Sleep and power management modes

## 🚀 Why Choose PY32F0xx?

| Advantage | Description |
|-----------|-------------|
| 💰 **Ultra Low Cost** | Starting at $0.10 per unit |
| 🔄 **STM32 Compatible** | Similar API and peripherals |
| 📦 **Small Packages** | Available in TSSOP20, SOP16, DFN8 |
| ⚡ **Low Power** | Excellent for battery applications |
| 🛠️ **Easy Development** | Standard ARM Cortex-M0+ tools |

## Peripheral Support Matrix

| Peripheral |    F002A   |   F002B         | F030/F003      |
| ---------- | ---------- | --------------- |--------------- |
| RCC        | ✅         | ✅              | ✅              |
| GPIO       | ✅         | ✅              | ✅              |
| INTERRUPT  | ✅         | ✅              | ✅              |
| DMA        | N/A        | N/A             | ✅             |
| EXTI       | ✅         | ✅              | ✅             |
| USART      | ✅         | ✅              | ✅             |
| I2C        | ❓         | ❓              | ❓             |
| SPI        | ✅         | ✅              | ✅             |
| ADC        | ✅         | ✅              | ✅             |
| RTC        | ✅         | ✅              | ✅             |
| FLASH      |            |                 |                |
| COMP       |            |                 |                |
| Timer(PWM) | ✅         | ✅              | ✅             |
| Watchdog   | ❓         | ❓              | ❓             |
| LED        | N/A        | N/A             | N/A            |

**Legend:**
- ✅ : Implemented and tested on PY32F003x4/x8
- Blank : Not implemented
- ❓ : Requires demo verification
- N/A : Not available on this device

**Note:** Peripheral implementations are tested primarily on PY32F003x4 and PY32F003x8 devices. Other device variants are supported in code but require hardware verification.

## TODOs

- LSE/LSI test and examples
- Hardware testing on PY32F030, PY32F002A, and PY32F002B variants
- Community contributions for testing on additional chip variants

## Working Examples

This HAL includes several tested and verified examples, particularly for serial communication:

### Serial Communication Examples
- **`serial_echo.rs`** - Basic USART2 echo example (PA0=TX, PA1=RX, PB5=Debug LED)
- **`serial_adc.rs`** - Advanced USART2 + ADC example with command interface

Both examples are confirmed working on **PY32F003I DFN8** package at 9600 bps.

## 📚 Documentation

📖 **[Online Documentation](https://unit-electronics-mx.github.io/py32f0xx-hal/)** - Complete guide with examples and API reference

Comprehensive documentation is also available locally in the `docs/` directory:

```bash
# Build and view documentation locally
./build-docs.sh serve
```

The documentation includes:
- **Getting Started Guide** - Setup and first program
- **Hardware Setup** - Wiring and connections  
- **Working Examples** - Detailed code explanations
- **Peripheral Drivers** - USART, GPIO, ADC, Timers, etc.
- **API Reference** - Complete Rust documentation
- **Troubleshooting** - Common issues and solutions
- **CH552 Multiprotocol Programmer** - Compatible programmer setup for testing additional chip variants

## Getting Started

The `examples` folder contains several example programs. To compile them, specify the target device as a cargo feature:

```bash
# For PY32F030 series
$ cargo build --features=py32f030xx4 --example=blinky

# For PY32F003 series  
$ cargo build --features=py32f003xx4 --example=serial_echo

# Build and flash example
$ make flash EXAMPLE=serial_echo
```

## 💻 Development Setup

### 1. Install Rust and Tools
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add ARM Cortex-M target
rustup target add thumbv6m-none-eabi

# Install cargo-binutils for flashing
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

### 2. Choose Your Device Feature
```toml
[dependencies.py32f0xx-hal]
version = "0.4"
features = [
    "py32f003xx4",  # For PY32F003 with 16KB Flash
    "rt"            # Runtime support
]
```

**Available Features:**
- `py32f030xx4/6/7/8` - PY32F030 series (16KB-64KB Flash)
- `py32f003xx4/6/8` - PY32F003 series (16KB-64KB Flash)  
- `py32f002ax5` - PY32F002A (20KB Flash)
- `py32f002bx5` - PY32F002B (24KB Flash)

### 3. Optional Features
- `rtic` - RTIC framework support
- `defmt` - Better debugging output
- `rt` - Runtime support (recommended)

## 🌟 Community & Support

### 📖 **Learning Resources**
- [📘 Embedded Rust Book](https://docs.rust-embedded.org/book/) - Start here!
- [🦀 Rust Embedded Documentation](https://docs.rust-embedded.org/)
- [❓ FAQ](https://docs.rust-embedded.org/faq.html) - Common questions answered

### 🤝 **Get Help**
- [💬 GitHub Discussions](https://github.com/UNIT-Electronics-MX/py32f0xx-hal/discussions) - Ask questions
- [🐛 Issues](https://github.com/UNIT-Electronics-MX/py32f0xx-hal/issues) - Report bugs
- [📧 Contact UNIT Electronics](https://github.com/UNIT-Electronics-MX) - Hardware support

### 🏆 **Contributing**
We welcome contributions! Whether it's:
- 🐛 **Bug fixes**
- ✨ **New features** 
- 📝 **Documentation improvements**
- 🧪 **Hardware testing**
- 💡 **Examples and tutorials**

See [CHANGELOG.md](CHANGELOG.md) for recent updates.

## 📄 License

Licensed under your choice of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

---

<div align="center">

**Made with ❤️ by [UNIT Electronics](https://github.com/UNIT-Electronics-MX)**

*Bringing affordable embedded development to everyone*

</div>

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

**We welcome contributions!** Please feel free to:
- Submit bug reports and feature requests
- Contribute code improvements and new examples  
- Improve documentation
- **Test on different PY32F0xx devices** (especially PY32F030, PY32F002A, PY32F002B variants)
- Share testing results with the community

### Testing Contributions Needed 🧪

If you have access to untested PY32 variants, your testing contributions would be valuable:
- Use the [recommended CH552 multiprotocol programmer](https://github.com/UNIT-Electronics-MX/unit_ch552_multiprotocol_programmer)
- Test basic examples (`blinky`, `serial_echo`, `adc_values`)
- Report results via GitHub issues
- Help us complete the device compatibility matrix
