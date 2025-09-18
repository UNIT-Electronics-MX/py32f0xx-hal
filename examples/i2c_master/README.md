# I2C Master Examples

Esta carpeta contiene ejemplos de I2C master para probar la comunicación con el PY32F003 configurado como slave.

## Archivos Principales

### `esp32h2_i2c_test.py`
Script básico para ESP32-H2 con MicroPython que prueba la comunicación I2C.

**Características:**
- Escaneo de dispositivos I2C
- Envío de datos simples al slave
- Configuración para pines PA10/PB6 del PY32F003

### `i2c_demo_master.py`
Demo completo con múltiples patrones de prueba.

**Características:**
- Pruebas con diferentes tipos de datos
- Patrones ASCII y binarios
- Manejo de errores mejorado
- Logging detallado de transacciones

### `i2c_minimal.py`
Implementación mínima para verificación rápida.

**Características:**
- Código mínimo para pruebas básicas
- Ideal para debugging inicial
- Configuración simplificada

## Configuración Hardware

### ESP32-H2 ↔ PY32F003
```
ESP32-H2    PY32F003    Función
--------    --------    -------
Pin 22   →  PB6        SCL (Clock)
Pin 12   →  PA10       SDA (Data)
GND      →  GND        Común
3.3V     →  VCC        Alimentación
```

### Resistencias Pull-up
Agregar resistencias de 4.7kΩ:
- Entre SDA y VCC
- Entre SCL y VCC

## Uso Rápido

1. **Flashear PY32F003:**
   ```bash
   make flash EXAMPLE=i2c_slave_demo
   ```

2. **Monitorear serial:**
   ```bash
   make monitor
   ```

3. **Ejecutar en ESP32-H2:**
   ```python
   # Cargar script en MicroPython REPL
   exec(open('esp32h2_i2c_test.py').read())
   ```

## Pruebas de Ejemplo

### Envío de Texto
```python
from machine import I2C, Pin

i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)
i2c.writeto(0x50, b'Hola PY32!')
```

### Envío de Datos Binarios
```python
data = bytearray([0x01, 0x02, 0x03, 0xFF])
i2c.writeto(0x50, data)
```

### Bucle de Prueba
```python
import time
for i in range(10):
    message = f'Test {i:02d}'.encode()
    i2c.writeto(0x50, message)
    time.sleep(0.5)
```

## Salida Esperada

### En PY32F003 (Serial Monitor)
```
=== I2C SLAVE DEMO ===
Configuracion: PA10=SDA, PB6=SCL
=== I2C Transaccion #1 ===
Dato #1: 0x48 (72)
ASCII: 'H'
Dato #2: 0x6F (111)
ASCII: 'o'
Dato #3: 0x6C (108)
ASCII: 'l'
Dato #4: 0x61 (97)
ASCII: 'a'
Transaccion completa
```

### En ESP32-H2 (MicroPython)
```
I2C devices found: [80]
Sending to 0x50: Hola
Data sent successfully
```

## Troubleshooting

- **No device found**: Verificar conexiones y pull-ups
- **Data errors**: Revisar dirección slave (0x50)
- **Communication fails**: Confirmar frecuencia I2C (100kHz)
- **Serial issues**: Verificar conexiones USART2 (PA0/PA1)

## Notas de Desarrollo

- Todos los scripts están diseñados para ESP32-H2 con MicroPython
- Frecuencia I2C fijada en 100kHz para comunicación confiable
- Compatible con la implementación I2C slave del PY32F003
- Probado con resistencias pull-up de 4.7kΩ
