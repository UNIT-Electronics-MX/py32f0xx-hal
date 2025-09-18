#!/usr/bin/env python3
"""
I2C Master Scanner Avanzado - MicroPython
Para comunicación completa con slave PY32F0xx

Funciones:
- Escaneo automático de direcciones
- Prueba de escritura/lectura
- Monitoreo continuo
- Comandos interactivos
"""

from machine import Pin, I2C
import time

class I2CMaster:
    def __init__(self, scl_pin, sda_pin, freq=100000):
        """Inicializar I2C Master"""
        self.i2c = I2C(0, scl=Pin(scl_pin), sda=Pin(sda_pin), freq=freq)
        self.slave_addr = 0x50  # Dirección del slave PY32F003
        
    def scan(self):
        """Escanear bus I2C"""
        print("Escaneando bus I2C...")
        devices = self.i2c.scan()
        
        if devices:
            print(f"Dispositivos encontrados: {len(devices)}")
            for addr in devices:
                print(f"  → 0x{addr:02X}")
        else:
            print("❌ No se encontraron dispositivos")
            
        return devices
    
    def ping_device(self, addr):
        """Hacer ping a un dispositivo específico"""
        try:
            self.i2c.writeto(addr, b'')
            return True
        except OSError:
            return False
    
    def write_byte(self, addr, data):
        """Escribir un byte a un dispositivo"""
        try:
            if isinstance(data, int):
                data = bytes([data])
            elif isinstance(data, str):
                data = data.encode()
                
            self.i2c.writeto(addr, data)
            print(f"✓ Enviado a 0x{addr:02X}: {data}")
            return True
        except OSError as e:
            print(f"❌ Error escribiendo a 0x{addr:02X}: {e}")
            return False
    
    def read_bytes(self, addr, num_bytes=1):
        """Leer bytes de un dispositivo"""
        try:
            data = self.i2c.readfrom(addr, num_bytes)
            print(f"✓ Leído de 0x{addr:02X}: {data}")
            return data
        except OSError as e:
            print(f"❌ Error leyendo de 0x{addr:02X}: {e}")
            return None
    
    def test_communication(self, addr):
        """Probar comunicación completa con un dispositivo"""
        print(f"\n=== Probando comunicación con 0x{addr:02X} ===")
        
        # 1. Ping básico
        if self.ping_device(addr):
            print("✓ Ping exitoso")
        else:
            print("❌ Ping falló")
            return False
        
        # 2. Escribir datos de prueba
        test_data = [0x01, 0x02, 0x03]
        for data in test_data:
            self.write_byte(addr, data)
            time.sleep_ms(100)
        
        # 3. Intentar leer respuesta
        self.read_bytes(addr, 1)
        
        return True
    
    def monitor_slave(self, addr=0x50, interval=2):
        """Monitorear slave continuamente"""
        print(f"\n=== Monitoreando slave 0x{addr:02X} ===")
        print("Presiona Ctrl+C para detener")
        
        counter = 0
        try:
            while True:
                counter += 1
                print(f"\n--- Intento {counter} ---")
                
                # Hacer ping
                if self.ping_device(addr):
                    print(f"✓ Slave 0x{addr:02X} responde")
                    
                    # Enviar contador como dato
                    self.write_byte(addr, counter & 0xFF)
                    
                else:
                    print(f"❌ Slave 0x{addr:02X} no responde")
                
                time.sleep(interval)
                
        except KeyboardInterrupt:
            print("\n⏹ Monitoreo detenido por usuario")

# Configuración para diferentes microcontroladores
BOARD_CONFIGS = {
    'esp32': {'scl': 22, 'sda': 21},
    'esp8266': {'scl': 5, 'sda': 4}, 
    'pico': {'scl': 1, 'sda': 0},
    'custom': {'scl': 22, 'sda': 21}  # Cambia según tu configuración
}

def main():
    print("=== I2C Master Scanner Avanzado ===")
    print("Para comunicar con slave PY32F003 en 0x50")
    print()
    
    # Seleccionar configuración (cambia según tu board)
    board = 'esp32'  # Cambia por: 'esp8266', 'pico', o 'custom'
    config = BOARD_CONFIGS[board]
    
    print(f"Configuración {board}: SCL={config['scl']}, SDA={config['sda']}")
    print("Conexiones:")
    print(f"  SCL (Pin {config['scl']}) → PB6 del PY32F003")
    print(f"  SDA (Pin {config['sda']}) → PA10 del PY32F003")
    print("  GND → GND común")
    print()
    
    # Crear master I2C
    master = I2CMaster(config['scl'], config['sda'])
    
    while True:
        print("\n" + "="*50)
        print("Opciones:")
        print("1. Escanear bus I2C")
        print("2. Probar slave PY32F003 (0x50)")
        print("3. Monitorear slave continuamente")
        print("4. Probar dirección personalizada")
        print("5. Salir")
        
        try:
            choice = input("\nSelecciona (1-5): ").strip()
            
            if choice == '1':
                master.scan()
                
            elif choice == '2':
                if 0x50 in master.scan():
                    master.test_communication(0x50)
                else:
                    print("❌ Slave PY32F003 no encontrado en 0x50")
                    
            elif choice == '3':
                master.monitor_slave()
                
            elif choice == '4':
                addr_str = input("Dirección hex (ej: 0x50): ").strip()
                try:
                    addr = int(addr_str, 16)
                    master.test_communication(addr)
                except ValueError:
                    print("❌ Dirección inválida")
                    
            elif choice == '5':
                print("👋 Saliendo...")
                break
                
            else:
                print("❌ Opción inválida")
                
        except KeyboardInterrupt:
            print("\n👋 Saliendo...")
            break
        except Exception as e:
            print(f"❌ Error: {e}")

if __name__ == "__main__":
    main()
