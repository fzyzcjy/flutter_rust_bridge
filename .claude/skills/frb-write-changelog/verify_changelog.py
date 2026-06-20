#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["typer>=0.12"]
# ///
from __future__ import annotations

import json
import re
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Annotated, Any, TypeVar

import typer


app = typer.Typer(add_completion=False)
DuplicateValue = TypeVar("DuplicateValue", int, str)


@dataclass(frozen=True)
class PullRequestInfo:
    number: int
    title: str
    author_login: str
    author_is_bot: bool
    merged_at: datetime
    base_ref_name: str


@dataclass(frozen=True)
class VerificationResult:
    version: str
    expected_pr_numbers: list[int]
    actual_pr_numbers: list[int]
    duplicate_pr_numbers: list[int]
    missing_pr_numbers: list[int]
    extra_pr_numbers: list[int]
    expected_thanks_authors: list[str]
    actual_thanks_authors: list[str]
    missing_thanks_authors: list[str]
    extra_thanks_authors: list[str]
    thanks_order_violations: list[str]

    @property
    def ok(self) -> bool:
        return not any(
            [
                self.duplicate_pr_numbers,
                self.missing_pr_numbers,
                self.extra_pr_numbers,
                self.missing_thanks_authors,
                self.extra_thanks_authors,
                self.thanks_order_violations,
            ],
        )


def verify_changelog(
    *,
    changelog_text: str,
    merged_prs: list[PullRequestInfo],
    version: str,
    previous_release_time: datetime,
    release_time: datetime | None,
    local_authors: set[str],
    ignored_pr_numbers: set[int],
    extra_local_pr_numbers: set[int],
) -> VerificationResult:
    section = extract_changelog_section(
        changelog_text=changelog_text,
        version=version,
    )
    expected_prs = expected_prs_for_release(
        merged_prs=merged_prs,
        previous_release_time=previous_release_time,
        release_time=release_time,
        ignored_pr_numbers=ignored_pr_numbers,
    )

    actual_pr_numbers = extract_pr_numbers(section)
    expected_pr_numbers = sorted(
        {pr.number for pr in expected_prs} | extra_local_pr_numbers,
        reverse=True,
    )

    actual_thanks_authors = extract_thanks_authors(section)
    expected_thanks_authors = sorted(
        {
            pr.author_login
            for pr in expected_prs
            if should_thank_author(pr=pr, local_authors=local_authors)
        },
    )

    return VerificationResult(
        version=version,
        expected_pr_numbers=expected_pr_numbers,
        actual_pr_numbers=actual_pr_numbers,
        duplicate_pr_numbers=find_duplicates(actual_pr_numbers),
        missing_pr_numbers=sorted(set(expected_pr_numbers) - set(actual_pr_numbers)),
        extra_pr_numbers=sorted(set(actual_pr_numbers) - set(expected_pr_numbers)),
        expected_thanks_authors=expected_thanks_authors,
        actual_thanks_authors=sorted(actual_thanks_authors),
        missing_thanks_authors=sorted(
            set(expected_thanks_authors) - set(actual_thanks_authors),
        ),
        extra_thanks_authors=sorted(
            set(actual_thanks_authors) - set(expected_thanks_authors),
        ),
        thanks_order_violations=find_thanks_order_violations(section),
    )


def extract_changelog_section(*, changelog_text: str, version: str) -> str:
    match = re.search(
        rf"^## {re.escape(version)}\n(?P<section>.*?)(?=^## |\Z)",
        changelog_text,
        flags=re.MULTILINE | re.DOTALL,
    )
    if match is None:
        raise ValueError(f"Could not find changelog section for {version}")

    return match.group("section")


def expected_prs_for_release(
    *,
    merged_prs: list[PullRequestInfo],
    previous_release_time: datetime,
    release_time: datetime | None,
    ignored_pr_numbers: set[int],
) -> list[PullRequestInfo]:
    return [
        pr
        for pr in merged_prs
        if pr.merged_at > previous_release_time
        and (release_time is None or pr.merged_at <= release_time)
        and pr.number not in ignored_pr_numbers
        and not is_all_contributors_pr(pr)
    ]


def should_thank_author(
    *,
    pr: PullRequestInfo,
    local_authors: set[str],
) -> bool:
    return not pr.author_is_bot and pr.author_login not in local_authors


def is_all_contributors_pr(pr: PullRequestInfo) -> bool:
    return pr.author_login == "app/allcontributors" or re.match(
        r"^docs: add .+ as a contributor(?: for .+)?$",
        pr.title,
    ) is not None


def extract_pr_numbers(section: str) -> list[int]:
    return [int(raw_number) for raw_number in re.findall(r"#(\d+)", section)]


def extract_thanks_authors(section: str) -> list[str]:
    return re.findall(r"\(thanks @([A-Za-z0-9-]+)\)", section)


