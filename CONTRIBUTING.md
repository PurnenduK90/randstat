# Contributing to randstat

Thank you for your interest in contributing to randstat! We welcome contributions from the community.

## Before You Contribute

### Contributor License Agreement (CLA)

**Important:** Before submitting any contributions, please read and agree to our [Contributor License Agreement (CLA.md)](CLA.md).

**Key Points:**
- Your contributions will be publicly available under EUPL-1.2
- You grant the project maintainer additional rights to use your contributions in proprietary/commercial projects
- You confirm you have the legal right to contribute the code
- You retain all your rights to your contributions

**Corporate Contributors:** If you're contributing on behalf of a corporation, your employer should sign the [Corporate CLA (CLA-CORPORATE.md)](CLA-CORPORATE.md).

### How to Indicate Agreement

When submitting a pull request, please include the following statement in your PR description:

```
I have read and agree to the Contributor License Agreement (CLA.md).
```

---

## Getting Started

### Prerequisites

```bash
# Rust toolchain (stable)
rustup update stable

# WASM target (for WebAssembly builds)
rustup target add wasm32-unknown-unknown

# just (optional task runner)
cargo install just
```

### Development Workflow

1. **Fork and clone** the repository
2. **Create a branch** for your feature or fix
3. **Make your changes** following our code style
4. **Run tests**: `just test` or `cargo test --workspace`
5. **Check code**: `just check` or `cargo check --workspace`
6. **Check WASM**: `just check-wasm`
7. **Commit your changes** with clear, descriptive commit messages
8. **Push to your fork** and submit a pull request
9. **Include CLA agreement** in your PR description

---

## Code Guidelines

### Architecture Principles

- **One test = one file**: Every `StreamTest` struct lives in its own `.rs` file
- **Algorithms in core**: Pure evaluation functions in `randstat_core::algorithms` — no state, just math
- **Tests as thin wrappers**: `evaluate()` calls a core algorithm; structs only hold accumulator counts
- **`no_std` compatibility**: Keep core crates `no_std` compatible (use `core` + `libm` only)
- **Zero heap allocation**: All tests should be `Copy + const`-constructible

### Adding a New Test

See [README.md - Adding a New Test](README.md#adding-a-new-test) for the step-by-step process.

### Code Style

- Follow standard Rust conventions (`rustfmt`)
- Write clear, self-documenting code
- Add comments for complex algorithms
- Include documentation comments (`///`) for public APIs
- Prefer explicit types over inference in public APIs

### Testing

- Add unit tests for new functionality
- Ensure all tests pass: `cargo test --workspace`
- Test `no_std` compatibility: `just check-wasm`
- Test all suite variants if applicable

---

## Types of Contributions

### Bug Reports

- Use the issue tracker to report bugs
- Include a clear description of the problem
- Provide steps to reproduce
- Include relevant error messages or output
- Specify your environment (OS, Rust version, etc.)

### Feature Requests

- Open an issue to discuss new features before implementation
- Explain the use case and benefit
- Consider whether it fits the project's scope and design principles

### Code Contributions

- **Bug fixes**: Always welcome
- **New tests**: Implement additional statistical tests from NIST, Dieharder, or other batteries
- **Performance improvements**: Especially for hot paths in streaming accumulators
- **Documentation**: Improve docs, examples, or comments
- **WASM optimization**: Reduce binary size or improve JS interop

### Documentation

- Fix typos, clarify explanations
- Add examples or usage guides
- Improve API documentation
- Update README or guides

---

## Pull Request Process

1. **Ensure CI passes**: All checks, tests, and WASM builds must succeed
2. **Update documentation**: If you change APIs or add features
3. **Follow commit conventions**: Use clear, descriptive commit messages
4. **Keep PRs focused**: One feature or fix per PR
5. **Respond to feedback**: Address review comments promptly
6. **Include CLA agreement**: Add the statement to your PR description

### PR Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass (`just test`)
- [ ] `no_std` compatibility maintained (`just check-wasm`)
- [ ] Documentation updated if needed
- [ ] CLA agreement included in PR description
- [ ] Commit messages are clear and descriptive

---

## Questions?

If you have questions about contributing:
- Open an issue for discussion
- Check existing issues and documentation
- Review the [CLA.md](CLA.md) for licensing questions

---

## Code of Conduct

- Be respectful and professional
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Assume good intentions

---

Thank you for contributing to randstat! 🎲
