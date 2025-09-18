# I2C TEST PASO A PASO - ESP32-H2
# Ejecuta cada sección por separado para evitar bloqueos

from machine import Pin, I2C
import time

print("=== CONFIGURACIÓN I2C ===")
i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)
print("I2C configurado: SCL=Pin22, SDA=Pin12, 100kHz")
print("✅ Configuración OK")

print("\n--- EJECUTAR CADA SECCIÓN POR SEPARADO ---")
print("1. Para test directo 0x50:")
print("   exec(open('test_direct.py').read())")
print("\n2. Para scan seguro:")  
print("   exec(open('test_scan.py').read())")
print("\n3. Para envío de datos:")
print("   exec(open('test_data.py').read())")

# Crear archivos de test individuales
print("\nCreando archivos de test...")

# Test directo
with open('test_direct.py', 'w') as f:
    f.write('''
print("=== TEST DIRECTO 0x50 ===")
try:
    i2c.writeto(0x50, b'')
    print("✅ 0x50 RESPONDE!")
    
    # Enviar algunos bytes
    i2c.writeto(0x50, b'\\x42')
    print("✅ Byte enviado")
    
    i2c.writeto(0x50, b'\\x01\\x02\\x03')
    print("✅ Múltiples bytes enviados")
    
except Exception as e:
    print(f"❌ Error: {e}")
''')

# Test scan
with open('test_scan.py', 'w') as f:
    f.write('''
print("=== SCAN SEGURO ===")
print("Escaneando direcciones específicas...")

# Scan manual más seguro
found = []
test_addrs = [0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52]

for addr in test_addrs:
    try:
        i2c.writeto(addr, b'', False)  # Sin stop automático
        found.append(addr)
        print(f"✅ Encontrado: 0x{addr:02X}")
    except:
        pass

if found:
    print(f"Dispositivos: {[hex(d) for d in found]}")
else:
    print("No se encontraron dispositivos en scan manual")
    
# Scan nativo (puede fallar)
try:
    native = i2c.scan()  
    print(f"Scan nativo: {[hex(d) for d in native]}")
except:
    print("Scan nativo falló")
''')

# Test datos
with open('test_data.py', 'w') as f:
    f.write('''
print("=== ENVÍO DE DATOS ===")
test_data = [
    (b'\\x00', "Byte cero"),
    (b'\\x42', "Byte 0x42"), 
    (b'\\x01\\x02', "Dos bytes"),
    (b'\\xAA\\xBB\\xCC', "Tres bytes")
]

for data, desc in test_data:
    try:
        i2c.writeto(0x50, data)
        print(f"✅ {desc}: {data.hex()} → OK")
        time.sleep_ms(50)
    except Exception as e:
        print(f"❌ {desc}: {e}")
''')

print("✅ Archivos creados")
print("\nUSO:")
print("exec(open('test_direct.py').read())  # Test más confiable")
print("exec(open('test_scan.py').read())    # Scan manual")  
print("exec(open('test_data.py').read())    # Envío datos")
