# Contributing to The Book of Rust

Thank you for your interest in contributing to The Book of Rust learning platform! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Security](#security)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/rust-learning-ground.git
   cd rust-learning-ground
   ```
3. **Create a branch** for your contribution:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## How to Contribute

### Reporting Bugs

Before creating a bug report, please check if the issue already exists. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the issue
- **Expected behavior** vs actual behavior
- **Screenshots** (if applicable)
- **Environment details** (OS, browser, versions)
- **Additional context**

Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md).

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. Include:

- **Clear title and description**
- **Problem it solves**
- **Proposed solution**
- **Alternatives considered**

Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md).

### Contributing Code

#### Good First Issues

Look for issues labeled `good first issue` or `help wanted` if you're new to the project.

#### Areas for Contribution

- **Exercises**: Add new Rust exercises or improve existing ones
- **Translations**: Help translate content to more languages
- **Documentation**: Improve docs, add examples
- **Bug fixes**: Fix reported issues
- **Features**: Implement requested features
- **Tests**: Add test coverage

## Development Setup

### Prerequisites

- Python 3.11+
- Node.js 18+ (for frontend development)
- Rust 1.75+ (for exercise testing)
- SQLite or PostgreSQL

### Backend Setup

```bash
cd server
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt

# Create .env file
cp .env.example .env
# Edit .env with your settings

# Run migrations
python -m app.main
```

### Frontend Setup

```bash
# Static files are served from web/
# No build step required for basic development

# For development, run both servers:
python start_servers.py
```

### Running Tests

```bash
cd server
pytest

# Run with coverage
pytest --cov=app --cov-report=html
```

### Security Audit

```bash
cd server
./scripts/security_audit.sh
```

## Pull Request Process

1. **Update documentation** if needed
2. **Add tests** for new functionality
3. **Ensure all tests pass**
4. **Update the CHANGELOG.md** with your changes
5. **Reference issues** your PR addresses
6. **Request review** from maintainers

### PR Checklist

- [ ] Code follows the style guidelines
- [ ] Tests added/updated and passing
- [ ] Documentation updated
- [ ] Security audit passed (`./scripts/security_audit.sh`)
- [ ] Commit messages are clear and descriptive
- [ ] PR description explains the changes

### Commit Message Format

We follow conventional commits:

```
type(scope): subject

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, semicolons, etc)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Build process or auxiliary tool changes
- `security`: Security-related changes

Example:
```
feat(auth): add OAuth2 support for GitHub

Implement GitHub OAuth2 authentication flow with proper
token refresh handling.

Closes #123
```

## Coding Standards

### Python

- Follow [PEP 8](https://pep8.org/)
- Use type hints
- Maximum line length: 100 characters
- Use `black` for formatting: `black app/`
- Use `isort` for imports: `isort app/`
- Use `flake8` for linting: `flake8 app/`

### JavaScript

- Use ES6+ features
- Prefer `const` and `let` over `var`
- Use semicolons
- 2 spaces for indentation
- Single quotes for strings

### Rust (Exercises)

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Include tests for exercises

### Documentation

- Use clear, concise language
- Include code examples
- Update relevant README files
- Add docstrings to Python functions

## Security

Please see [SECURITY.md](SECURITY.md) for:
- Reporting security vulnerabilities
- Security best practices
- Security audit procedures

### Security Requirements for Contributors

- Never commit secrets or credentials
- Use environment variables for sensitive configuration
- Follow the principle of least privilege
- Validate all user input
- Use parameterized queries for database access
- Keep dependencies updated

## Questions?

- Join our [Discussions](https://github.com/your-org/rust-learning-ground/discussions)
- Check existing [Issues](https://github.com/your-org/rust-learning-ground/issues)
- Email: contributors@rustlearning.local

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to The Book of Rust! 🦀
