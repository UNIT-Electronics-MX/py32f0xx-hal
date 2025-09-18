
##### Project Configuration #####

# Build type: rust or c
# Use 'rust' for Rust HAL examples, 'c' for traditional C projects
BUILD_TYPE ?= rust

# For C projects - Project name determines which folder contains the source code
PROJECT ?= pwm

# For Rust projects - Example name (from examples/ directory)
EXAMPLE ?= blinky

# The path for generated files
BUILD_DIR		= Build

# MCU types: 
#   PY32F002Ax5
#   PY32F002Bx5
#   PY32F003x4, PY32F003x6, PY32F003x8,
#   PY32F030x6, PY32F030x8, 
#   PY32F072xB
MCU_TYPE		= PY32F003x4

##### Rust Configuration #####

# Rust target
RUST_TARGET		= thumbv6m-none-eabi
# Rust features for different MCU types
RUST_FEATURES	= py32f003,rt

# Virtual environment path for pyocd
VENV_PATH		?= venv
# Python executable in virtual environment
PYTHON_VENV		= $(VENV_PATH)/bin/python
# PyOCD executable in virtual environment
PYOCD_VENV		= $(VENV_PATH)/bin/pyocd

##### Options #####

# Use LL library instead of HAL, y:yes, n:no (for C projects)
USE_LL_LIB ?= n
# Enable printf float %f support, y:yes, n:no (for C projects)
ENABLE_PRINTF_FLOAT	?= n
# Build with FreeRTOS, y:yes, n:no (for C projects)
USE_FREERTOS	?= n
# Build with CMSIS DSP functions, y:yes, n:no (for C projects)
USE_DSP			?= y
# Build with Waveshare e-paper lib, y:yes, n:no (for C projects)
USE_EPAPER		?= n
# Programmer, jlink or pyocd
FLASH_PROGRM	?= pyocd

##### Toolchains #######
# linux
ARM_TOOLCHAIN	?= /usr/bin
# windows change to your path of ARM GCC <- C:/ARM_GCC/ saved in environment variable
# ARM_TOOLCHAIN ?= C:/ARM_GCC/bin

# path to JLinkExe
JLINKEXE		?= /opt/SEGGER/JLink/JLinkExe
# path to PyOCD (system-wide)
PYOCD_EXE		?= pyocd

##### Paths ############

# C and CPP source folders
CDIRS		:= $(PROJECT)
# Single C and CPP source files
CFILES		:= 
CPPFILES	:= 

# ASM source folders
ADIRS		:= $(PROJECT)
# Single ASM source files
AFILES		:= 

# Include paths
INCLUDES	:= Libraries/CMSIS/Core/Include \
			Libraries/CMSIS/Device/PY32F0xx/Include \
			$(CDIRS)

##### Library Paths ############

# Library flags
LIB_FLAGS		= $(MCU_TYPE)
# JLink device (Uppercases)
JLINK_DEVICE	?= $(shell echo $(MCU_TYPE) | tr '[:lower:]' '[:upper:]')
# PyOCD device (Lowercases)
PYOCD_DEVICE	?= $(shell echo $(MCU_TYPE) | tr '[:upper:]' '[:lower:]')
# Link descript file: 
LDSCRIPT		= Libraries/LDScripts/$(PYOCD_DEVICE).ld


ifneq (,$(findstring PY32F002B,$(MCU_TYPE)))

# PY32F002B >>>
CFILES		+= Libraries/CMSIS/Device/PY32F0xx/Source/system_py32f002b.c

ifeq ($(USE_LL_LIB),y)
CDIRS		+= Libraries/PY32F002B_LL_Driver/Src \
		Libraries/PY32F002B_LL_BSP/Src
INCLUDES	+= Libraries/PY32F002B_LL_Driver/Inc \
		Libraries/PY32F002B_LL_BSP/Inc
LIB_FLAGS   += USE_FULL_LL_DRIVER
else
CDIRS		+= Libraries/PY32F002B_HAL_Driver/Src \
		Libraries/PY32F002B_HAL_BSP/Src
