from __future__ import annotations

import importlib.util
import sys
from pathlib import Path
from typing import Any


def load_release_ci_gate_module() -> Any:
    module_path = Path(__file__).with_name("release_ci_gate.py")
    spec = importlib.util.spec_from_file_location("release_ci_gate", module_path)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


release_ci_gate = load_release_ci_gate_module()


def test_classify_release_ci_gate_allows_non_release_paths() -> None:
    """Non-release tooling and documentation paths can bypass a fresh CI wait."""

    result = release_ci_gate.classify_release_ci_gate(
        changed_paths=[
            ".claude/skills/frb-publish-release/SKILL.md",
            ".devcontainer/Dockerfile",
            ".github/workflows/publish_dev_docker.yaml",
            "website/docs/index.md",
        ],
        base_ref="green",
        release_ref="release",
    )

    assert result.allowed is True
    assert result.blocked_paths == []


def test_classify_release_ci_gate_blocks_release_surface_paths() -> None:
    """Package, changelog, and release script changes still require normal CI."""

    result = release_ci_gate.classify_release_ci_gate(
        changed_paths=[
            "CHANGELOG.md",
            "Cargo.toml",
            "frb_dart/pubspec.yaml",
            "frb_rust/src/lib.rs",
            "tools/frb_internal/lib/src/release/releaser.dart",
        ],
        base_ref="green",
        release_ref="release",
    )

    assert result.allowed is False
    assert [path.path for path in result.blocked_paths] == [
        "CHANGELOG.md",
        "Cargo.toml",
        "frb_dart/pubspec.yaml",
        "frb_rust/src/lib.rs",
        "tools/frb_internal/lib/src/release/releaser.dart",
    ]


def test_classify_release_ci_gate_allows_empty_diff() -> None:
    """A release ref equal to the green ref has no extra CI risk."""

    result = release_ci_gate.classify_release_ci_gate(
        changed_paths=[],
        base_ref="green",
        release_ref="green",
    )

    assert result.allowed is True
    assert result.changed_paths == []


def test_format_result_lists_blocked_paths() -> None:
    """Human output explains why an exception is rejected."""

    result = release_ci_gate.classify_release_ci_gate(
        changed_paths=["frb_codegen/src/lib.rs", "book/src/index.md"],
        base_ref="green",
        release_ref="release",
    )

    text = release_ci_gate.format_result(result)

    assert "Release CI gate exception: BLOCKED" in text
    assert "BLOCK frb_codegen/src/lib.rs" in text
    assert "ALLOW book/src/index.md (book/**)" in text
