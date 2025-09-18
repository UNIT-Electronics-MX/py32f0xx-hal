# SCRIPT I2C PARA ESP32-H2 
# Copia y pega línea por línea en tu REPL de MicroPython

from machine import Pin, I2C
import time

# Configuración ESP32-H2
i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)
print("I2C configurado: SCL=Pin22, SDA=Pin12, 100kHz")

# Test 1: Scan nativo con timeout
print("\n=== TEST 1: SCAN NATIVO ===")
print("Escaneando... (esto puede tomar unos segundos)")
try:
    devices = i2c.scan()
    print(f"Dispositivos encontrados: {[hex(d) for d in devices]}")
    if 0x50 in devices:
        print("✅ ¡SLAVE 0x50 DETECTADO!")
    else:
        print("❌ Slave 0x50 no detectado en scan")
        print("(Pero puede funcionar con ping directo)")
except Exception as e:
    print(f"⚠ Scan falló: {e}")
    print("Continuando con tests directos...")

# Test 2: Ping directo a 0x50
print("\n=== TEST 2: PING DIRECTO 0x50 ===")
try:
    i2c.writeto(0x50, b'')
    print("✅ 0x50 RESPONDE AL PING!")
except Exception as e:
    print(f"❌ 0x50 no responde: {e}")

# Test 3: Enviar datos
print("\n=== TEST 3: ENVIAR DATOS ===")
test_data = [b'\x42', b'\x01\x02', b'\xAA\xBB\xCC']
for i, data in enumerate(test_data):
    try:
        i2c.writeto(0x50, data)
        print(f"✅ Envío {i+1}: {data} → OK")
        time.sleep_ms(100)
    except Exception as e:
        print(f"❌ Envío {i+1}: {e}")

# Test 4: Loop de monitoreo
print("\n=== TEST 4: MONITOREO CONTINUO ===")
print("Enviando pings cada segundo... (Ctrl+C para parar)")
counter = 0
try:
    while True:
        counter += 1
        try:
            i2c.writeto(0x50, bytes([counter & 0xFF]))
            print(f"Ping #{counter}: ✅")
        except:
            print(f"Ping #{counter}: ❌")
        time.sleep(1)
except KeyboardInterrupt:
    print("\nMonitoreo detenido")
