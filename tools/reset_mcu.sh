#!/bin/bash
# Script para hacer reset por software del PY32F003x4

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PYOCD_VENV="$PROJECT_ROOT/venv/bin/pyocd"

echo "Realizando reset sw_sysresetreq del PY32F003x4..."

# Verificar que el venv existe
if [ ! -f "$PYOCD_VENV" ]; then
    echo "Error: No se encontró PyOCD en el entorno virtual en $PYOCD_VENV"
    exit 1
fi

cd "$SCRIPT_DIR/Misc"

# Ejecutar reset por software usando system reset request
printf "reset sw_sysresetreq\nquit\n" | "$PYOCD_VENV" commander -t py32f003x4 --config pyocd.yaml

echo "Reset sw_sysresetreq completado."
