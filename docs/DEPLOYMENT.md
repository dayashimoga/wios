# WIOS Deployment Guide

## Platforms

### Android
```bash
flutter build apk --release
flutter build appbundle --release
```

### iOS
```bash
flutter build ipa --release
```

### Windows
```bash
flutter build windows --release
```

### Linux
```bash
flutter build linux --release
```

### macOS
```bash
flutter build macos --release
```

### Web
```bash
flutter build web --release
```

## Docker
```bash
docker-compose up -d
```

## Configuration
Copy `configs/wios.example.toml` to `~/.wios/config.toml` and edit as needed.
