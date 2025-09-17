#!/bin/bash
# Script de limpieza completa del proyecto PY32F0xx-HAL

echo "🧹 Limpiando proyecto PY32F0xx-HAL..."

# Limpiar artifacts de Rust
echo "Limpiando artifacts de build..."
cargo clean
rm -rf Build/

# Limpiar archivos temporales del editor
echo "Limpiando archivos temporales..."
find . -name "*.tmp" -delete
find . -name "*.bak" -delete
find . -name "*~" -delete
find . -name ".DS_Store" -delete

# Limpiar logs de compilación
find . -name "*.log" -delete

echo "✅ Limpieza completada!"
echo ""
echo "📁 Estructura limpia del proyecto:"
echo "  examples/            - Ejemplos principales"  
echo "  examples/testing/    - Archivos de prueba"
echo "  examples/README.md   - Guía de ejemplos"
echo "  GPIO_PORT_GUIDE.md   - Guía de intercambio de puertos"
echo ""
echo "🚀 Comandos disponibles:"
echo "  make flash EXAMPLE=blinky_hal_simple    - Ejemplo principal"
echo "  make flash EXAMPLE=gpio_easy_swap       - Demo de intercambio de puertos"
echo "  make reset                              - Reset por software"
echo "  make info                               - Ver configuración"
