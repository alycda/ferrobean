# Contributing Guidelines

We welcome contributions to Ferrobean! This guide will help you get started.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/ferrobean.git
   cd ferrobean
   ```
3. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

Ensure you have the required tools installed:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install additional tools
cargo install cargo-generate
npm install -g wrangler
```

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Test locally
npx wrangler dev
```

## Contribution Guidelines

### Code Style

- Follow Rust's official style guidelines
- Use `cargo fmt` to format code
- Run `cargo clippy` to catch common issues
- Ensure all tests pass before submitting

### Commit Messages

Use clear, descriptive commit messages:

```
feat: add support for new beancount directive
fix: resolve flag parsing edge case
docs: update API documentation
test: add integration tests for flags module
```

### Testing

- Write unit tests for all new functionality
- Ensure existing tests continue to pass
- Add integration tests when appropriate
- Aim for high test coverage

### Documentation

- Update relevant documentation for any changes
- Add inline code comments for complex logic
- Update the API reference for new endpoints
- Include examples in documentation

## Pull Request Process

1. **Ensure your branch is up to date**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run the full test suite**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

3. **Create a pull request** with:
   - Clear description of changes
   - Link to any related issues
   - Screenshots for UI changes (if applicable)
   - Test results and coverage information

4. **Address review feedback** promptly and respectfully

## Code Review Guidelines

### For Reviewers

- Be constructive and specific in feedback
- Test the changes locally when possible
- Check for security implications
- Ensure documentation is updated

### For Contributors

- Respond to feedback promptly
- Ask questions if feedback is unclear
- Make requested changes in separate commits
- Thank reviewers for their time

## Reporting Issues

When reporting bugs or requesting features:

1. **Search existing issues** first
2. **Use the issue templates** when available
3. **Provide detailed information**:
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details
   - Error messages or logs

## Security

- Report security vulnerabilities privately to the maintainers
- Do not publish security issues in public forums
- Allow time for fixes before public disclosure

## Community

- Be respectful and inclusive
- Follow the project's code of conduct
- Help others in discussions and issues
- Share knowledge and best practices

## Recognition

Contributors will be recognized in:
- The project's contributor list
- Release notes for significant contributions
- Documentation acknowledgments

Thank you for contributing to Ferrobean!