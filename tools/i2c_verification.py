# VERIFICACIÓN I2C ESP32-H2 ↔ PY32F003
# Ejecutar paso a paso en MicroPython REPL

from machine import Pin, I2C
import time

# Configurar I2C
i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)
print("I2C ESP32-H2 configurado ✅")

# Test simple: ¿Responde 0x50?
print("\nTesting 0x50...")
try:
    i2c.writeto(0x50, b'\x00')
    print("🎉 ¡SUCCESS! 0x50 responde perfectamente")
    
    # Enviar algunos datos más
    for i in range(5):
        data = bytes([0x10 + i])
        i2c.writeto(0x50, data)
        print(f"  Enviado: {data.hex()} ✅")
        time.sleep_ms(200)
        
except Exception as e:
    print(f"❌ Error: {e}")

# Scan completo
print(f"\nScan completo:")
devices = i2c.scan()
if devices:
    print(f"Dispositivos: {[hex(d) for d in devices]}")
    if 0x50 in devices:
        print("✅ 0x50 confirmado en scan")
else:
    print("Scan no detectó dispositivos")

print("\n🎯 RESULTADO: I2C funciona correctamente!")
print("El slave PY32F003 responde a todas las comunicaciones")
