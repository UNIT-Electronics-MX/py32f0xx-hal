#!/bin/bash

# Setup script for PY32F0xx-HAL Rust development environment
# This script configures the development environment for PY32F0xx microcontrollers

set -e

echo "=== PY32F0xx-HAL Setup Script ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "src" ]; then
    echo -e "${RED}Error: This script must be run from the py32f0xx-hal project root directory${NC}"
    exit 1
fi

echo -e "${BLUE}Checking prerequisites...${NC}"

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Rust is not installed. Please install Rust from https://rustup.rs/${NC}"
    exit 1
else
    echo -e "${GREEN}✓ Rust found: $(rustc --version)${NC}"
fi

# Check for Python3
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}Python3 is not installed. Please install Python3${NC}"
    exit 1
else
    echo -e "${GREEN}✓ Python3 found: $(python3 --version)${NC}"
fi

# Check for ARM toolchain
if ! command -v arm-none-eabi-gcc &> /dev/null; then
    echo -e "${YELLOW}Warning: ARM toolchain (arm-none-eabi-gcc) not found in PATH${NC}"
    echo -e "${YELLOW}You may need to install it or update your PATH${NC}"
else
    echo -e "${GREEN}✓ ARM toolchain found: $(arm-none-eabi-gcc --version | head -n1)${NC}"
fi

echo ""
echo -e "${BLUE}Setting up Rust target...${NC}"

# Add thumbv6m-none-eabi target if not already installed
if ! rustup target list --installed | grep -q "thumbv6m-none-eabi"; then
    echo "Adding thumbv6m-none-eabi target..."
    rustup target add thumbv6m-none-eabi
    echo -e "${GREEN}✓ thumbv6m-none-eabi target added${NC}"
else
    echo -e "${GREEN}✓ thumbv6m-none-eabi target already installed${NC}"
fi

echo ""
echo -e "${BLUE}Setting up Python virtual environment for PyOCD...${NC}"

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "Creating Python virtual environment..."
    python3 -m venv venv
    echo -e "${GREEN}✓ Virtual environment created${NC}"
else
    echo -e "${GREEN}✓ Virtual environment already exists${NC}"
fi

# Activate virtual environment and install packages
echo "Installing PyOCD in virtual environment..."
venv/bin/python -m pip install --upgrade pip
venv/bin/python -m pip install pyocd

echo -e "${GREEN}✓ PyOCD installed in virtual environment${NC}"

echo ""
echo -e "${BLUE}Verifying installation...${NC}"

# Test PyOCD installation
if venv/bin/pyocd --version &> /dev/null; then
    echo -e "${GREEN}✓ PyOCD working: $(venv/bin/pyocd --version)${NC}"
else
    echo -e "${RED}✗ PyOCD installation failed${NC}"
    exit 1
fi

# Test Rust compilation
echo "Testing Rust compilation..."
if cargo check --target thumbv6m-none-eabi --example blinky --features py32f003,rt &> /dev/null; then
    echo -e "${GREEN}✓ Rust compilation test passed${NC}"
else
    echo -e "${RED}✗ Rust compilation test failed${NC}"
    echo "Please check your Rust installation and target configuration"
fi

echo ""
echo -e "${GREEN}=== Setup Complete! ===${NC}"
echo ""
echo -e "${BLUE}Usage examples:${NC}"
echo "# Show help"
echo "make help"
echo ""
echo "# List available examples"  
echo "make list-examples"
echo ""
echo "# Build an example"
echo "make build EXAMPLE=blinky"
echo "make EXAMPLE=pwm"
echo ""
echo "# Flash an example"
echo "make flash EXAMPLE=blinky"
echo ""
echo "# Build for different MCU"
echo "make EXAMPLE=serial_echo MCU_TYPE=PY32F030x8"
echo ""
echo "# Check code quality"
echo "make check EXAMPLE=watchdog"
echo "make clippy EXAMPLE=adc_values"
echo ""
echo -e "${BLUE}Configuration:${NC}"
echo "Virtual environment: venv/"
echo "PyOCD config: tools/Misc/pyocd.yaml"
echo "Default MCU: PY32F003x4"
echo "Default example: blinky"
echo ""
echo -e "${YELLOW}Note: Connect your PY32F0xx board before flashing${NC}"