INCLUDES	+= Libraries/PY32F002B_HAL_Driver/Inc \
		Libraries/PY32F002B_HAL_BSP/Inc
endif
# Startup file
AFILES	:= Libraries/CMSIS/Device/PY32F0xx/Source/gcc/startup_py32f002b.s
# PY32F002B <<<

else ifneq (,$(findstring PY32F07,$(MCU_TYPE)))

#  PY32F07x >>>
CFILES		+= Libraries/CMSIS/Device/PY32F0xx/Source/system_py32f07x.c

CDIRS		+= Libraries/PY32F07x_HAL_Driver/Src \
		Libraries/PY32F07x_HAL_BSP/Src
INCLUDES	+= Libraries/PY32F07x_HAL_Driver/Inc \
		Libraries/PY32F07x_HAL_BSP/Inc
LIB_FLAGS   += USE_HAL_DRIVER
# Startup file
AFILES	:= Libraries/CMSIS/Device/PY32F0xx/Source/gcc/startup_py32f072.s
#  PY32F07 <<<

else

# PY32F002A,003,030 >>>
CFILES		+= Libraries/CMSIS/Device/PY32F0xx/Source/system_py32f0xx.c

ifeq ($(USE_LL_LIB),y)
CDIRS		+= Libraries/PY32F0xx_LL_Driver/Src \
		Libraries/PY32F0xx_LL_BSP/Src
INCLUDES	+= Libraries/PY32F0xx_LL_Driver/Inc \
		Libraries/PY32F0xx_LL_BSP/Inc
LIB_FLAGS   += USE_FULL_LL_DRIVER
else
CDIRS		+= Libraries/PY32F0xx_HAL_Driver/Src \
		Libraries/PY32F0xx_HAL_BSP/Src
INCLUDES	+= Libraries/PY32F0xx_HAL_Driver/Inc \
		Libraries/PY32F0xx_HAL_BSP/Inc
endif
# Startup file
ifneq (,$(findstring PY32F002A,$(LIB_FLAGS)))
AFILES	:= Libraries/CMSIS/Device/PY32F0xx/Source/gcc/startup_py32f002a.s
endif
ifneq (,$(findstring PY32F003,$(LIB_FLAGS)))
AFILES	:= Libraries/CMSIS/Device/PY32F0xx/Source/gcc/startup_py32f003.s
endif
ifneq (,$(findstring PY32F030,$(LIB_FLAGS)))
AFILES	:= Libraries/CMSIS/Device/PY32F0xx/Source/gcc/startup_py32f030.s
endif
# PY32F002A,003,030 <<<

endif

######## Additional Libs ########

ifeq ($(USE_FREERTOS),y)
CDIRS		+= Libraries/FreeRTOS \
			Libraries/FreeRTOS/portable/GCC/ARM_CM0

CFILES		+= Libraries/FreeRTOS/portable/MemMang/heap_4.c

INCLUDES	+= Libraries/FreeRTOS/include \
			Libraries/FreeRTOS/portable/GCC/ARM_CM0
endif

ifeq ($(USE_DSP),y)
CFILES 		+= Libraries/CMSIS/DSP/Source/BasicMathFunctions/BasicMathFunctions.c \
		Libraries/CMSIS/DSP/Source/BayesFunctions/BayesFunctions.c \
		Libraries/CMSIS/DSP/Source/CommonTables/CommonTables.c \
		Libraries/CMSIS/DSP/Source/ComplexMathFunctions/ComplexMathFunctions.c \
		Libraries/CMSIS/DSP/Source/ControllerFunctions/ControllerFunctions.c \
		Libraries/CMSIS/DSP/Source/DistanceFunctions/DistanceFunctions.c \
		Libraries/CMSIS/DSP/Source/FastMathFunctions/FastMathFunctions.c \
		Libraries/CMSIS/DSP/Source/FilteringFunctions/FilteringFunctions.c \
		Libraries/CMSIS/DSP/Source/InterpolationFunctions/InterpolationFunctions.c \
		Libraries/CMSIS/DSP/Source/MatrixFunctions/MatrixFunctions.c \
		Libraries/CMSIS/DSP/Source/QuaternionMathFunctions/QuaternionMathFunctions.c \
		Libraries/CMSIS/DSP/Source/StatisticsFunctions/StatisticsFunctions.c \
		Libraries/CMSIS/DSP/Source/SupportFunctions/SupportFunctions.c \
		Libraries/CMSIS/DSP/Source/SVMFunctions/SVMFunctions.c \
		Libraries/CMSIS/DSP/Source/TransformFunctions/TransformFunctions.c
