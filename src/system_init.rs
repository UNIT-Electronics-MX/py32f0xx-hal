//! Módulo de inicialización común para PY32F0xx
//! 
//! Este módulo proporciona funciones de inicialización estándar que todos los ejemplos pueden usar
//! para configurar automáticamente el reloj del sistema y los periféricos básicos.

use crate::pac;
use crate::prelude::*;
use crate::rcc::{RccExt, HSIFreq, Rcc};
use crate::gpio::{
    gpioa::Parts as GpioAParts, 
    gpiob::Parts as GpioBParts,
};
#[cfg(any(feature = "py32f030", feature = "py32f003"))]
use crate::gpio::gpiof::Parts as GpioFParts;

/// Configuración del reloj del sistema
pub struct SystemClockConfig {
    /// Frecuencia HSI objetivo
    pub hsi_freq: HSIFreq,
    /// Frecuencia del sistema en MHz
    pub sysclk_mhz: u32,
}

impl Default for SystemClockConfig {
    fn default() -> Self {
        Self {
            hsi_freq: HSIFreq::Freq8mhz,
            sysclk_mhz: 8,
        }
    }
}

/// Configuración inicial del sistema
pub struct SystemInit {
    /// RCC configurado
    pub rcc: Rcc,
    /// GPIO Port A
    pub gpioa: GpioAParts,
    /// GPIO Port B
    pub gpiob: GpioBParts,
    /// GPIO Port F (solo disponible en py32f030 y py32f003)
    #[cfg(any(feature = "py32f030", feature = "py32f003"))]
    pub gpiof: GpioFParts,
}

impl SystemInit {
    /// Inicializa el sistema con configuración por defecto (8MHz)
    pub fn init() -> Self {
        Self::init_with_config(SystemClockConfig::default())
    }

    /// Inicializa el sistema con configuración de 24MHz (máxima frecuencia)
    pub fn init_24mhz() -> Self {
        Self::init_with_config(SystemClockConfig {
            hsi_freq: HSIFreq::Freq24mhz,
            sysclk_mhz: 24,
        })
    }

    /// Inicializa el sistema con configuración de 16MHz
    pub fn init_16mhz() -> Self {
        Self::init_with_config(SystemClockConfig {
            hsi_freq: HSIFreq::Freq16mhz,
            sysclk_mhz: 16,
        })
    }

    /// Inicializa el sistema con configuración de 4MHz (bajo consumo)
    pub fn init_4mhz() -> Self {
        Self::init_with_config(SystemClockConfig {
            hsi_freq: HSIFreq::Freq4mhz,
            sysclk_mhz: 4,
        })
    }

    /// Inicializa el sistema con configuración personalizada
    pub fn init_with_config(clock_config: SystemClockConfig) -> Self {
        let mut p = pac::Peripherals::take().unwrap();

        // Configurar RCC usando el HAL
        let rcc = p.RCC
            .configure()
            .hsi(clock_config.hsi_freq)
            .sysclk(clock_config.sysclk_mhz.MHz())
            .freeze(&mut p.FLASH);

        // Inicializar todos los GPIO disponibles
        let gpioa = p.GPIOA.split();
        let gpiob = p.GPIOB.split();
        #[cfg(any(feature = "py32f030", feature = "py32f003"))]
        let gpiof = p.GPIOF.split();

        SystemInit {
            rcc,
            gpioa,
            gpiob,
            #[cfg(any(feature = "py32f030", feature = "py32f003"))]
            gpiof,
        }
    }
}

/// Macro para simplificar la inicialización en ejemplos
#[macro_export]
macro_rules! init_system {
    () => {
        py32f0xx_hal::system_init::SystemInit::init()
    };
    (4mhz) => {
        py32f0xx_hal::system_init::SystemInit::init_4mhz()
    };
    (8mhz) => {
        py32f0xx_hal::system_init::SystemInit::init()
    };
    (16mhz) => {
        py32f0xx_hal::system_init::SystemInit::init_16mhz()
    };
    (24mhz) => {
        py32f0xx_hal::system_init::SystemInit::init_24mhz()
    };
    ($config:expr) => {
        py32f0xx_hal::system_init::SystemInit::init_with_config($config)
    };
}
