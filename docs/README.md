# PY32F0xx HAL Documentation

This directory contains comprehensive documentation for the PY32F0xx Hardware Abstraction Layer, built with [mdBook](https://rust-lang.github.io/mdBook/).

## Documentation Structure

```
docs/
├── book.toml              # mdBook configuration
├── src/                   # Documentation source files
│   ├── SUMMARY.md         # Table of contents
│   ├── introduction.md    # Main introduction
│   ├── getting-started/   # Setup and first steps
│   ├── examples/          # Working code examples
│   ├── peripherals/       # HAL peripheral documentation
│   ├── devices/           # Device-specific guides
│   ├── troubleshooting/   # Problem solving guides
│   ├── reference/         # Technical references
│   └── contributing/      # Development guidelines
└── book/                  # Generated documentation (auto-created)
```

## Building the Documentation

### Prerequisites

Install mdBook:

```bash
# Install mdBook
cargo install mdbook

# Verify installation
mdbook --version
```

### Build Documentation

```bash
# From the docs/ directory
cd docs/

# Build the documentation
mdbook build

# Serve locally with live reload
mdbook serve

# Open in browser (usually http://localhost:3000)
```

### Automated Build

Use the provided script:

```bash
# From the project root
./build-docs.sh

# Or make it executable first
chmod +x build-docs.sh
./build-docs.sh
```

## Documentation Content

### Getting Started Section
- **[Quick Start](src/getting-started/quick-start.md)** - Get up and running in minutes
- **[Hardware Setup](src/getting-started/hardware-setup.md)** - Wiring and connections
- **[Development Environment](src/getting-started/development-environment.md)** - Tools setup
- **[First Program](src/getting-started/first-program.md)** - Your first embedded Rust program

### Working Examples
- **[Blinky LED](src/examples/blinky.md)** - Basic GPIO output
- **[Serial Communication](src/examples/serial.md)** - USART examples
- **[Serial Echo](src/examples/serial-echo.md)** - Basic serial I/O
- **[Serial ADC](src/examples/serial-adc.md)** - Advanced serial + ADC

### Peripheral Documentation
- **[GPIO](src/peripherals/gpio.md)** - General purpose I/O
- **[USART](src/peripherals/usart.md)** - Serial communication
- **[ADC](src/peripherals/adc.md)** - Analog-to-digital converter
- **[Timers](src/peripherals/timers.md)** - Timer peripherals
- **[SPI](src/peripherals/spi.md)** - Serial peripheral interface
- **[I2C](src/peripherals/i2c.md)** - Inter-integrated circuit
- **[RTC](src/peripherals/rtc.md)** - Real-time clock
- **[DMA](src/peripherals/dma.md)** - Direct memory access

### Troubleshooting Guides
- **[Serial Issues](src/troubleshooting/serial.md)** - Serial communication problems
- **[Build Issues](src/troubleshooting/build.md)** - Compilation and linking
- **[GPIO Problems](src/troubleshooting/gpio.md)** - Pin configuration issues
- **[Debug LED](src/troubleshooting/debug-led.md)** - LED troubleshooting

## Contributing to Documentation

### Adding New Content

1. **Create new markdown file** in appropriate directory
2. **Add entry to SUMMARY.md** to include in navigation
3. **Follow existing style** and format conventions
4. **Build and test** before submitting

### Style Guidelines

- Use **clear, descriptive headings**
- Include **working code examples** where possible
- Add **troubleshooting sections** for complex topics
- Use **consistent formatting** throughout
- Include **hardware requirements** for examples

### Testing Changes

```bash
# Build and serve locally
mdbook serve

# Check for broken links
mdbook test

# Verify all examples compile
cargo build --examples --all-features
```

## Documentation Features

### Search
- Full-text search of all documentation
- Indexed headings and content
- Keyboard shortcuts (press 's' to search)

### Navigation
- Sidebar navigation tree
- Previous/next chapter buttons
- Breadcrumb navigation
- Mobile-friendly responsive design

### Code Highlighting
- Rust syntax highlighting
- Bash/shell command highlighting
- Configuration file highlighting
- Copy code button

### Print Support
- Print-friendly formatting
- Single-page print view
- Optimized for PDF generation

## Viewing Options

### Local Development
```bash
# Live reload server
mdbook serve --open

# Custom port
mdbook serve --port 8080

# External access
mdbook serve --hostname 0.0.0.0
```

### Static Hosting
The generated `book/` directory contains static HTML files that can be hosted on:
- GitHub Pages
- Netlify
- Vercel
- Any static web server

### Offline Viewing
Open `book/index.html` in any web browser for offline access.

## Maintenance

### Updating Documentation

When code examples change:
1. Update relevant documentation files
2. Test all examples still work
3. Rebuild documentation
4. Check for broken links

### Regular Tasks
- Keep examples synchronized with HAL changes
- Update troubleshooting guides based on user feedback
- Add new peripheral documentation as HAL expands
- Verify all links and references are current

### Version Management
Documentation versions should align with HAL releases:
- Major HAL changes → Update getting started guides
- New peripherals → Add peripheral documentation
- Bug fixes → Update troubleshooting guides

## Feedback and Issues

Found documentation issues?
- **Typos/errors**: Fix and submit PR
- **Missing content**: Open issue with details
- **Unclear sections**: Suggest improvements
- **New examples needed**: Propose additions

## Related Resources

- **[HAL API Docs](https://docs.rs/py32f0xx-hal/)** - Rust API documentation
- **[PY32F0xx Datasheet](../tools/)** - Hardware reference
- **[mdBook Documentation](https://rust-lang.github.io/mdBook/)** - mdBook usage guide

---

This documentation aims to make PY32F0xx development accessible to everyone, from beginners to experts. Every improvement helps the entire community!
