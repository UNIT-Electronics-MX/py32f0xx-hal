# Project Scripts

This directory contains various automation and setup scripts for the PY32F0xx HAL project.

## Scripts

### Build and Documentation
- **`build-docs.sh`** - Build mdBook documentation
- **`build-docs-fixed.sh`** - Alternative documentation build script
- **`clean_project.sh`** - Clean project build artifacts

### Development Setup
- **`setup.sh`** - Main environment setup script
  - Installs Rust toolchain and targets
  - Sets up PyOCD in a Python virtual environment
  - Configures development dependencies

### Debugging
- **`debug_py32.sh`** - PY32 debugging utilities

## Usage

### Environment Setup
```bash
# Run from project root
./scripts/setup.sh
```

### Documentation Build
```bash
# Build mdBook documentation
make docs
# or directly:
./scripts/build-docs.sh
```

### Project Cleanup
```bash
# Clean build artifacts
./scripts/clean_project.sh
```

## File Permissions

Make sure scripts are executable:
```bash
chmod +x scripts/*.sh
```

## Integration

These scripts are integrated into the main Makefile:
- `make docs` → uses `scripts/build-docs.sh`
- Setup references in documentation point to `scripts/setup.sh`

## Migration Note

These scripts were moved from the project root to improve organization:
- `build-docs.sh` → `scripts/build-docs.sh`
- `setup.sh` → `scripts/setup.sh` 
- `clean_project.sh` → `scripts/clean_project.sh`
- `debug_py32.sh` → `scripts/debug_py32.sh`

All documentation and references have been updated to reflect the new paths.
