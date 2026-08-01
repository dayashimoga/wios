# Contributing to WIOS

## Development Setup

1. Install Flutter SDK (3.44+)
2. Install Rust toolchain (stable)
3. Install Melos: `dart pub global activate melos`
4. Bootstrap: `melos bootstrap`
5. Build Rust: `cd crates && cargo build`

## Code Style

### Rust
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Dart/Flutter
- Run `dart format .` before committing
- Run `dart analyze` and fix all issues
- Follow [Effective Dart](https://dart.dev/effective-dart)

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Update documentation
6. Submit PR with clear description

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `refactor:` Code refactoring
- `test:` Tests
- `chore:` Maintenance
