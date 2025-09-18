"""
I2C Master Scanner Simple - MicroPython
Copia y pega este código en tu REPL de MicroPython

Para ESP32: Conecta SCL→Pin22, SDA→Pin21
Para ESP8266: Conecta SCL→Pin5, SDA→Pin4  
Para Pico: Conecta SCL→Pin1, SDA→Pin0

Luego conecta al PY32F003:
- Tu_SCL → PB6 del PY32F003
- Tu_SDA → PA10 del PY32F003
- GND → GND común
"""

from machine import Pin, I2C
import time

# ===== CONFIGURACIÓN =====
# Cambia estos pines según tu microcontrolador
SCL_PIN = 22  # Para ESP32
SDA_PIN = 21  # Para ESP32

# Para ESP8266 usa:
# SCL_PIN = 5
# SDA_PIN = 4

# Para Raspberry Pi Pico usa:
# SCL_PIN = 1  
# SDA_PIN = 0

# Crear objeto I2C
i2c = I2C(0, scl=Pin(SCL_PIN), sda=Pin(SDA_PIN), freq=100000)

print("=== I2C Scanner MicroPython ===")
print(f"SCL: Pin {SCL_PIN}")
print(f"SDA: Pin {SDA_PIN}")
print("Conecta al PY32F003:")
print(f"  Pin {SCL_PIN} (SCL) → PB6")
print(f"  Pin {SDA_PIN} (SDA) → PA10")
print("  GND → GND")
print()

def scan_i2c():
    """Escanear direcciones I2C"""
    print("Escaneando I2C...")
    devices = i2c.scan()
    
    if devices:
        print(f"✅ Encontrados {len(devices)} dispositivos:")
        for addr in devices:
            print(f"   0x{addr:02X} ({addr})")
            
        # Verificar si encontramos nuestro slave
        if 0x50 in devices:
            print("🎉 ¡Encontrado slave PY32F003 en 0x50!")
    else:
        print("❌ No se encontraron dispositivos")
        print("Verifica las conexiones y que el PY32F003 esté ejecutando el código slave")
    
    return devices

def ping_slave(addr=0x50):
    """Hacer ping al slave"""
    try:
        i2c.writeto(addr, b'')
        print(f"✅ Ping OK a 0x{addr:02X}")
        return True
    except:
        print(f"❌ Sin respuesta de 0x{addr:02X}")
        return False

def test_slave(addr=0x50):
    """Probar comunicación con slave"""
    print(f"\n=== Probando slave 0x{addr:02X} ===")
    
    # Ping básico
    if not ping_slave(addr):
        return
    
    # Enviar algunos bytes de prueba
    test_bytes = [0x01, 0x42, 0xFF]
    for b in test_bytes:
        try:
            i2c.writeto(addr, bytes([b]))
            print(f"✅ Enviado: 0x{b:02X}")
            time.sleep_ms(100)
        except Exception as e:
            print(f"❌ Error enviando 0x{b:02X}: {e}")

# ===== EJECUCIÓN =====
while True:
    print("\n" + "="*40)
    
    # Escanear bus
    devices = scan_i2c()
    
    # Si encontramos dispositivos, probar el 0x50
    if 0x50 in devices:
        test_slave(0x50)
    
    print("\nEsperando 5 segundos...")
    print("Presiona Ctrl+C para detener")
    
    try:
        time.sleep(5)
    except KeyboardInterrupt:
        print("\n👋 Detenido por usuario")
        break
