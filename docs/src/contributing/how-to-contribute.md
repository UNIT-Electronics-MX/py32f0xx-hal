# How to Contribute

We welcome contributions to the PY32F0xx HAL project! Whether you're fixing bugs, adding features, improving documentation, or testing on new hardware, your help is appreciated.

## Ways to Contribute

### Code Contributions
- **Bug fixes** - Fix issues you encounter
- **New features** - Add support for new peripherals or devices
- **Examples** - Create examples for common use cases
- **Optimizations** - Improve performance or reduce code size

### Documentation
- **Fix typos** - Correct spelling and grammar errors
- **Add examples** - Document working code patterns  
- **Improve clarity** - Make instructions easier to follow
- **Translate content** - Help with internationalization

### Testing and Validation
- **Test on new hardware** - Verify HAL works on different devices
- **Report bugs** - Document issues you encounter
- **Validate examples** - Confirm examples work as expected
- **Performance testing** - Benchmark and optimize code

## Getting Started

### 1. Set Up Development Environment

Follow our [Development Environment Guide](../getting-started/development-environment.md) to set up your tools.

### 2. Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/py32f0xx-hal.git
cd py32f0xx-hal

# Add upstream remote
git remote add upstream https://github.com/UNIT-Electronics-MX/py32f0xx-hal.git
```

### 3. Create a Feature Branch

```bash
# Create and switch to a new branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

## Development Guidelines

### Code Style

- **Follow Rust conventions** - Use `cargo fmt` and `cargo clippy`
- **Document public APIs** - Add doc comments to public functions
- **Include examples** - Show how to use new features
- **Test thoroughly** - Verify code works on real hardware

### Commit Messages

Use clear, descriptive commit messages:

```
type: brief description

Optional longer description explaining the change.

Fixes #123
```

**Types:**
- `feat:` - New features
- `fix:` - Bug fixes  
- `docs:` - Documentation changes
- `style:` - Code formatting
- `refactor:` - Code restructuring
- `test:` - Test additions or changes
- `chore:` - Maintenance tasks

### Testing

- **Test on real hardware** - Ensure changes work on actual devices
- **Include test cases** - Add tests for new functionality
- **Verify examples** - Make sure all examples still compile and work
- **Check documentation** - Verify docs are accurate and complete

## Submitting Changes

### 1. Prepare Your Changes

```bash
# Format code
cargo fmt

# Run clippy for linting
cargo clippy --all-targets --all-features

# Build all examples
make build-all

# Test documentation
cd docs && mdbook test
```

### 2. Commit and Push

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m \"feat: add USART interrupt support\"

# Push to your fork
git push origin feature/your-feature-name
```

### 3. Create Pull Request

1. **Go to GitHub** and create a pull request
2. **Describe your changes** clearly in the PR description
3. **Reference issues** if your PR fixes them
4. **Include testing details** - what hardware you tested on

### Pull Request Template

```markdown
## Description
Brief description of what this PR does.

## Changes
- List of changes made
- New features added
- Bugs fixed

## Testing
- [ ] Tested on PY32F003I DFN8
- [ ] Tested on PY32F030 TSSOP20
- [ ] All examples compile
- [ ] Documentation updated

## Related Issues
Fixes #123
```

## Areas Needing Help

### High Priority
- **Device support expansion** - Test on more PY32F0xx variants
- **Peripheral implementations** - Complete I2C, SPI, DMA drivers
- **Example collection** - More real-world examples
- **Documentation improvements** - Clearer getting-started guides

### Medium Priority
- **Performance optimizations** - Reduce code size and improve speed
- **Power management** - Low-power mode support
- **Bootloader support** - Custom bootloader implementations
- **Testing framework** - Automated hardware-in-loop testing

### Good First Issues
- **Fix documentation typos** - Easy way to get started
- **Add code comments** - Improve code readability  
- **Create simple examples** - Basic peripheral usage
- **Update README files** - Keep information current

## Development Best Practices

### Hardware Testing

When testing changes:
1. **Test on multiple devices** if possible
2. **Use different packages** (DFN8, TSSOP20, etc.)
3. **Verify with oscilloscope** for timing-critical changes
4. **Document test setup** in PR description

### Code Review Process

1. **Self-review first** - Check your own code carefully
2. **Address feedback promptly** - Respond to review comments
3. **Update tests** - Modify tests if needed
4. **Squash commits** - Clean up commit history if requested

### Documentation Standards

- **Include working examples** for new features
- **Update CHANGELOG.md** for significant changes
- **Add troubleshooting info** for complex features
- **Keep style consistent** with existing docs

## Community Guidelines

### Be Respectful
- **Welcome newcomers** - Everyone was a beginner once
- **Give constructive feedback** - Focus on the code, not the person
- **Be patient** - Not everyone has the same experience level

### Be Collaborative
- **Share knowledge** - Help others learn
- **Ask questions** - Don't hesitate to seek clarification
- **Offer help** - Assist with testing, documentation, or code review

## Getting Help

Need help with contributing?

### Communication Channels
- **GitHub Issues** - For bugs and feature requests
- **GitHub Discussions** - For questions and general discussion
- **PR Comments** - For code-specific questions

### Resources
- [Rust Embedded Book](https://docs.rust-embedded.org/book/) - Learn embedded Rust
- [PY32F0xx Documentation](../reference/) - Hardware reference
- [HAL Examples](../examples/) - Working code samples

## Recognition

Contributors are recognized in:
- **CHANGELOG.md** - Credits for significant changes
- **README.md** - Hall of fame for major contributors
- **GitHub Contributors** - Automatic recognition

## Legal

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT/Apache 2.0).

---

**Thank you for contributing to PY32F0xx HAL!** Every contribution, no matter how small, helps make embedded Rust development better for everyone.