def find_thanks_order_violations(section: str) -> list[str]:
    seen_entry_without_thanks = False
    violations: list[str] = []
    for entry in extract_pr_entry_lines(section):
        if "(thanks @" in entry:
            if seen_entry_without_thanks:
                violations.append(entry)
        else:
            seen_entry_without_thanks = True

    return violations


def extract_pr_entry_lines(section: str) -> list[str]:
    return [
        line.strip()
        for line in section.splitlines()
        if line.startswith("* ") and re.search(r"#\d+", line) is not None
    ]


def find_duplicates(values: list[DuplicateValue]) -> list[DuplicateValue]:
    seen: set[DuplicateValue] = set()
    duplicates: set[DuplicateValue] = set()
    for value in values:
        if value in seen:
            duplicates.add(value)
        else:
            seen.add(value)

    return sorted(duplicates)


def load_merged_prs(path: Path) -> list[PullRequestInfo]:
    raw_data = json.loads(path.read_text())
    if not isinstance(raw_data, list):
        raise ValueError("Merged PR JSON must contain a list")

    return [parse_pr(raw_pr) for raw_pr in raw_data]


def parse_pr(raw_pr: Any) -> PullRequestInfo:
    if not isinstance(raw_pr, dict):
        raise ValueError(f"PR entry must be an object: {raw_pr!r}")
    author = raw_pr.get("author")
    if not isinstance(author, dict):
        raise ValueError(f"PR author must be an object: {raw_pr!r}")

    return PullRequestInfo(
        number=int(raw_pr["number"]),
        title=str(raw_pr["title"]),
        author_login=str(author["login"]),
        author_is_bot=bool(author.get("is_bot", False)),
        merged_at=parse_datetime(str(raw_pr["mergedAt"])),
        base_ref_name=str(raw_pr.get("baseRefName", "")),
    )


def parse_datetime(raw_value: str) -> datetime:
    return datetime.fromisoformat(raw_value.replace("Z", "+00:00"))


def format_result(result: VerificationResult) -> str:
    status = "OK" if result.ok else "FAILED"
    return "\n".join(
        [
            f"Changelog verification: {status}",
            f"Version: {result.version}",
            f"Expected PR numbers: {len(result.expected_pr_numbers)}",
            f"Actual PR numbers: {len(result.actual_pr_numbers)}",
            f"Duplicate PR numbers: {result.duplicate_pr_numbers}",
            f"Missing PR numbers: {result.missing_pr_numbers}",
            f"Extra PR numbers: {result.extra_pr_numbers}",
            f"Expected third-party thanks authors: {result.expected_thanks_authors}",
            f"Actual third-party thanks authors: {result.actual_thanks_authors}",
            f"Missing thanks authors: {result.missing_thanks_authors}",
            f"Extra thanks authors: {result.extra_thanks_authors}",
            f"Thanks ordering violations: {result.thanks_order_violations}",
        ],
    )


@app.command()
def main(
    version: Annotated[
        str,
        typer.Option("--version", help="Changelog section version to verify."),
    ],
    previous_release_time: Annotated[
        str,
        typer.Option(
            "--previous-release-time",
            help="Previous release timestamp, for example 2026-03-29T21:34:04+08:00.",
        ),
    ],
    merged_prs_json: Annotated[
        Path,
        typer.Option(
            "--merged-prs-json",
            help="Path to gh pr list JSON output.",
        ),
    ],
    release_time: Annotated[
        str | None,
        typer.Option(
            "--release-time",
            help="Target release timestamp. Use this when verifying an already-published release.",
        ),
    ] = None,
    changelog_path: Annotated[
        Path,
        typer.Option("--changelog", help="Path to CHANGELOG.md."),
    ] = Path("CHANGELOG.md"),
    local_author: Annotated[
        list[str],
        typer.Option(
            "--local-author",
            help="Author login that should not receive thanks attribution.",
        ),
    ] = ["fzyzcjy"],
    ignore_pr: Annotated[
        list[int],
        typer.Option(
            "--ignore-pr",
            help="Merged PR number intentionally excluded from the changelog.",
        ),
    ] = [],
    extra_local_pr: Annotated[
        list[int],
        typer.Option(
            "--extra-local-pr",
            help="PR number expected in the changelog but absent from the merged PR JSON, authored by a local maintainer.",
        ),
    ] = [],
) -> None:
    result = verify_changelog(
        changelog_text=changelog_path.read_text(),
        merged_prs=load_merged_prs(merged_prs_json),
        version=version,
        previous_release_time=parse_datetime(previous_release_time),
        release_time=(
            parse_datetime(release_time) if release_time is not None else None
        ),
        local_authors=set(local_author),
        ignored_pr_numbers=set(ignore_pr),
        extra_local_pr_numbers=set(extra_local_pr),
    )

    typer.echo(format_result(result))
    if not result.ok:
        raise typer.Exit(code=1)


if __name__ == "__main__":
    app()
