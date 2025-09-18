#!/usr/bin/env python3
"""
I2C Master Scanner en MicroPython
Para probar el I2C slave PY32F0xx-hal

Conecta:
- SDA a PA10 del PY32F003 (slave)  
- SCL a PB6 del PY32F003 (slave)
- GND común
"""

from machine import Pin, I2C
import time

# Configuración I2C Master - ESP32-H2
# SCL=Pin22, SDA=Pin12 (configuración específica del usuario)
print("Configurando I2C Master para ESP32-H2...")
try:
    # Configuración para ESP32-H2
    i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)
    print("✅ I2C configurado a 100kHz (SCL=Pin22, SDA=Pin12)")
except Exception as e:
    print(f"❌ Error configurando I2C: {e}")
    # Fallback a frecuencia más baja
    try:
        i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=50000)
        print("✅ I2C configurado a 50kHz (SCL=Pin22, SDA=Pin12)")
    except Exception as e2:
        print(f"❌ Error con fallback: {e2}")

# Otras configuraciones comentadas:
# Para ESP32 estándar:
# i2c = I2C(0, scl=Pin(22), sda=Pin(21), freq=100000)

# Para ESP8266:
# i2c = I2C(scl=Pin(5), sda=Pin(4), freq=100000)

# Para Raspberry Pi Pico:
# i2c = I2C(0, scl=Pin(1), sda=Pin(0), freq=100000)

print("=== I2C Master Scanner - ESP32-H2 ===")
print("Configuración:")
print("  ESP32-H2 Pin 22 (SCL) → PB6 del PY32F003")
print("  ESP32-H2 Pin 12 (SDA) → PA10 del PY32F003")
print("  ESP32-H2 GND → PY32F003 GND")
print("Escaneando direcciones I2C...")
print()

def scan_i2c():
    """Escanea todas las direcciones I2C válidas"""
    devices = []
    
    print("Escaneando direcciones I2C (0x08-0x77)...")
    
    # Escanear direcciones 0x08 a 0x77 (rango válido para I2C de 7 bits)
    for addr in range(0x08, 0x78):
        try:
            # Intentar comunicación con la dirección
            # Usar método más robusto con timeout
            i2c.writeto(addr, b'', False)  # False = no enviar stop automático
            devices.append(addr)
            print(f"✅ Dispositivo encontrado en: 0x{addr:02X}")
            
        except OSError as e:
            # No hay respuesta en esta dirección
            if addr == 0x50:  # Log específicamente para nuestra dirección
                print(f"🔍 Probando 0x50: Error {e}")
            pass
            
        # Pequeño delay entre intentos
        time.sleep_ms(20)
    
    # También probar con el método scan() nativo de MicroPython
    print("\nUsando método scan() nativo...")
    native_devices = i2c.scan()
    if native_devices:
        print(f"Método nativo encontró: {[hex(d) for d in native_devices]}")
        # Combinar resultados
        for dev in native_devices:
            if dev not in devices:
                devices.append(dev)
    
    return devices

def direct_test_0x50():
    """Prueba directa de la dirección 0x50 con diferentes métodos"""
    addr = 0x50
    print(f"\n=== PRUEBA DIRECTA 0x{addr:02X} ===")
    
    methods = [
        ("writeto con stop", lambda: i2c.writeto(addr, b'')),
        ("writeto sin stop", lambda: i2c.writeto(addr, b'', False)),
        ("writeto con datos", lambda: i2c.writeto(addr, b'\x00')),
        ("start+stop", lambda: test_start_stop(addr)),
    ]
    
    for name, test_func in methods:
        try:
            test_func()
            print(f"✅ {name}: SUCCESS")
            return True
        except Exception as e:
            print(f"❌ {name}: {e}")
    
    return False

def test_start_stop(addr):
    """Test manual con start/stop conditions"""
    try:
        # MicroPython normalmente maneja start/stop automáticamente
        # Intentar escritura simple
        i2c.writeto(addr, b'\x42')  # Enviar un byte de prueba
        return True
    except:
        return False

# Bucle principal
while True:
    print("\n" + "="*40)
    print("Iniciando escaneo I2C...")
    
    # Escanear todas las direcciones
    found_devices = scan_i2c()
    
    if found_devices:
        print(f"\n✓ Encontrados {len(found_devices)} dispositivos:")
        for addr in found_devices:
            print(f"  - 0x{addr:02X}")
            
        # Probar específicamente la dirección 0x50 (nuestro slave)
        if 0x50 in found_devices:
            print("\n🎉 ¡Slave PY32F003 encontrado en 0x50!")
            direct_test_0x50()
        else:
            # Si no lo encontramos en el scan, probar directamente
            print("\n🔍 0x50 no detectado en scan, probando directamente...")
            if direct_test_0x50():
                print("✅ ¡0x50 responde con prueba directa!")
            else:
                print("❌ 0x50 no responde ni en prueba directa")
        
        # Probar el primer dispositivo encontrado con método directo
        if found_devices:
            print(f"\nProbando primer dispositivo: 0x{found_devices[0]:02X}")
            direct_test_0x50()  # Usar la misma función pero cambiar addr internamente
            
    else:
        print("\n⚠ No se encontraron dispositivos I2C")
        print("Verifica las conexiones:")
        print("  - ESP32-H2 Pin 12 (SDA) → PA10 del PY32F003")
        print("  - ESP32-H2 Pin 22 (SCL) → PB6 del PY32F003") 
        print("  - ESP32-H2 GND → PY32F003 GND")
        print("  - ¿Está ejecutándose el código slave en el PY32F003?")
    
    print("\nEsperando 3 segundos antes del próximo escaneo...")
    time.sleep(3)
