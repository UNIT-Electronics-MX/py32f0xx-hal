# NUEVO TEST DESPUÉS DE ACTUALIZACIÓN DE ACK
# El slave ahora maneja mejor el ACK - probémoslo

from machine import Pin, I2C
import time

# Configurar I2C
i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)
print("I2C configurado después de actualización ACK")

# Test 1: Ping simple
print("\n=== TEST 1: PING SIMPLE ===")
try:
    i2c.writeto(0x50, b'')
    print("✅ SUCCESS! 0x50 responde con ACK correcto")
except Exception as e:
    print(f"❌ Error: {e}")

# Test 2: Enviar byte específico
print("\n=== TEST 2: ENVÍO BYTE ===")
try:
    i2c.writeto(0x50, b'\x42')
    print("✅ Byte 0x42 enviado correctamente")
except Exception as e:
    print(f"❌ Error enviando byte: {e}")

# Test 3: Múltiples bytes
print("\n=== TEST 3: MÚLTIPLES BYTES ===")
try:
    i2c.writeto(0x50, b'\x01\x02\x03')
    print("✅ Múltiples bytes enviados")
except Exception as e:
    print(f"❌ Error múltiples bytes: {e}")

# Test 4: Serie de comandos
print("\n=== TEST 4: SERIE DE COMANDOS ===")
for i in range(5):
    try:
        data = bytes([0x10 + i])
        i2c.writeto(0x50, data)
        print(f"✅ Comando {i}: 0x{data[0]:02X} → OK")
        time.sleep_ms(100)
    except Exception as e:
        print(f"❌ Comando {i}: {e}")
        break

print("\n=== RESULTADO ===")
print("Si no hay errores, el ACK está funcionando correctamente!")
print("Verifica el monitor serie del PY32F003 para ver los datos recibidos.")
