# I2C MASTER DEMO - ESP32-H2
# Script simple para probar el I2C slave PY32F003

from machine import Pin, I2C
import time

print("=== I2C Master Demo ===")
print("ESP32-H2 → PY32F003")
print("Conexiones:")
print("  SCL: Pin 22 → PB6")
print("  SDA: Pin 12 → PA10")
print("  GND → GND")
print()

# Configurar I2C
i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)
print("I2C configurado: 100kHz")

# Demo de envío de datos
demo_data = [
    (b'A', "Letra 'A'"),
    (b'B', "Letra 'B'"),
    (b'\x42', "Byte 0x42"),
    (b'123', "Número 123"),
    (b'\x01\x02\x03', "Secuencia 1-2-3")
]

print("\nEnviando datos al slave PY32F003...")
print("(Verifica el monitor serie del PY32F003)")
print()

for i, (data, desc) in enumerate(demo_data):
    print(f"{i+1}. Enviando {desc}...")
    try:
        i2c.writeto(0x50, data)
        print("   ✅ Enviado correctamente")
    except Exception as e:
        print(f"   ❌ Error: {e}")
    
    time.sleep(2)  # Pausa para leer en monitor serie

print("\n=== Demo completado ===")
print("El slave debería haber mostrado todos los datos recibidos")
print("en formato hexadecimal, decimal, binario y ASCII.")