INCLUDES	+= Libraries/CMSIS/DSP/Include \
		Libraries/CMSIS/DSP/PrivateInclude
endif

ifeq ($(USE_EPAPER),y)
CDIRS		+= Libraries/EPaper/Lib \
			Libraries/EPaper/Examples \
			Libraries/EPaper/Fonts \
			Libraries/EPaper/GUI

INCLUDES	+= Libraries/EPaper/Lib \
			Libraries/EPaper/Examples \
			Libraries/EPaper/Fonts \
			Libraries/EPaper/GUI
endif

##### Conditional Build System #####

ifeq ($(BUILD_TYPE),rust)
# Rust build system
include ./rust.mk
else
# Traditional C build system  
include ./rules.mk
endif

##### Additional Rust-specific rules #####

# Setup Python virtual environment for pyocd
.PHONY: setup-venv
setup-venv:
	@echo "Setting up Python virtual environment for pyocd..."
	@python3 -m venv $(VENV_PATH)
	@$(PYTHON_VENV) -m pip install --upgrade pip
	@$(PYTHON_VENV) -m pip install pyocd
	@echo "Virtual environment setup complete at $(VENV_PATH)"

# Check if virtual environment exists
check-venv:
	@if [ ! -f $(PYOCD_VENV) ]; then \
		echo "Virtual environment not found. Run 'make setup-venv' first."; \
		exit 1; \
	fi

