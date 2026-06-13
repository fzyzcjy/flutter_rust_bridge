#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["typer>=0.12"]
# ///
from __future__ import annotations

import fnmatch
import json
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Annotated

import typer


DEFAULT_ALLOWED_PATTERNS = (
    ".agents/**",
    ".claude/**",
    ".devcontainer/**",
    ".github/ISSUE_TEMPLATE/**",
    ".github/PULL_REQUEST_TEMPLATE.md",
    ".github/config.yml",
    ".github/dependabot.yml",
    ".github/settings.yml",
    ".github/workflows/ci.yaml",
    ".github/workflows/no_response.yml",
    ".github/workflows/precommit_autofix.yaml",
    ".github/workflows/precommit_autofix_comment.yaml",
    ".github/workflows/publish_dev_docker.yaml",
    ".github/workflows/remove_labels.yml",
    ".github/workflows/stale.yml",
    "AGENTS.md",
    "CLAUDE.md",
    "CODE_OF_CONDUCT.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
    "book/**",
    "codecov.yml",
    "tools/manual_tests/**",
    "tools/tart_macos/**",
    "website/**",
)


@dataclass(frozen=True)
class ClassifiedPath:
    path: str
    allowed: bool
    matched_pattern: str | None


@dataclass(frozen=True)
class GateResult:
    allowed: bool
    base_ref: str
    release_ref: str
    changed_paths: list[ClassifiedPath]

    @property
    def blocked_paths(self) -> list[ClassifiedPath]:
        return [path for path in self.changed_paths if not path.allowed]


app = typer.Typer(add_completion=False)


def classify_release_ci_gate(
    *,
    changed_paths: list[str],
    base_ref: str,
    release_ref: str,
    allowed_patterns: tuple[str, ...] = DEFAULT_ALLOWED_PATTERNS,
) -> GateResult:
    classified_paths = [
        classify_path(path=path, allowed_patterns=allowed_patterns)
        for path in sorted(changed_paths)
    ]

    return GateResult(
        allowed=not any(not path.allowed for path in classified_paths),
        base_ref=base_ref,
        release_ref=release_ref,
        changed_paths=classified_paths,
    )


def classify_path(*, path: str, allowed_patterns: tuple[str, ...]) -> ClassifiedPath:
    normalized_path = path.strip()
    if normalized_path.startswith("./"):
        normalized_path = normalized_path[2:]

    for pattern in allowed_patterns:
        if fnmatch.fnmatchcase(normalized_path, pattern):
            return ClassifiedPath(
                path=normalized_path,
                allowed=True,
                matched_pattern=pattern,
            )

    return ClassifiedPath(
        path=normalized_path,
        allowed=False,
        matched_pattern=None,
    )


def changed_paths_between_refs(
    *,
    repo_root: Path,
    base_ref: str,
    release_ref: str,
) -> list[str]:
    result = subprocess.run(
        [
            "git",
            "-C",
            str(repo_root),
            "diff",
            "--name-only",
            f"{base_ref}..{release_ref}",
        ],
        check=True,
        text=True,
        capture_output=True,
    )

    return [line for line in result.stdout.splitlines() if line.strip()]


def git_repo_root() -> Path:
    result = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        check=True,
        text=True,
        capture_output=True,
    )

    return Path(result.stdout.strip())


def result_to_dict(result: GateResult) -> dict[str, object]:
    return {
        "allowed": result.allowed,
        "baseRef": result.base_ref,
        "releaseRef": result.release_ref,
        "changedPaths": [
            {
                "path": path.path,
                "allowed": path.allowed,
                "matchedPattern": path.matched_pattern,
            }
            for path in result.changed_paths
        ],
        "blockedPaths": [path.path for path in result.blocked_paths],
    }


def format_result(result: GateResult) -> str:
    status = "ALLOWED" if result.allowed else "BLOCKED"
    lines = [
        f"Release CI gate exception: {status}",
        f"Base green ref: {result.base_ref}",
        f"Release ref: {result.release_ref}",
        f"Changed paths: {len(result.changed_paths)}",
    ]

    for path in result.changed_paths:
        if path.allowed:
            lines.append(f"  ALLOW {path.path} ({path.matched_pattern})")
        else:
            lines.append(f"  BLOCK {path.path}")

    if result.allowed:
        lines.append("Decision: all changes are in the non-release allowlist.")
    else:
        lines.append("Decision: wait for normal CI or remove the blocked changes from the release commit.")

    return "\n".join(lines)


@app.command()
def main(
    base_green_ref: Annotated[
        str,
        typer.Option(
            "--base-green-ref",
            help="Last commit whose normal CI was green.",
        ),
    ],
    release_ref: Annotated[
        str,
        typer.Option(
            "--release-ref",
            help="Commit intended for release. Defaults to HEAD.",
        ),
    ] = "HEAD",
    repo_root: Annotated[
        Path | None,
        typer.Option(
            "--repo-root",
            help="Repository root. Defaults to git rev-parse --show-toplevel.",
        ),
    ] = None,
    json_output: Annotated[
        bool,
        typer.Option(
            "--json",
            help="Print machine-readable JSON.",
        ),
    ] = False,
) -> None:
    resolved_repo_root = repo_root if repo_root is not None else git_repo_root()
    changed_paths = changed_paths_between_refs(
        repo_root=resolved_repo_root,
        base_ref=base_green_ref,
        release_ref=release_ref,
    )
    result = classify_release_ci_gate(
        changed_paths=changed_paths,
        base_ref=base_green_ref,
        release_ref=release_ref,
    )

    if json_output:
        typer.echo(json.dumps(result_to_dict(result), indent=2, sort_keys=True))
    else:
        typer.echo(format_result(result))

    if not result.allowed:
        raise typer.Exit(code=1)


if __name__ == "__main__":
    app()
