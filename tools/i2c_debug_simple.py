"""
DIAGNÓSTICO I2C SIMPLE - ESP32-H2
Copia y pega en MicroPython REPL
Configuración: SCL=Pin22, SDA=Pin12
"""

from machine import Pin, I2C
import time

# Configuración I2C para ESP32-H2
i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)

print("=== DIAGNÓSTICO I2C DIRECTO ESP32-H2 ===")
print("Configuración:")
print("  SCL: Pin 22 → PB6 del PY32F003")
print("  SDA: Pin 12 → PA10 del PY32F003")
print("  GND: Común")
print("Probando comunicación con 0x50...")
print()

def ping_0x50():
    try:
        i2c.writeto(0x50, b'')
        return True
    except Exception as e:
        print(f"Error: {e}")
        return False

def scan_native():
    devices = i2c.scan()
    print(f"Scan nativo: {[hex(d) for d in devices]}")
    return devices

# Pruebas
print("\n1. Scan nativo MicroPython:")
scan_native()

print("\n2. Ping directo a 0x50:")
if ping_0x50():
    print("✅ 0x50 RESPONDE!")
else:
    print("❌ 0x50 no responde")

print("\n3. Prueba con diferentes métodos:")
methods = [
    ("Vacío", b''),
    ("Un byte", b'\x42'),
    ("Dos bytes", b'\x01\x02'),
]

for name, data in methods:
    try:
        i2c.writeto(0x50, data)
        print(f"✅ {name}: OK")
    except Exception as e:
        print(f"❌ {name}: {e}")

print("\n4. Información I2C:")
print(f"Frecuencia: 100kHz")
print("Pines ESP32-H2: SCL=22, SDA=12")
print("Conexiones:")
print("  Pin 22 (SCL) → PB6 del PY32F003")
print("  Pin 12 (SDA) → PA10 del PY32F003")
print("  GND → GND común")