# Show available Rust examples
.PHONY: list-examples
list-examples:
	@echo "Available Rust examples:"
	@ls -1 examples/*.rs | sed 's/examples\///g' | sed 's/\.rs$$//g' | sort

# Show MCU features mapping
.PHONY: show-features
show-features:
	@echo "MCU Type to Rust Features mapping:"
	@echo "PY32F002A -> py32f002a,rt"
	@echo "PY32F002B -> py32f002b,rt" 
	@echo "PY32F003  -> py32f003,rt"
	@echo "PY32F030  -> py32f030,rt"
	@echo "PY32F072  -> py32f072,rt"
	@echo ""
	@echo "Current MCU_TYPE: $(MCU_TYPE)"
	@echo "Current Features: $(RUST_FEATURES)"

# Help target
.PHONY: help
help:
	@echo ""
	@echo "PY32F0xx-HAL - Quick Commands"
	@echo "============================="
	@echo ""
	@echo "Quick Examples:"
	@echo "  make blinky        Build blinky"
	@echo "  make flash-blinky  Flash blinky"
	@echo "  make serial_echo   Build serial_echo"
	@echo "  make flash-serial_echo Flash serial_echo"
	@echo "  make i2c_find_address Build I2C scanner"
	@echo "  make flash-i2c_find_address Flash I2C scanner"
	@echo "  make example=NAME  Build any example"
	@echo ""
	@echo "Debug Commands:"
	@echo "  make debug-blinky      Build blinky for debugging"
	@echo "  make debug-serial_echo Build serial_echo for debugging"
	@echo ""
	@echo "PyOCD Rescue Commands (when PyOCD gets stuck):"
	@echo "  make flash-no-reset    Flash without reset (avoids timeout)"
	@echo "  make flash-kill-reset  Kill PyOCD processes and flash"
	@echo "  make flash-emergency   Emergency flash (kills all + no reset)"
	@echo ""
	@echo "More Commands:"
	@echo "  make clean         Clean build"
	@echo "  make size          Show binary size"
	@echo "  make docs          Build documentation"
	@echo "  make list-examples List all examples"
	@echo "  make full-help     Show complete help"
	@echo ""
	@echo "Environment Setup:"
	@echo "  make setup-venv    Setup Python virtual environment with pyocd"
	@echo ""
	@echo "Configuration:"
	@echo "  EXAMPLE=$(EXAMPLE)       Current Rust example"
	@echo "  MCU_TYPE=$(MCU_TYPE)     Current MCU type"
	@echo "  FLASH_PROGRM=$(FLASH_PROGRM)   Flash programmer (pyocd/jlink)"
	@echo ""
	@echo "Examples:"
	@echo "  make EXAMPLE=blinky                    Build blinky example"
	@echo "  make EXAMPLE=pwm MCU_TYPE=PY32F030x6   Build PWM for PY32F030"
	@echo "  make flash EXAMPLE=serial_echo         Flash serial echo example"
	@echo ""
	@echo "Shorthand Commands:"
	@echo "  make blinky                            Build blinky example"
	@echo "  make flash-blinky                      Flash blinky example"
	@echo "  make quick-blinky                      Quick flash blinky (PY32F003x4)"
	@echo "  make example=blinky                    Alternative syntax"
	@echo "  make example=serial_echo flash         Alternative with flash"

##### Shorthand Commands #####

# Shorthand commands for common examples
.PHONY: blinky serial_echo serial_adc pwm adc_values i2c_find_address
blinky:
	@echo "Building blinky example..."
	$(MAKE) EXAMPLE=blinky

serial_echo:
	@echo "Building serial_echo example..."
	$(MAKE) EXAMPLE=serial_echo

serial_adc:
	@echo "Building serial_adc example..."
	$(MAKE) EXAMPLE=serial_adc

pwm:
	@echo "Building pwm example..."
	$(MAKE) EXAMPLE=pwm

adc_values:
	@echo "Building adc_values example..."
	$(MAKE) EXAMPLE=adc_values

i2c_find_address:
	@echo "Building i2c_find_address example..."
	$(MAKE) EXAMPLE=i2c_find_address

# Shorthand flash commands
.PHONY: flash-blinky flash-serial_echo flash-serial_adc flash-pwm flash-adc_values flash-i2c_find_address
flash-blinky:
	@echo "Flashing blinky example..."
	$(MAKE) flash EXAMPLE=blinky

flash-serial_echo:
	@echo "Flashing serial_echo example..."
	$(MAKE) flash EXAMPLE=serial_echo

flash-serial_adc:
	@echo "Flashing serial_adc example..."
	$(MAKE) flash EXAMPLE=serial_adc

flash-pwm:
	@echo "Flashing pwm example..."
	$(MAKE) flash EXAMPLE=pwm

flash-adc_values:
	@echo "Flashing adc_values example..."
	$(MAKE) flash EXAMPLE=adc_values

flash-i2c_find_address:
	@echo "Flashing i2c_find_address example..."
	$(MAKE) flash EXAMPLE=i2c_find_address

# PyOCD rescue commands for I2C example
.PHONY: flash-i2c-no-reset flash-i2c-emergency
flash-i2c-no-reset:
	@echo "Flashing I2C scanner without reset (avoids PyOCD timeout)..."
	$(MAKE) flash-no-reset EXAMPLE=i2c_find_address

flash-i2c-emergency:
	@echo "Emergency flash I2C scanner (kills processes + no reset)..."
	$(MAKE) flash-emergency EXAMPLE=i2c_find_address

# Quick commands with automatic MCU type detection
.PHONY: quick-blinky quick-serial quick-pwm
quick-blinky:
	@echo "Quick build and flash: blinky (PY32F003x4)"
	$(MAKE) flash EXAMPLE=blinky MCU_TYPE=PY32F003x4

quick-serial:
	@echo "Quick build and flash: serial_echo (PY32F003x4)"
	$(MAKE) flash EXAMPLE=serial_echo MCU_TYPE=PY32F003x4

quick-pwm:
	@echo "Quick build and flash: pwm (PY32F003x4)"
	$(MAKE) flash EXAMPLE=pwm MCU_TYPE=PY32F003x4

# Universal shorthand syntax: make example=NAME [action]
ifneq ($(example),)
EXAMPLE := $(example)
endif

# Support for: make example=blinky flash
.PHONY: example
example:
ifneq ($(example),)
	@echo "Building example: $(example)"
	$(MAKE) EXAMPLE=$(example)
else
	@echo "Usage: make example=EXAMPLE_NAME [flash]"
	@echo "Examples: make example=blinky"
	@echo "         make example=serial_echo flash"
endif

##### Full Help System #####

.PHONY: full-help
full-help:
	@echo ""
	@echo "PY32F0xx-HAL Complete Build System"
	@echo "=================================="
	@echo ""
	@echo "Variables:"
	@echo "  BUILD_TYPE      Build configuration (default: release)"
	@echo "  EXAMPLE         Example to build (required for Rust builds)"
	@echo "  MCU_TYPE        Target MCU (default: PY32F003x4)"
	@echo "  PROGRAMMER      Programming interface (default: pyocd)"
	@echo ""
	@echo "Available MCU Types:"
	@echo "  PY32F002Ax5, PY32F002Bx5, PY32F003x4, PY32F003x6, PY32F003x8"
	@echo "  PY32F030x4, PY32F030x6, PY32F030x7, PY32F030x8, PY32F072xB"
	@echo ""
	@echo "Standard Targets:"
	@echo "  make build         Build the project (default)"
	@echo "  make flash         Build and flash to target"
	@echo "  make size          Show binary size information"
	@echo "  make clean         Clean build artifacts"
	@echo "  make gdb           Start GDB debugging session"
	@echo "  make docs          Build documentation with mdBook"
	@echo "  make list-examples List all available examples"
	@echo ""
	@echo "Advanced Usage:"
	@echo "  make EXAMPLE=blinky MCU_TYPE=PY32F030x6"
	@echo "  make flash EXAMPLE=serial_echo PROGRAMMER=jlink"

##### Documentation Build #####

.PHONY: docs
docs:
	@echo "Building documentation with mdBook..."
	@if [ -x "./scripts/build-docs.sh" ]; then \
		./scripts/build-docs.sh; \
	else \
		echo "Error: scripts/build-docs.sh not found or not executable"; \
		echo "Please run: chmod +x scripts/build-docs.sh"; \
		exit 1; \
	fi

##### Debug Build Targets #####

.PHONY: debug-build debug-blinky debug-serial_echo debug-pwm debug-adc_values
debug-build:
	@echo "Building debug version of $(EXAMPLE)..."
	@echo "MCU Type: $(MCU_TYPE)"
	@if [ "$(MCU_TYPE)" = "PY32F003x4" ]; then \
		echo "Features: py32f003xx4,rt"; \
		cargo build --target thumbv6m-none-eabi --example $(EXAMPLE) --features py32f003xx4,rt; \
	else \
		echo "Features: py32f003,rt"; \
		cargo build --target thumbv6m-none-eabi --example $(EXAMPLE) --features py32f003,rt; \
	fi
	@echo "Debug build complete!"
	@echo "Binary: target/thumbv6m-none-eabi/debug/examples/$(EXAMPLE)"

debug-blinky:
	@$(MAKE) debug-build EXAMPLE=blinky

debug-serial_echo:
	@$(MAKE) debug-build EXAMPLE=serial_echo

debug-pwm:
	@$(MAKE) debug-build EXAMPLE=pwm

debug-adc_values:
	@$(MAKE) debug-build EXAMPLE=adc_values
