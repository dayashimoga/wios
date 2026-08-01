# Deployment

## Docker Production Build

```bash
docker build -f Dockerfile.prod -t wios:latest .
docker run -p 80:80 wios:latest
```

## GitHub Actions Release

Tag a version to trigger multi-platform builds:

```bash
git tag v0.1.0
git push origin v0.1.0
```

Produces: Web, Android APK, Windows, Linux, macOS builds.

## Manual Build

```bash
# Rust
cd crates && cargo build --workspace --release

# Flutter Web
cd apps/wios_app && flutter build web --release

# Flutter Android
flutter build apk --release

# Flutter Windows
flutter build windows --release

# Flutter Linux
flutter build linux --release
```
