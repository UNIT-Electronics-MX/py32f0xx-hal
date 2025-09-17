py32f0xx-hal
================

> Hardware Abstraction Layer (HAL) for the PY32F0xx family of microcontrollers

[![Crates.io](https://img.shields.io/crates/d/py32f0xx-hal.svg)](https://crates.io/crates/py32f0xx-hal)
[![Crates.io](https://img.shields.io/crates/v/py32f0xx-hal.svg)](https://crates.io/crates/py32f0xx-hal)
[![docs.rs](https://docs.rs/py32f0xx-hal/badge.svg)](https://docs.rs/py32f0xx-hal/)
[![dependency status](https://deps.rs/repo/github/py32-rust/py32f0xx-hal/status.svg)](https://deps.rs/repo/github/py32-rust/py32f0xx-hal)
[![Continuous integration](https://github.com/py32-rust/py32f0xx-hal/workflows/Continuous%20integration/badge.svg)](https://github.com/py32-rust/py32f0xx-hal)

[_py32f0xx-hal_](https://github.com/py32-rust/py32f0xx-hal) contains a hardware abstraction layer on top of the peripheral access API for the Puya Semiconductor PY32F0xx family of microcontrollers. These are low-cost ARM Cortex-M0+ based MCUs that offer an excellent alternative to STM32F0xx series.


<p align="center">
    <img src="https://skillicons.dev/icons?i=rust" alt="Tech Stack Icons">
</p>
Collaboration on this crate is highly welcome, as are pull requests!

## Supported Devices

* **py32f030** (py32f030xx4, py32f030xx6, py32f030xx7, py32f030xx8)
* **py32f003** (py32f003xx4, py32f003xx6, py32f003xx8)
* **py32f002a** (py32f002ax5)
* **py32f002b** (py32f002bx5)

## Testing Status

### Tested Devices ✅
- **PY32F003x4** - Fully tested and verified
- **PY32F003x8** - Fully tested and verified

### Next to Test 🧪
The following devices are supported in code but need hardware testing:
- **PY32F030** series (all variants)
- **PY32F002A** series
- **PY32F002B** series

### Recommended Programmer for Testing
For testing untested devices, we recommend the **UNIT Electronics CH552 Multiprotocol Programmer**:
- Repository: [unit_ch552_multiprotocol_programmer](https://github.com/UNIT-Electronics-MX/unit_ch552_multiprotocol_programmer)
- Supports multiple PY32 series chips
- Affordable and reliable solution

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

Comprehensive documentation is available in the `docs/` directory:

```bash
# Build and view documentation locally
./build-docs.sh serve
```

The documentation includes:
- **Getting Started Guide** - Setup and first program
- **Hardware Setup** - Wiring and connections  
- **Working Examples** - Detailed code explanations
- **Peripheral Drivers** - USART, GPIO, ADC, Timers, etc.
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

### Using py32f0xx-hal in Your Project

To use py32f0xx-hal as a dependency in a standalone project, specify the target device feature in your `Cargo.toml`:

```toml
[dependencies]
embedded-hal = "1"
nb = "1"
cortex-m = "0.7.7"
cortex-m-rt = "0.7.3"
# Panic behaviour, see https://crates.io/keywords/panic-impl for alternatives
panic-halt = "0.2.0"
py32f0xx-hal = { version = "0.4.0", features = ["py32f003xx4"] }
```

### Device Features
Choose the appropriate feature for your target device:
- `py32f030xx4` - PY32F030 with 16KB Flash
- `py32f030xx6` - PY32F030 with 32KB Flash  
- `py32f030xx7` - PY32F030 with 48KB Flash
- `py32f030xx8` - PY32F030 with 64KB Flash
- `py32f003xx4` - PY32F003 with 16KB Flash
- `py32f003xx6` - PY32F003 with 32KB Flash
- `py32f003xx8` - PY32F003 with 64KB Flash
- `py32f002ax5` - PY32F002A with 20KB Flash
- `py32f002bx5` - PY32F002B with 20KB Flash

## Optional Features

- **rtic** - Includes a `monotonic` timer module for use with the RTIC framework
- **defmt** - Adds `derive(defmt::Format)` to `Error` types for better debugging
- **rt** - Enables the `rt` feature in the `py32f0xx` PAC crate for runtime support

## Learning Resources

If you are unfamiliar with embedded development using Rust, here are some excellent resources:

- [Embedded Rust Documentation](https://docs.rust-embedded.org/)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [Rust Embedded FAQ](https://docs.rust-embedded.org/faq.html)
- [rust-embedded/awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)


## Minimum Supported Rust Version

The minimum supported Rust version is the latest stable release. Older versions may compile, especially when some features are not used in your application.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## Credits

This repository was inspired by [stm32f0xx-hal](https://github.com/stm32-rs/stm32f0xx-hal) and [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal).

Forked from [py32-rust/py32f0xx-hal](https://github.com/py32-rust/py32f0xx-hal).

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

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
