# WIOS Makefile — Docker-first development commands
# All commands run inside Docker. Nothing installed locally.

DOCKER_RUN = docker compose run --rm dev
DOCKER_EXEC = docker compose exec dev

.PHONY: build up down shell rust-check rust-test rust-build rust-clippy rust-fmt \
        flutter-get flutter-analyze flutter-test flutter-web flutter-clean \
        test lint format all clean help

# ── Docker Lifecycle ────────────────────────────────────────────

## Build the development Docker image
build:
	docker compose build dev

## Start dev container in background
up:
	docker compose up -d dev

## Stop all containers
down:
	docker compose down

## Open a shell in the dev container
shell:
	$(DOCKER_RUN) bash

# ── Rust Commands ───────────────────────────────────────────────

## Check all Rust crates compile
rust-check:
	$(DOCKER_RUN) bash -c "cd crates && cargo check --workspace"

## Run all Rust tests
rust-test:
	$(DOCKER_RUN) bash -c "cd crates && cargo test --workspace"

## Build Rust release
rust-build:
	$(DOCKER_RUN) bash -c "cd crates && cargo build --workspace --release"

## Run Clippy lints
rust-clippy:
	$(DOCKER_RUN) bash -c "cd crates && cargo clippy --workspace --all-targets -- -D warnings"

## Format Rust code
rust-fmt:
	$(DOCKER_RUN) bash -c "cd crates && cargo fmt --all"

## Check Rust formatting
rust-fmt-check:
	$(DOCKER_RUN) bash -c "cd crates && cargo fmt --all --check"

## Run Rust coverage
rust-coverage:
	$(DOCKER_RUN) bash -c "cd crates && cargo tarpaulin --workspace --out Html --output-dir /workspace/coverage/rust"

# ── Flutter Commands ────────────────────────────────────────────

## Get Flutter dependencies
flutter-get:
	$(DOCKER_RUN) bash -c "melos bootstrap"

## Run Dart analysis
flutter-analyze:
	$(DOCKER_RUN) bash -c "melos run analyze"

## Run Flutter tests
flutter-test:
	$(DOCKER_RUN) bash -c "melos run test"

## Build Flutter web release
flutter-web:
	$(DOCKER_RUN) bash -c "cd apps/wios_app && flutter build web --release"

## Serve Flutter web (interactive — use docker exec)
flutter-serve:
	$(DOCKER_RUN) -p 8082:8082 bash -c "cd apps/wios_app && flutter run -d web-server --web-port 8082 --web-hostname 0.0.0.0"

## Clean Flutter builds
flutter-clean:
	$(DOCKER_RUN) bash -c "melos run clean"

## Format Dart code
flutter-format:
	$(DOCKER_RUN) bash -c "melos run format"

# ── Combined Commands ───────────────────────────────────────────

## Run all tests (Rust + Flutter)
test: rust-test flutter-test

## Run all lints (Rust + Flutter)
lint: rust-clippy rust-fmt-check flutter-analyze

## Format all code
format: rust-fmt flutter-format

## Full CI check (lint + test + build)
ci: lint test rust-build flutter-web

## Build everything
all: rust-build flutter-web

## Clean all artifacts
clean:
	$(DOCKER_RUN) bash -c "cd crates && cargo clean"
	$(DOCKER_RUN) bash -c "melos run clean"

# ── Help ────────────────────────────────────────────────────────

## Show this help
help:
	@echo "WIOS Development Commands (Docker-first)"
	@echo ""
	@echo "  make build          - Build Docker dev image"
	@echo "  make up             - Start dev container"
	@echo "  make down           - Stop all containers"
	@echo "  make shell          - Open shell in container"
	@echo ""
	@echo "  make rust-check     - Check Rust compilation"
	@echo "  make rust-test      - Run Rust tests"
	@echo "  make rust-build     - Build Rust release"
	@echo "  make rust-clippy    - Run Clippy lints"
	@echo "  make rust-fmt       - Format Rust code"
	@echo "  make rust-coverage  - Generate coverage report"
	@echo ""
	@echo "  make flutter-get    - Get Flutter dependencies"
	@echo "  make flutter-analyze- Analyze Dart code"
	@echo "  make flutter-test   - Run Flutter tests"
	@echo "  make flutter-web    - Build web release"
	@echo "  make flutter-serve  - Serve web on :8082"
	@echo ""
	@echo "  make test           - Run ALL tests"
	@echo "  make lint           - Run ALL lints"
	@echo "  make ci             - Full CI check"
	@echo "  make all            - Build everything"
