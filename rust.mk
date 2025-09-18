##### Rust Build Rules #####

# Set MCU-specific features
ifeq ($(findstring PY32F002A,$(MCU_TYPE)),PY32F002A)
    RUST_FEATURES = py32f002a,rt
    PYOCD_TARGET = py32f002ax5
else ifeq ($(findstring PY32F002B,$(MCU_TYPE)),PY32F002B)
    RUST_FEATURES = py32f002b,rt
    PYOCD_TARGET = py32f002bx5
else ifeq ($(findstring PY32F003,$(MCU_TYPE)),PY32F003)
    ifeq ($(MCU_TYPE),PY32F003x4)
        RUST_FEATURES = py32f003xx4,rt
        PYOCD_TARGET = py32f003x4
    else ifeq ($(MCU_TYPE),PY32F003x6)
        RUST_FEATURES = py32f003xx6,rt
        PYOCD_TARGET = py32f003x6
    else ifeq ($(MCU_TYPE),PY32F003x8)
        RUST_FEATURES = py32f003xx8,rt
        PYOCD_TARGET = py32f003x8
    else
        RUST_FEATURES = py32f003xx4,rt
        PYOCD_TARGET = py32f003x4
    endif
else ifeq ($(findstring PY32F030,$(MCU_TYPE)),PY32F030)
    RUST_FEATURES = py32f030,rt
    PYOCD_TARGET = py32f030x8
else ifeq ($(findstring PY32F072,$(MCU_TYPE)),PY32F072)
    RUST_FEATURES = py32f072,rt
    PYOCD_TARGET = py32f072xb
else
    RUST_FEATURES = py32f003,rt
    PYOCD_TARGET = py32f003x4
endif

# Build directories and files
TARGET_DIR = target
RUST_BINARY = $(TARGET_DIR)/$(RUST_TARGET)/release/examples/$(EXAMPLE)
RUST_HEX = $(BUILD_DIR)/$(EXAMPLE).hex
RUST_BIN = $(BUILD_DIR)/$(EXAMPLE).bin
RUST_ELF = $(BUILD_DIR)/$(EXAMPLE).elf

# Create build directory
$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)

# Default target - build the current example
.PHONY: build
build: $(BUILD_DIR)
	@echo "Building Rust example: $(EXAMPLE)"
	@echo "MCU Type: $(MCU_TYPE)"
	@echo "Features: $(RUST_FEATURES)"
	@echo "Target: $(RUST_TARGET)"
	cargo build --release --target $(RUST_TARGET) --example $(EXAMPLE) --features $(RUST_FEATURES)
	@echo "Converting to hex and bin formats..."
	arm-none-eabi-objcopy -O ihex $(RUST_BINARY) $(RUST_HEX)
	arm-none-eabi-objcopy -O binary $(RUST_BINARY) $(RUST_BIN)
	cp $(RUST_BINARY) $(RUST_ELF)
	@echo "Build complete!"
	@echo "ELF: $(RUST_ELF)"
	@echo "HEX: $(RUST_HEX)"
	@echo "BIN: $(RUST_BIN)"

# Default make target
all: build

# Check Rust code without building
.PHONY: check
check:
	@echo "Checking Rust code..."
	cargo check --target $(RUST_TARGET) --example $(EXAMPLE) --features $(RUST_FEATURES)

# Run Rust linter
.PHONY: clippy
clippy:
	@echo "Running Rust clippy..."
	cargo clippy --target $(RUST_TARGET) --example $(EXAMPLE) --features $(RUST_FEATURES) -- -D warnings

# Show size of the built binary
.PHONY: size
size: build
	@echo "Size information for $(EXAMPLE):"
	arm-none-eabi-size $(RUST_ELF)

# Reset MCU by software
.PHONY: reset
reset: check-venv
	@echo "Resetting MCU..."
	@$(PWD)/tools/reset_mcu.sh

