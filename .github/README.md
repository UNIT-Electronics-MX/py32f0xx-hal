# GitHub Actions Workflows

Este directorio contiene los workflows de GitHub Actions para automatizar CI/CD y deployment del proyecto PY32F0xx HAL.

## Workflows Disponibles

### 🚀 deploy-docs.yml
**Despliega automáticamente la documentación en GitHub Pages**

**Triggers:**
- Push a la rama `main` con cambios en `docs/`, `README.md`, o `CHANGELOG.md`
- Pull requests a `main` (solo build, no deploy)
- Dispatch manual

**Funcionalidad:**
- Construye la documentación mdBook
- Genera documentación de Rust API
- Combina todo en un sitio web unificado
- Despliega automáticamente en GitHub Pages

**URL del sitio:** `https://unit-electronics-mx.github.io/py32f0xx-hal/`

### 🔄 continuous-integration.yml
**Validación continua de código y ejemplos**

**Triggers:**
- Push a `main` o `develop`
- Pull requests a `main` o `develop`

**Jobs:**
- **Check** - Verifica que el código compile para todas las features
- **Test** - Ejecuta tests unitarios
- **Clippy** - Linting de código Rust
- **Format** - Verifica formato de código
- **Examples** - Construye ejemplos clave para diferentes chips
- **Docs** - Valida que la documentación se genere correctamente

## Configuración de GitHub Pages

Para habilitar el deployment automático:

1. Ve a **Settings** → **Pages** en tu repositorio de GitHub
2. En **Source**, selecciona **GitHub Actions**
3. Los workflows se ejecutarán automáticamente

## Estructura del Sitio Desplegado

```
https://unit-electronics-mx.github.io/py32f0xx-hal/
├── index.html                 # Página principal (redirige a introduction.html)
├── introduction.html          # Introducción del proyecto
├── getting-started/           # Guías de inicio
├── peripherals/              # Documentación de periféricos
├── examples/                 # Ejemplos detallados
├── api/                      # Documentación Rust API
│   └── py32f0xx_hal/        # API docs del HAL
└── ...                       # Otras páginas de mdBook
```

## Desarrollo Local

### Probar Workflows Localmente

```bash
# Instalar nektos/act para testing local
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Simular push a main (solo build, no deploy)
act push -W .github/workflows/deploy-docs.yml

# Simular CI
act push -W .github/workflows/continuous-integration.yml
```

### Build Manual de Documentación

```bash
# Instalar mdBook
curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.37/mdbook-v0.4.37-x86_64-unknown-linux-gnu.tar.gz | tar -xz
sudo mv mdbook /usr/local/bin/

# Construir y servir documentación
cd docs
mdbook serve --open

# Generar docs de API de Rust
cargo doc --open --no-deps --all-features --target thumbv6m-none-eabi
```

## Troubleshooting

### Error en Deploy
- Verificar que GitHub Pages esté habilitado con source "GitHub Actions"
- Revisar permisos de workflow en Settings → Actions → General

### Error en Build de Docs
- Verificar sintaxis de archivos markdown
- Comprobar enlaces internos en la documentación
- Revisar que todos los ejemplos de código sean válidos

### Error en CI
- Verificar que todos los ejemplos compilen
- Revisar formato de código con `cargo fmt`
- Corregir warnings de clippy

## Badges de Estado

Añadir a README.md principal:

```markdown
[![Deploy Docs](https://github.com/UNIT-Electronics-MX/py32f0xx-hal/actions/workflows/deploy-docs.yml/badge.svg)](https://github.com/UNIT-Electronics-MX/py32f0xx-hal/actions/workflows/deploy-docs.yml)
[![CI](https://github.com/UNIT-Electronics-MX/py32f0xx-hal/actions/workflows/continuous-integration.yml/badge.svg)](https://github.com/UNIT-Electronics-MX/py32f0xx-hal/actions/workflows/continuous-integration.yml)
```
