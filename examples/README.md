# 📁 Ejemplos del Proyecto PY32F0xx-HAL

## 🚀 Ejemplos Principales (Usar estos)

### **Básicos**
- **`blinky_hal_simple.rs`** - Ejemplo básico de LED parpadeante con inicialización automática
- **`gpio_easy_swap.rs`** - Demostración de intercambio fácil de puertos GPIO  
- **`gpio_multi_port_demo.rs`** - Demo avanzado con múltiples puertos a la vez

### **Originales del HAL**
- **`blinky.rs`** - Ejemplo original de blinky del proyecto
- **`blinky_delay.rs`** - Blinky con delays precisos
- **`blinky_timer.rs`** - Blinky usando timers

### **Periféricos**
- **`adc_values.rs`** - Lectura de valores ADC
- **`serial_echo.rs`** - Echo serial
- **`i2c_find_address.rs`** - Escaneo de dispositivos I2C
- **`spi_hal_apa102c.rs`** - Control de LEDs APA102C via SPI
- **`pwm.rs`** - Generación de PWM

## 🧪 Archivos de Prueba y Desarrollo (`testing/`)

Los siguientes archivos están en `examples/testing/` para mantener el directorio principal limpio:

- `blinky_pa1.rs` - Prueba específica para PA1
- `blinky_working.rs` - Versión de trabajo durante desarrollo  
- `clock_gpio_test.rs` - Pruebas de configuración de reloj
- `diagnostic_direct.rs` - Diagnósticos directos de hardware
- `direct_gpio_*.rs` - Manipulación directa de registros GPIO
- `gpio_test_*.rs` - Varias pruebas de GPIO
- `led_on.rs` - Prueba simple de encendido de LED
- `test_multiple_pins.rs` - Pruebas con múltiples pines

## 🛠️ Sistema de Inicialización Generalizado

Todos los ejemplos nuevos utilizan el sistema de inicialización automática:

```rust
use py32f0xx_hal::system_init::SystemInit;

// Una línea inicializa todo el sistema
let sys = SystemInit::init();

// Intercambiar puertos es fácil - solo cambiar esta línea:
let mut pin = sys.gpiob.pb5.into_push_pull_output();
```

## 📋 Comandos Útiles

```bash
# Compilar ejemplo
make build EXAMPLE=blinky_hal_simple

# Compilar y flashear (con reset automático)  
make flash EXAMPLE=gpio_easy_swap

# Solo reset por software
make reset

# Limpiar archivos de build
make clean

# Ver configuración actual
make info
```

## 🎯 Configuración Automática Incluida

✅ Reloj del sistema (HSI 8MHz por defecto, 24MHz disponible)  
✅ Todos los puertos GPIO inicializados (A, B, F)  
✅ Reset automático después de flashear  
✅ PyOCD desde entorno virtual  
✅ Configuración correcta para PY32F003x4