# Flash using PyOCD in virtual environment
.PHONY: flash-pyocd-venv
flash-pyocd-venv: build check-venv
	@echo "Flashing $(EXAMPLE) using PyOCD from virtual environment..."
	@echo "Target: $(PYOCD_TARGET)"
	cd tools/Misc && $(PWD)/$(PYOCD_VENV) flash -t $(PYOCD_TARGET) --config pyocd.yaml ../../$(RUST_HEX)

# Flash using PyOCD in virtual environment with automatic reset
.PHONY: flash-pyocd-venv-reset
flash-pyocd-venv-reset: build check-venv
	@echo "Flashing $(EXAMPLE) using PyOCD from virtual environment with reset..."
	@echo "Target: $(PYOCD_TARGET)"
	cd tools/Misc && $(PWD)/$(PYOCD_VENV) flash -t $(PYOCD_TARGET) --config pyocd.yaml ../../$(RUST_HEX)
	@echo "Performing software reset..."
	@$(PWD)/tools/reset_mcu.sh

# Flash using system PyOCD
.PHONY: flash-pyocd-system  
flash-pyocd-system: build
	@echo "Flashing $(EXAMPLE) using system PyOCD..."
	@echo "Target: $(PYOCD_TARGET)"
	cd tools/Misc && $(PYOCD_EXE) flash -t $(PYOCD_TARGET) --config pyocd.yaml ../../$(RUST_HEX)

# Flash without reset (when PyOCD gets stuck)
.PHONY: flash-no-reset
flash-no-reset: build
	@echo "Flashing $(EXAMPLE) using PyOCD WITHOUT reset..."
	@echo "Target: $(PYOCD_TARGET)"
	@echo "This avoids the reset timeout issue"
	cd tools/Misc && $(PWD)/$(PYOCD_VENV) flash -t $(PYOCD_TARGET) --config pyocd.yaml ../../$(RUST_HEX)
	@echo "Flash complete - NO RESET performed"

# Kill PyOCD processes and flash (when PyOCD gets stuck)
.PHONY: flash-kill-reset
flash-kill-reset: build
	@echo "Killing PyOCD processes and flashing..."
	@pkill -f pyocd || true
	@pkill -f gdb-server || true  
	@sleep 2
	@echo "Flashing $(EXAMPLE) after killing PyOCD processes..."
	@echo "Target: $(PYOCD_TARGET)"
	cd tools/Misc && $(PWD)/$(PYOCD_VENV) flash -t $(PYOCD_TARGET) --config pyocd.yaml ../../$(RUST_HEX)
	@echo "Flash complete"

# Emergency flash (kills processes, waits, then flashes without reset)
.PHONY: flash-emergency
flash-emergency: build
	@echo "=== EMERGENCY FLASH MODE ==="
	@echo "Killing all PyOCD and debugging processes..."
	@pkill -f pyocd || true
	@pkill -f gdb-server || true
	@pkill -f openocd || true
	@pkill -f jlink || true
	@sleep 3
	@echo "Flashing $(EXAMPLE) in emergency mode..."
	@echo "Target: $(PYOCD_TARGET)"
	cd tools/Misc && $(PWD)/$(PYOCD_VENV) flash -t $(PYOCD_TARGET) --config pyocd.yaml ../../$(RUST_HEX)
	@echo "=== EMERGENCY FLASH COMPLETE ==="

# Flash using JLink
.PHONY: flash-jlink
flash-jlink: build
	@echo "Flashing $(EXAMPLE) using JLink..."
	@echo "Device: $(JLINK_DEVICE)"
	@echo "halt\nloadfile $(RUST_HEX)\nr\ng\nexit\n" > $(BUILD_DIR)/jlink_script
	$(JLINKEXE) -device $(JLINK_DEVICE) -if SWD -speed 4000 -CommanderScript $(BUILD_DIR)/jlink_script

# Main flash target - chooses method based on FLASH_PROGRM
.PHONY: flash
flash:
ifeq ($(FLASH_PROGRM),pyocd)
	@if [ -f $(PYOCD_VENV) ]; then \
		$(MAKE) flash-pyocd-venv-reset; \
	else \
		echo "Virtual environment not found, using system PyOCD..."; \
		$(MAKE) flash-pyocd-system; \
	fi
