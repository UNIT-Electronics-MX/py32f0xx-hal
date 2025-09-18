#!/bin/bash

# Script de diagnóstico para PY32F003x4
# Verifica si el programa está corriendo y los GPIOs funcionando

echo "=== Diagnóstico PY32F003x4 ==="
echo ""

cd /media/mr/firmware/personal/rust/py32f0xx-hal/tools/Misc

echo "1. Conectando a PY32F003x4..."
echo "halt" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml

echo ""
echo "2. Verificando estado del programa..."
echo "reg pc" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml

echo ""
echo "3. Verificando memoria flash (debe contener el programa)..."
echo "mem 0x08000000 32" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml

echo ""
echo "4. Verificando configuración de clocks RCC..."
echo "mem 0x40021000 16" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml

echo ""
echo "5. Verificando configuración GPIO A (PA1)..."
echo "mem 0x50000000 8" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml

echo ""
echo "6. Verificando configuración GPIO B (PB5)..."
echo "mem 0x50000400 8" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml

echo ""
echo "7. Verificando estado actual de los pines..."
echo "mem 0x50000010 1" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml  # GPIOA ODR
echo "mem 0x50000410 1" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml  # GPIOB ODR

echo ""
echo "8. Reiniciando MCU y continuando ejecución..."
echo "reset" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml
echo "go" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml

echo ""
echo "=== Diagnóstico completado ==="
echo ""
echo "Instrucciones:"
echo "- Verifica que PC (Program Counter) no sea 0x00000000"
echo "- La memoria flash en 0x08000000 debe contener datos (no todo 0xFF)"
echo "- Los registros GPIO deben mostrar configuración como output"
echo "- Los registros ODR deben cambiar de valor"
echo ""
echo "Para monitoreo continuo del estado de los pines:"
echo "watch -n 1 'echo \"mem 0x50000010 1; mem 0x50000410 1\" | ../../venv/bin/pyocd commander -t py32f003x4 --config pyocd.yaml'"
