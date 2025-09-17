
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
	@echo "PY32F0xx-HAL Makefile Help"
	@echo "=========================="
	@echo ""
	@echo "Build Types:"
	@echo "  BUILD_TYPE=rust    Build Rust HAL examples (default)"
	@echo "  BUILD_TYPE=c       Build traditional C projects"
	@echo ""
	@echo "Rust Commands:"
	@echo "  make               Build current Rust example ($(EXAMPLE))"
	@echo "  make flash         Flash current Rust example via $(FLASH_PROGRM)"
	@echo "  make clean         Clean Rust build artifacts"
	@echo "  make check         Check Rust code without building"
	@echo "  make clippy        Run Rust linter"
	@echo "  make size          Show size of built binary"
	@echo "  make list-examples Show available examples"
	@echo "  make show-features Show MCU feature mapping"
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
