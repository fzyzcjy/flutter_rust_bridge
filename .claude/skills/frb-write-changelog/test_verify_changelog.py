from __future__ import annotations

import importlib.util
import sys
from pathlib import Path
from typing import Any


def load_verify_changelog_module() -> Any:
    module_path = Path(__file__).with_name("verify_changelog.py")
    spec = importlib.util.spec_from_file_location("verify_changelog", module_path)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


verify_changelog = load_verify_changelog_module()


def test_verify_changelog_accepts_complete_section_with_grouped_prs() -> None:
    """Verifier accepts complete PR coverage and third-party thanks attribution."""

    merged_prs = [
        make_pr(number=3, author_login="external"),
        make_pr(number=2, author_login="fzyzcjy"),
        make_pr(number=1, author_login="app/allcontributors", title="docs: add external as a contributor for code"),
    ]
    changelog_text = """
# Changelog

## 2.0.0

* Add feature #3 (thanks @external)
* Improve CI #2

## 1.0.0
"""

    result = verify_changelog.verify_changelog(
        changelog_text=changelog_text,
        merged_prs=merged_prs,
        version="2.0.0",
        previous_release_time=verify_changelog.parse_datetime("2026-01-01T00:00:00Z"),
        local_authors={"fzyzcjy"},
        ignored_pr_numbers=set(),
        extra_local_pr_numbers=set(),
    )

    assert result.ok is True
    assert result.expected_pr_numbers == [3, 2]
    assert result.actual_pr_numbers == [3, 2]
    assert result.expected_thanks_authors == ["external"]
    assert result.actual_thanks_authors == ["external"]


def test_verify_changelog_reports_missing_extra_and_duplicate_pr_numbers() -> None:
    """Verifier reports PR number coverage problems."""

    merged_prs = [
        make_pr(number=3),
        make_pr(number=2),
    ]
    changelog_text = """
# Changelog

## 2.0.0

* Add feature #3 #3 #99

## 1.0.0
"""

    result = verify_changelog.verify_changelog(
        changelog_text=changelog_text,
        merged_prs=merged_prs,
        version="2.0.0",
        previous_release_time=verify_changelog.parse_datetime("2026-01-01T00:00:00Z"),
        local_authors={"fzyzcjy"},
        ignored_pr_numbers=set(),
        extra_local_pr_numbers=set(),
    )

    assert result.ok is False
    assert result.duplicate_pr_numbers == [3]
    assert result.missing_pr_numbers == [2]
    assert result.extra_pr_numbers == [99]


def test_verify_changelog_reports_thanks_attribution_problems() -> None:
    """Verifier reports missing and extra third-party thanks."""

    merged_prs = [
        make_pr(number=4, author_login="alice"),
        make_pr(number=3, author_login="bob"),
        make_pr(number=2, author_login="fzyzcjy"),
    ]
    changelog_text = """
# Changelog

## 2.0.0

* Add feature #4 #3 #2 (thanks @alice) (thanks @alice) (thanks @carol)

## 1.0.0
"""

    result = verify_changelog.verify_changelog(
        changelog_text=changelog_text,
        merged_prs=merged_prs,
        version="2.0.0",
        previous_release_time=verify_changelog.parse_datetime("2026-01-01T00:00:00Z"),
        local_authors={"fzyzcjy"},
        ignored_pr_numbers=set(),
        extra_local_pr_numbers=set(),
    )

    assert result.ok is False
    assert result.missing_thanks_authors == ["bob"]
    assert result.extra_thanks_authors == ["carol"]


def test_verify_changelog_accepts_duplicate_thanks_author() -> None:
    """Verifier accepts thanking the same third-party author on multiple entries."""

    merged_prs = [
        make_pr(number=4, author_login="alice"),
        make_pr(number=3, author_login="alice"),
        make_pr(number=2),
    ]
    changelog_text = """
# Changelog

## 2.0.0

* Add first feature #4 (thanks @alice)
* Add second feature #3 (thanks @alice)
* Improve CI #2

## 1.0.0
"""

    result = verify_changelog.verify_changelog(
        changelog_text=changelog_text,
        merged_prs=merged_prs,
        version="2.0.0",
        previous_release_time=verify_changelog.parse_datetime("2026-01-01T00:00:00Z"),
        local_authors={"fzyzcjy"},
        ignored_pr_numbers=set(),
        extra_local_pr_numbers=set(),
    )

    assert result.ok is True
    assert result.expected_thanks_authors == ["alice"]
    assert result.actual_thanks_authors == ["alice", "alice"]


def test_verify_changelog_reports_thanks_order_problems() -> None:
    """Verifier reports thanks entries placed after entries without thanks."""

    merged_prs = [
        make_pr(number=4, author_login="alice"),
        make_pr(number=3),
        make_pr(number=2, author_login="bob"),
    ]
    changelog_text = """
# Changelog

## 2.0.0

* Please refer to https://example.com for what's changed.
* Add first feature #4 (thanks @alice)
* Improve CI #3
* Add second feature #2 (thanks @bob)

## 1.0.0
"""

    result = verify_changelog.verify_changelog(
        changelog_text=changelog_text,
        merged_prs=merged_prs,
        version="2.0.0",
        previous_release_time=verify_changelog.parse_datetime("2026-01-01T00:00:00Z"),
        local_authors={"fzyzcjy"},
        ignored_pr_numbers=set(),
        extra_local_pr_numbers=set(),
    )

    assert result.ok is False
    assert result.thanks_order_violations == ["* Add second feature #2 (thanks @bob)"]


def test_verify_changelog_accepts_stacked_local_pr() -> None:
    """Verifier accepts an explicitly included local PR missing from merged PR JSON."""

    merged_prs = [
        make_pr(number=3),
        make_pr(number=2),
    ]
    changelog_text = """
# Changelog

## 2.0.0

* Improve release tooling #4 #3 #2

## 1.0.0
"""

    result = verify_changelog.verify_changelog(
        changelog_text=changelog_text,
        merged_prs=merged_prs,
        version="2.0.0",
        previous_release_time=verify_changelog.parse_datetime("2026-01-01T00:00:00Z"),
        local_authors={"fzyzcjy"},
        ignored_pr_numbers=set(),
        extra_local_pr_numbers={4},
    )

    assert result.ok is True
    assert result.expected_pr_numbers == [4, 3, 2]
    assert result.actual_pr_numbers == [4, 3, 2]


def make_pr(
    *,
    number: int,
    author_login: str = "fzyzcjy",
    title: str = "Add feature",
) -> Any:
    return verify_changelog.PullRequestInfo(
        number=number,
        title=title,
        author_login=author_login,
        author_is_bot=author_login.startswith("app/"),
        merged_at=verify_changelog.parse_datetime("2026-02-01T00:00:00Z"),
        base_ref_name="master",
    )