else ifeq ($(FLASH_PROGRM),jlink)
	$(MAKE) flash-jlink
else
	@echo "Unknown flash programmer: $(FLASH_PROGRM)"
	@echo "Use FLASH_PROGRM=pyocd or FLASH_PROGRM=jlink"
	@exit 1
endif

# Flash without automatic reset (original behavior)
.PHONY: flash-no-reset
flash-no-reset:
ifeq ($(FLASH_PROGRM),pyocd)
	@if [ -f $(PYOCD_VENV) ]; then \
		$(MAKE) flash-pyocd-venv; \
	else \
		echo "Virtual environment not found, using system PyOCD..."; \
		$(MAKE) flash-pyocd-system; \
	fi
else ifeq ($(FLASH_PROGRM),jlink)
	$(MAKE) flash-jlink
else
	@echo "Unknown flash programmer: $(FLASH_PROGRM)"
	@echo "Use FLASH_PROGRM=pyocd or FLASH_PROGRM=jlink"
	@exit 1
endif

# Debug using PyOCD in virtual environment
.PHONY: debug-venv
debug-venv: build check-venv
	@echo "Starting debug session with PyOCD from virtual environment..."
	cd tools/Misc && $(PWD)/$(PYOCD_VENV) gdbserver -t $(PYOCD_TARGET) --config pyocd.yaml

# Debug using system PyOCD
.PHONY: debug-system
debug-system: build
	@echo "Starting debug session with system PyOCD..."
	cd tools/Misc && $(PYOCD_EXE) gdbserver -t $(PYOCD_TARGET) --config pyocd.yaml

# Main debug target
.PHONY: debug
debug:
	@if [ -f $(PYOCD_VENV) ]; then \
		$(MAKE) debug-venv; \
	else \
		echo "Virtual environment not found, using system PyOCD..."; \
		$(MAKE) debug-system; \
	fi

# Clean Rust build artifacts
.PHONY: clean
clean:
	@echo "Cleaning Rust build artifacts..."
	cargo clean
	rm -rf $(BUILD_DIR)

# Clean everything including virtual environment
.PHONY: clean-all
clean-all: clean
	@echo "Cleaning virtual environment..."
	rm -rf $(VENV_PATH)

# Format Rust code
.PHONY: format
format:
	@echo "Formatting Rust code..."
	cargo fmt

# Run all examples in check mode
.PHONY: check-all
check-all:
	@echo "Checking all examples..."
	@for example in $(shell ls examples/*.rs | sed 's/examples\///g' | sed 's/\.rs$$//g'); do \
		echo "Checking $$example..."; \
		cargo check --target $(RUST_TARGET) --example $$example --features $(RUST_FEATURES) || exit 1; \
	done

# Build all examples
.PHONY: build-all
build-all:
	@echo "Building all examples..."
	@for example in $(shell ls examples/*.rs | sed 's/examples\///g' | sed 's/\.rs$$//g'); do \
		echo "Building $$example..."; \
		$(MAKE) build EXAMPLE=$$example || exit 1; \
	done

# Show information about current configuration
.PHONY: info
info:
	@echo "Current Configuration:"
	@echo "====================="
	@echo "Build Type: $(BUILD_TYPE)"
	@echo "Example: $(EXAMPLE)"
	@echo "MCU Type: $(MCU_TYPE)"
	@echo "Rust Target: $(RUST_TARGET)"
	@echo "Rust Features: $(RUST_FEATURES)"
	@echo "PyOCD Target: $(PYOCD_TARGET)"
	@echo "Flash Programmer: $(FLASH_PROGRM)"
	@echo ""
	@echo "Paths:"
	@echo "Virtual Env: $(VENV_PATH)"
	@echo "PyOCD (venv): $(PYOCD_VENV)"
	@echo "PyOCD (system): $(PYOCD_EXE)"
	@echo ""
	@echo "Generated Files:"
	@echo "ELF: $(RUST_ELF)"
	@echo "HEX: $(RUST_HEX)"  
	@echo "BIN: $(RUST_BIN)"
