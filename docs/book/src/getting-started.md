# Getting Started

## Prerequisites

- **Rust** ≥ 1.87 (stable)
- **Flutter** ≥ 3.44
- **Melos** — `dart pub global activate melos`

## Development Setup

```bash
# 1. Clone
git clone https://github.com/your-org/wios.git && cd wios

# 2. Setup (installs dependencies)
./scripts/setup.sh

# 3. Run Rust tests (115 tests)
cd crates && cargo test --workspace

# 4. Run Flutter app
cd apps/wios_app && flutter run -d chrome
```

## Docker Setup

```bash
docker-compose up -d    # Dev services
docker build -f Dockerfile.dev -t wios-dev .  # Dev container
docker build -f Dockerfile.prod -t wios .     # Production
```

## Project Structure

```
WIOS/
├── crates/          # Rust backend (8 crates)
├── apps/wios_app/   # Flutter application
├── packages/        # Shared Dart packages (4)
├── proto/           # Protobuf + OpenAPI specs
├── docs/            # Documentation
├── scripts/         # Build/test/lint scripts
└── .github/         # CI/CD workflows
```
