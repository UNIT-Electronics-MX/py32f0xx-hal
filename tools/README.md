# Development Tools

This directory contains development and build tools for the PY32F0xx HAL project.

## Available Tools

### `check.py`
Python script for project validation and testing.

### `capture_example_bloat.sh` / `capture_nightly_example_bloat.sh`  
Scripts for analyzing code size and memory usage of examples.

### `reset_mcu.sh`
Script for resetting the MCU during development.

### `openocd/`
OpenOCD configuration files and utilities for debugging and programming.

### `Misc/`
Additional development utilities and configuration files.

## I2C Testing

I2C master test scripts have been moved to `examples/i2c_master/` for better organization.

## Usage

Most tools are used internally by the build system. For I2C testing, see the examples in `examples/i2c_master/`.
