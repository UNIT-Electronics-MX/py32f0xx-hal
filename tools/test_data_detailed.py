# TEST DE ENVÍO DE DATOS ESPECÍFICOS AL PY32F003
# Para ver los datos impresos en el monitor serie

from machine import Pin, I2C
import time

# Configurar I2C
i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)
print("=== TEST DE DATOS I2C ===")
print("Enviando datos específicos al PY32F003...")
print("Verifica el monitor serie del PY32F003 para ver los datos recibidos")

# Test 1: Bytes individuales
print("\n1. Enviando bytes individuales...")
test_bytes = [0x00, 0x42, 0x55, 0xAA, 0xFF]

for i, byte_val in enumerate(test_bytes):
    try:
        print(f"   Enviando byte {i+1}: 0x{byte_val:02X} ({byte_val})")
        i2c.writeto(0x50, bytes([byte_val]))
        time.sleep_ms(500)  # Pausa para leer en el monitor
        print("   ✅ Enviado")
    except Exception as e:
        print(f"   ❌ Error: {e}")
        break

print("\n2. Enviando caracteres ASCII...")
ascii_chars = [ord('A'), ord('B'), ord('C'), ord('1'), ord('2'), ord('3')]

for char_val in ascii_chars:
    try:
        print(f"   Enviando ASCII: '{chr(char_val)}' (0x{char_val:02X})")
        i2c.writeto(0x50, bytes([char_val]))
        time.sleep_ms(500)
        print("   ✅ Enviado")
    except Exception as e:
        print(f"   ❌ Error: {e}")
        break

print("\n3. Enviando secuencia de números...")
for num in range(10, 20):
    try:
        print(f"   Enviando número: {num} (0x{num:02X})")
        i2c.writeto(0x50, bytes([num]))
        time.sleep_ms(300)
        print("   ✅ Enviado")
    except Exception as e:
        print(f"   ❌ Error: {e}")
        break

print("\n4. Enviando múltiples bytes en una transacción...")
try:
    multi_data = b'\x01\x02\x03\x04\x05'
    print(f"   Enviando: {multi_data.hex()}")
    i2c.writeto(0x50, multi_data)
    time.sleep_ms(1000)
    print("   ✅ Múltiples bytes enviados")
except Exception as e:
    print(f"   ❌ Error múltiples bytes: {e}")

print("\n=== TEST COMPLETADO ===")
print("Revisa el monitor serie del PY32F003 para ver:")
print("- Cada dato recibido en hexadecimal, decimal y binario")
print("- Caracteres ASCII cuando sea aplicable")
print("- Contadores de transacciones y bytes")
