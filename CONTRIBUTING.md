# Contributing to Air Quality Notifier

Thank you for considering contributing to Air Quality Notifier! We welcome contributions from everyone.

## Code of Conduct

This project follows a simple code of conduct: be respectful, constructive, and collaborative.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce**
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version, etc.)
- **Relevant logs or error messages**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Clear title and description**
- **Use case and motivation**
- **Possible implementation approach**
- **Alternative solutions considered**

### Pull Requests

1. **Fork the repository** and create your branch from `master`
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Follow the project structure**
   - Domain logic goes in `src/domain/`
   - Use cases in `src/use_cases/`
   - External adapters in `src/adapters/`

3. **Write clean, idiomatic Rust code**
   - Follow Rust naming conventions
   - Keep functions focused and small
   - Use type system for safety
   - Handle errors properly

4. **Format your code**
   ```bash
   cargo fmt
   ```

5. **Check for warnings**
   ```bash
   cargo clippy -- -D warnings
   ```

6. **Test your changes**
   ```bash
   cargo build
   cargo test
   ```

7. **Commit with descriptive messages**
   ```
   Add feature: Support for OpenWeatherMap API

   - Implement OpenWeatherMap adapter
   - Add configuration for API key
   - Update documentation
   ```

8. **Push to your fork** and submit a pull request

## Architecture Guidelines

This project follows **Clean Architecture** principles:

### Domain Layer (`src/domain/`)
- Pure business logic
- No external dependencies
- Core entities and value objects
- Domain services

### Use Cases Layer (`src/use_cases/`)
- Application business rules
- Orchestrates domain logic
- Defines interfaces (traits) for adapters
- Independent of frameworks and UI

### Adapters Layer (`src/adapters/`)
- External interfaces (APIs, databases, config)
- Implements use case interfaces
- Framework-specific code
- All I/O operations

### Dependency Rule
**Dependencies point inward**: Adapters → Use Cases → Domain

## Code Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Prefer composition over inheritance
- Write self-documenting code
- Add comments for complex logic only
- Use descriptive variable names

## Testing

- Write unit tests for business logic
- Test use cases with mock adapters
- Integration tests for external adapters
- Keep tests focused and readable

## Documentation

- Update README.md for new features
- Add inline documentation for public APIs
- Update .env.example for new config options
- Document breaking changes

## Getting Help

- Open an issue for questions
- Check existing issues and discussions
- Review the architecture documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
