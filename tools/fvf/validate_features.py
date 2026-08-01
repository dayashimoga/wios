#!/usr/bin/env python3
"""WIOS Feature Verification Framework — Feature Validator.

Parses feature_registry.yaml, verifies all linked files exist,
detects placeholders/mocks, generates RTM, and exits non-zero on failure.
"""

import os
import re
import sys
import json
import yaml
from pathlib import Path
from datetime import datetime

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
REGISTRY_PATH = Path(__file__).resolve().parent / "feature_registry.yaml"

PLACEHOLDER_PATTERNS = [
    r'\bTODO\b', r'\bFIXME\b', r'\bHACK\b', r'\bXXX\b',
    r'\bunimplemented!\b', r'\btodo!\b',
    r'dummy', r'placeholder', r'mock_data', r'hardcoded',
    r'stub\(\)', r'fake_', r'sample_data',
]

PLACEHOLDER_RE = re.compile('|'.join(PLACEHOLDER_PATTERNS), re.IGNORECASE)

# Files/patterns to ignore in placeholder detection
IGNORE_PATTERNS = [
    'feature_registry.yaml', 'validate_features.py', 'validate_docs.py',
    'CHANGELOG.md', 'TODO.md', 'DECISIONS.md', 'KNOWN_LIMITATIONS.md',
    '.git/', 'target/', 'build/', 'node_modules/',
]


def load_registry():
    with open(REGISTRY_PATH) as f:
        return yaml.safe_load(f)


def check_file_exists(path_str):
    """Check if a file exists relative to repo root."""
    full = REPO_ROOT / path_str
    return full.exists()


def scan_for_placeholders(path_str):
    """Scan a source file for placeholder patterns."""
    full = REPO_ROOT / path_str
    if not full.exists():
        return []

    if any(ig in str(full) for ig in IGNORE_PATTERNS):
        return []

    findings = []
    try:
        with open(full, encoding='utf-8', errors='ignore') as f:
            for i, line in enumerate(f, 1):
                # Skip comments that are legitimate documentation
                stripped = line.strip()
                if stripped.startswith('//!') or stripped.startswith('///'):
                    continue
                matches = PLACEHOLDER_RE.findall(line)
                if matches:
                    findings.append((i, matches, stripped[:100]))
    except Exception:
        pass
    return findings


def validate_feature(feature):
    """Validate a single feature entry. Returns (errors, warnings)."""
    errors = []
    warnings = []
    fid = feature['id']

    # Required fields
    for field in ['id', 'name', 'module', 'requirement', 'code', 'status']:
        if field not in feature or not feature[field]:
            errors.append(f"[{fid}] Missing required field: {field}")

    # Verify code files exist
    for code_file in feature.get('code', []):
        if not check_file_exists(code_file):
            errors.append(f"[{fid}] Code file not found: {code_file}")

    # Verify UI files exist
    for ui_file in feature.get('ui', []):
        if '/' in ui_file:
            ui_path = f"apps/wios_app/lib/{ui_file}"
            if not check_file_exists(ui_path):
                errors.append(f"[{fid}] UI file not found: {ui_path}")

    # Scan code for placeholders
    for code_file in feature.get('code', []):
        placeholders = scan_for_placeholders(code_file)
        for line_no, matches, context in placeholders:
            warnings.append(
                f"[{fid}] Placeholder in {code_file}:{line_no}: {', '.join(matches)}"
            )

    # Check test linkage
    if not feature.get('tests') and feature['status'] == 'complete':
        warnings.append(f"[{fid}] No tests linked for complete feature")

    # Check doc linkage
    if not feature.get('docs') and feature['status'] == 'complete':
        warnings.append(f"[{fid}] No docs linked for complete feature")

    return errors, warnings


def generate_rtm(features):
    """Generate Requirement Traceability Matrix."""
    rtm = []
    for f in features:
        code_str = ', '.join(f.get('code', [])[:2])
        api_str = ', '.join(f.get('api', [])[:2]) or '—'
        ui_str = ', '.join(f.get('ui', [])[:1]) or '—'
        test_str = ', '.join(f.get('tests', [])[:2]) or '—'
        doc_str = ', '.join(f.get('docs', [])[:2]) or '—'
        plat_str = ', '.join(f.get('platforms', [])[:3])
        if len(f.get('platforms', [])) > 3:
            plat_str += '...'
        status = '✅' if f['status'] == 'complete' else '❌'

        rtm.append({
            'requirement': f['requirement'],
            'feature_id': f['id'],
            'feature_name': f['name'],
            'code': code_str,
            'api': api_str,
            'ui': ui_str,
            'tests': test_str,
            'docs': doc_str,
            'platforms': plat_str,
            'status': status,
        })
    return rtm


def write_rtm_markdown(rtm, output_path):
    """Write RTM as markdown file."""
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("# WIOS Requirement Traceability Matrix (RTM)\n\n")
        f.write(f"> Auto-generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}\n\n")
        f.write("| Req | Feature ID | Name | Code | API | UI | Tests | Docs | Platforms | Status |\n")
        f.write("|-----|-----------|------|------|-----|----|-------|------|-----------|--------|\n")
        for row in rtm:
            f.write(f"| {row['requirement']} | {row['feature_id']} | {row['feature_name']} | "
                    f"`{row['code']}` | {row['api']} | {row['ui']} | {row['tests']} | "
                    f"{row['docs']} | {row['platforms']} | {row['status']} |\n")
        f.write(f"\n**Total features: {len(rtm)}** | "
                f"**Complete: {sum(1 for r in rtm if r['status'] == '✅')}** | "
                f"**Pending: {sum(1 for r in rtm if r['status'] == '❌')}**\n")


def write_rtm_json(rtm, output_path):
    """Write RTM as JSON."""
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump({'generated': datetime.now().isoformat(), 'features': rtm}, f, indent=2)


def main():
    print("=" * 60)
    print("WIOS Feature Verification Framework")
    print("=" * 60)

    registry = load_registry()
    features = registry.get('features', [])
    print(f"\nRegistry: {len(features)} features loaded")

    all_errors = []
    all_warnings = []

    for feature in features:
        errors, warnings = validate_feature(feature)
        all_errors.extend(errors)
        all_warnings.extend(warnings)

    # Generate RTM
    rtm = generate_rtm(features)
    rtm_md_path = REPO_ROOT / "docs" / "RTM.md"
    rtm_json_path = REPO_ROOT / "docs" / "rtm.json"
    write_rtm_markdown(rtm, rtm_md_path)
    write_rtm_json(rtm, rtm_json_path)
    print(f"\nRTM generated: {rtm_md_path}")

    # Summary
    complete = sum(1 for f in features if f['status'] == 'complete')
    pending = len(features) - complete

    print(f"\n{'-' * 40}")
    print(f"Features: {len(features)} total, {complete} complete, {pending} pending")
    print(f"Errors:   {len(all_errors)}")
    print(f"Warnings: {len(all_warnings)}")

    if all_warnings:
        print(f"\n[WARN] Warnings:")
        for w in all_warnings[:20]:
            print(f"  {w}")
        if len(all_warnings) > 20:
            print(f"  ... and {len(all_warnings) - 20} more")

    if all_errors:
        print(f"\n[FAIL] Errors:")
        for e in all_errors:
            print(f"  {e}")
        print(f"\nFEATURE VALIDATION FAILED")
        return 1

    print(f"\n[PASS] FEATURE VALIDATION PASSED")
    return 0


if __name__ == '__main__':
    sys.exit(main())
