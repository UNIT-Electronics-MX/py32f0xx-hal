# Changelog

All notable changes to the PY32F0xx HAL project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete mdBook documentation system
- Comprehensive serial communication examples
- English translation of all documentation
- Enhanced README with working examples
- Detailed troubleshooting guides
- Device-specific configuration guides

### Changed
- Improved project structure and organization
- Enhanced example code with better comments
- Standardized clock configuration across examples

### Fixed
- USART2 configuration for DFN8 package
- Serial communication reliability issues
- Documentation inconsistencies

## [0.4.0] - Previous Release

### Added
- Initial serial communication support
- Basic peripheral drivers (GPIO, USART, ADC, SPI, Timers)
- Support for PY32F002A, PY32F002B, PY32F003, PY32F030 families
- Examples for common use cases
- DMA support for F030/F003 series

### Changed
- Updated to embedded-hal 1.0
- Improved error handling
- Enhanced GPIO pin configuration

### Fixed
- Clock configuration issues
- Peripheral initialization problems

## [0.3.0] - Earlier Release

### Added
- Multi-device support
- Enhanced GPIO functionality
- Timer and PWM support
- RTC peripheral driver

### Changed
- Refactored peripheral access patterns
- Improved documentation

### Fixed
- Various peripheral configuration issues

## [0.2.0] - Early Release

### Added
- Basic HAL structure
- GPIO support
- Initial documentation

### Changed
- Project organization

### Fixed
- Initial bug fixes

## [0.1.0] - Initial Release

### Added
- Initial PY32F0xx HAL implementation
- Basic peripheral support
- Project foundation
