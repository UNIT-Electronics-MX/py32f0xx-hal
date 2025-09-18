# TEST MÁS SIMPLE POSIBLE
# Copia línea por línea en tu REPL

from machine import Pin, I2C

# Setup
i2c = I2C(0, scl=Pin(22), sda=Pin(12), freq=100000)

# Test inmediato
print("Testing 0x50...")
i2c.writeto(0x50, b'\x42')
print("SUCCESS! 0x50 responde")

# Test con más datos  
i2c.writeto(0x50, b'\x01\x02\x03')
print("Datos enviados OK")

print("I2C FUNCIONA PERFECTAMENTE!")
