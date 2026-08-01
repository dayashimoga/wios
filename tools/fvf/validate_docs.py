#!/usr/bin/env python3
"""WIOS Feature Verification Framework — Documentation Validator.

Verifies all required docs exist, are non-empty, reference current
modules, and stay synchronized with the implementation.
"""

import os
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
DOCS_DIR = REPO_ROOT / "docs"

REQUIRED_DOCS = [
    "README.md",            # In repo root
    "ARCHITECTURE.md",
    "IMPLEMENTATION.md",
    "REQUIREMENTS.md",
    "API.md",
    "SDK.md",
    "SECURITY.md",
    "TESTING.md",
    "DEPLOYMENT.md",
    "CODE_STRUCTURE.md",
    "ROADMAP.md",
    "CHANGELOG.md",
    "TODO.md",
    "DECISIONS.md",
    "KNOWN_LIMITATIONS.md",
]

# Modules that should be referenced in architecture/implementation docs
EXPECTED_MODULES = [
    'wios-core', 'wios-crypto', 'wios-storage', 'wios-network',
    'wios-ai', 'wios-compute', 'wios-bridge', 'wios-api', 'wios-cli',
]

MIN_DOC_BYTES = 200  # Minimum doc size to not be a stub


def check_doc_exists(name):
    """Check if doc exists in docs/ or repo root."""
    if name == "README.md":
        path = REPO_ROOT / name
    else:
        path = DOCS_DIR / name
    return path.exists(), path


def check_doc_size(path):
    """Check doc is not a stub."""
    if not path.exists():
        return False, 0
    size = path.stat().st_size
    return size >= MIN_DOC_BYTES, size


def check_module_references(path, modules):
    """Check if a doc references expected modules."""
    if not path.exists():
        return []
    content = path.read_text(encoding='utf-8', errors='ignore')
    missing = []
    for mod in modules:
        if mod not in content:
            missing.append(mod)
    return missing


def check_changelog_has_version(path, version="0.5.0"):
    """Check CHANGELOG has current version entry."""
    if not path.exists():
        return False
    content = path.read_text(encoding='utf-8', errors='ignore')
    return version in content


def check_todo_no_stale(path):
    """Check TODO doesn't have stale unchecked items that are actually done."""
    # This is a basic check — the FVF validates against the feature registry
    if not path.exists():
        return []
    return []  # Detailed check done by validate_features.py


def main():
    print("=" * 60)
    print("WIOS Documentation Validator")
    print("=" * 60)

    errors = []
    warnings = []

    # 1. Check all required docs exist
    print("\n[1] Checking required documents...")
    for doc_name in REQUIRED_DOCS:
        exists, path = check_doc_exists(doc_name)
        if not exists:
            errors.append(f"Missing required document: {doc_name}")
            print(f"  [FAIL] {doc_name} -- NOT FOUND")
        else:
            ok, size = check_doc_size(path)
            if not ok:
                warnings.append(f"Document too small ({size}B): {doc_name}")
                print(f"  [WARN] {doc_name} -- {size}B (may be a stub)")
            else:
                print(f"  [OK]   {doc_name} -- {size}B")

    # 2. Check ARCHITECTURE.md references all modules
    print("\n[2] Checking module references in ARCHITECTURE.md...")
    arch_path = DOCS_DIR / "ARCHITECTURE.md"
    if arch_path.exists():
        missing = check_module_references(arch_path, EXPECTED_MODULES)
        for mod in missing:
            warnings.append(f"ARCHITECTURE.md missing module reference: {mod}")
            print(f"  [WARN] Missing: {mod}")
        if not missing:
            print(f"  [OK]   All {len(EXPECTED_MODULES)} modules referenced")
    else:
        errors.append("ARCHITECTURE.md not found")

    # 3. Check CHANGELOG has current version
    print("\n[3] Checking CHANGELOG currency...")
    changelog_path = DOCS_DIR / "CHANGELOG.md"
    if check_changelog_has_version(changelog_path):
        print("  [OK]   Current version found in CHANGELOG")
    else:
        warnings.append("CHANGELOG.md missing current version entry")
        print("  [WARN] Current version not found in CHANGELOG")

    # 4. Check IMPLEMENTATION.md has test counts
    print("\n[4] Checking IMPLEMENTATION.md...")
    impl_path = DOCS_DIR / "IMPLEMENTATION.md"
    if impl_path.exists():
        content = impl_path.read_text(encoding='utf-8', errors='ignore')
        if 'tests' in content.lower() or 'passing' in content.lower():
            print("  [OK]   Test information present")
        else:
            warnings.append("IMPLEMENTATION.md missing test information")
            print("  [WARN] No test information found")
    else:
        errors.append("IMPLEMENTATION.md not found")

    # Summary
    print(f"\n{'-' * 40}")
    print(f"Documents: {len(REQUIRED_DOCS)} required")
    print(f"Errors:    {len(errors)}")
    print(f"Warnings:  {len(warnings)}")

    if all_warnings := warnings:
        print(f"\n[WARN] Warnings:")
        for w in all_warnings:
            print(f"  {w}")

    if errors:
        print(f"\n[FAIL] Errors:")
        for e in errors:
            print(f"  {e}")
        print(f"\nDOCUMENTATION VALIDATION FAILED")
        return 1

    print(f"\n[PASS] DOCUMENTATION VALIDATION PASSED")
    return 0


if __name__ == '__main__':
    sys.exit(main())
