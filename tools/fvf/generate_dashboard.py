#!/usr/bin/env python3
"""WIOS Feature Verification Framework — Dashboard Generator.

Generates docs/DASHBOARD.md with module progress, feature health,
coverage summary, platform status, and release readiness.
"""

import sys
import yaml
from pathlib import Path
from datetime import datetime
from collections import Counter, defaultdict

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
REGISTRY_PATH = Path(__file__).resolve().parent / "feature_registry.yaml"


def load_registry():
    with open(REGISTRY_PATH) as f:
        return yaml.safe_load(f)


def count_rust_tests():
    """Count Rust test functions across all crates."""
    test_count = 0
    crates_dir = REPO_ROOT / "crates"
    for rs_file in crates_dir.rglob("*.rs"):
        if 'target' in str(rs_file):
            continue
        try:
            content = rs_file.read_text(encoding='utf-8', errors='ignore')
            test_count += content.count('#[test]')
            test_count += content.count('#[tokio::test]')
        except Exception:
            pass
    return test_count


def count_lines_of_code():
    """Count lines of Rust and Dart code."""
    rust_lines = 0
    dart_lines = 0

    for rs_file in (REPO_ROOT / "crates").rglob("*.rs"):
        if 'target' in str(rs_file):
            continue
        try:
            rust_lines += sum(1 for _ in open(rs_file, encoding='utf-8', errors='ignore'))
        except Exception:
            pass

    for dart_file in (REPO_ROOT / "apps").rglob("*.dart"):
        if 'build' in str(dart_file):
            continue
        try:
            dart_lines += sum(1 for _ in open(dart_file, encoding='utf-8', errors='ignore'))
        except Exception:
            pass

    return rust_lines, dart_lines


def generate_dashboard(features):
    """Generate the dashboard markdown."""
    now = datetime.now().strftime('%Y-%m-%d %H:%M')
    total = len(features)
    complete = sum(1 for f in features if f['status'] == 'complete')
    pending = total - complete

    # Module breakdown
    modules = defaultdict(lambda: {'total': 0, 'complete': 0, 'features': []})
    for f in features:
        mod = f['module']
        modules[mod]['total'] += 1
        if f['status'] == 'complete':
            modules[mod]['complete'] += 1
        modules[mod]['features'].append(f)

    # Platform coverage
    platform_counts = Counter()
    for f in features:
        for p in f.get('platforms', []):
            platform_counts[p] += 1

    # Test and code counts
    test_count = count_rust_tests()
    rust_lines, dart_lines = count_lines_of_code()

    # Features with tests vs without
    with_tests = sum(1 for f in features if f.get('tests'))
    without_tests = total - with_tests

    # Features with docs vs without
    with_docs = sum(1 for f in features if f.get('docs'))
    without_docs = total - with_docs

    lines = []
    lines.append("# WIOS Project Dashboard")
    lines.append(f"\n> Auto-generated: {now}")
    lines.append("")

    # Overall status
    pct = (complete / total * 100) if total > 0 else 0
    lines.append("## Overall Status")
    lines.append("")
    lines.append(f"| Metric | Value |")
    lines.append(f"|--------|-------|")
    lines.append(f"| **Features** | {complete}/{total} ({pct:.0f}%) |")
    lines.append(f"| **Rust Tests** | {test_count} |")
    lines.append(f"| **Rust LOC** | {rust_lines:,} |")
    lines.append(f"| **Dart LOC** | {dart_lines:,} |")
    lines.append(f"| **Total LOC** | {rust_lines + dart_lines:,} |")
    lines.append(f"| **Feature Coverage (tested)** | {with_tests}/{total} ({with_tests/total*100:.0f}%) |")
    lines.append(f"| **Doc Coverage** | {with_docs}/{total} ({with_docs/total*100:.0f}%) |")
    lines.append("")

    # Module progress
    lines.append("## Module Progress")
    lines.append("")
    lines.append("| Module | Features | Complete | Progress |")
    lines.append("|--------|----------|----------|----------|")
    for mod_name in sorted(modules.keys()):
        m = modules[mod_name]
        bar_pct = m['complete'] / m['total'] * 100 if m['total'] > 0 else 0
        bar = '█' * int(bar_pct / 10) + '░' * (10 - int(bar_pct / 10))
        lines.append(f"| {mod_name} | {m['total']} | {m['complete']} | {bar} {bar_pct:.0f}% |")
    lines.append("")

    # Platform coverage
    lines.append("## Platform Coverage")
    lines.append("")
    lines.append("| Platform | Features |")
    lines.append("|----------|----------|")
    for plat in ['android', 'ios', 'windows', 'linux', 'macos', 'web']:
        count = platform_counts.get(plat, 0)
        lines.append(f"| {plat.capitalize()} | {count}/{total} |")
    lines.append("")

    # Feature health
    lines.append("## Feature Health")
    lines.append("")
    lines.append("| Category | Count | Status |")
    lines.append("|----------|-------|--------|")
    lines.append(f"| Registered | {total} | ✅ |")
    lines.append(f"| Implemented | {complete} | {'✅' if complete == total else '⚠️'} |")
    lines.append(f"| With Tests | {with_tests} | {'✅' if with_tests == total else '⚠️'} |")
    lines.append(f"| With Docs | {with_docs} | {'✅' if with_docs == total else '⚠️'} |")
    lines.append(f"| Without Tests | {without_tests} | {'✅' if without_tests == 0 else '⚠️'} |")
    lines.append("")

    # CI/CD status
    lines.append("## CI/CD Pipeline")
    lines.append("")
    lines.append("| Stage | Status |")
    lines.append("|-------|--------|")
    lines.append("| Format (rustfmt) | ✅ Configured |")
    lines.append("| Lint (clippy) | ✅ Configured |")
    lines.append("| Security (cargo-audit) | ✅ Configured |")
    lines.append("| Tests (cargo test) | ✅ Configured |")
    lines.append("| Coverage (tarpaulin) | ✅ Configured |")
    lines.append("| SBOM (cyclonedx) | ✅ Configured |")
    lines.append("| Feature Validation | ✅ Configured |")
    lines.append("| Doc Validation | ✅ Configured |")
    lines.append("| Build (7 platforms) | ✅ Configured |")
    lines.append("")

    # Release readiness
    all_complete = complete == total
    lines.append("## Release Readiness")
    lines.append("")
    if all_complete:
        lines.append("> ✅ **All features complete. Project is release-ready.**")
    else:
        lines.append(f"> ⚠️ {pending} features pending. Review before release.")
    lines.append("")

    return '\n'.join(lines)


def main():
    print("=" * 60)
    print("WIOS Dashboard Generator")
    print("=" * 60)

    registry = load_registry()
    features = registry.get('features', [])

    dashboard = generate_dashboard(features)

    output_path = REPO_ROOT / "docs" / "DASHBOARD.md"
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(dashboard)

    print(f"\n[PASS] Dashboard generated: {output_path}")
    print(f"   Features: {len(features)}")
    return 0


if __name__ == '__main__':
    sys.exit(main())
