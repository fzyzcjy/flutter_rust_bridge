#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["typer>=0.12"]
# ///

from __future__ import annotations

import dataclasses
import json
import re
import subprocess
from pathlib import PurePosixPath
from typing import Annotated, Literal

import typer


Category = Literal["test-only", "non-test-only", "mixed", "unclassified"]
app = typer.Typer(add_completion=False)


@dataclasses.dataclass(frozen=True)
class ChangedLines:
    old: frozenset[int]
    new: frozenset[int]
    binary: bool = False


@dataclasses.dataclass(frozen=True)
class FileResult:
    path: str
    category: Category
    test_lines: int
    non_test_lines: int


@app.command()
def main(
    base: Annotated[
        str,
        typer.Option("--base", help="Base revision used to compute the merge base."),
    ] = "origin/master",
    head: Annotated[
        str,
        typer.Option("--head", help="Head revision to classify."),
    ] = "HEAD",
    json_output: Annotated[
        bool,
        typer.Option("--json", help="Print machine-readable JSON."),
    ] = False,
) -> None:
    results = categorize_diff(base=base, head=head)

    if json_output:
        typer.echo(json.dumps([dataclasses.asdict(result) for result in results], indent=2))
    else:
        _print_text(results=results, base=base, head=head)


def categorize_diff(base: str, head: str) -> list[FileResult]:
    merge_base = _git("merge-base", base, head).strip()
    diff = _git(
        "diff",
        "--no-ext-diff",
        "--find-renames",
        "--unified=0",
        merge_base,
        head,
        "--",
    )
    changed = _parse_diff(diff)

    return [
        _categorize_file(
            path=path,
            changed_lines=changed_lines,
            base=merge_base,
            head=head,
        )
        for path, changed_lines in sorted(changed.items())
    ]


def _parse_diff(diff: str) -> dict[str, ChangedLines]:
    paths: dict[str, tuple[set[int], set[int], bool]] = {}
    current_path: str | None = None
    old_path: str | None = None
    old_line = 0
    new_line = 0

    for line in diff.splitlines():
        if line.startswith("diff --git "):
            current_path = None
            old_path = None
        elif line.startswith("--- a/"):
            old_path = line[6:]
        elif line.startswith("+++ b/"):
            current_path = line[6:]
            paths.setdefault(current_path, (set(), set(), False))
        elif line == "+++ /dev/null" and old_path is not None:
            current_path = old_path
            paths.setdefault(current_path, (set(), set(), False))
        elif line.startswith("Binary files ") and current_path is not None:
            old, new, _ = paths[current_path]
            paths[current_path] = (old, new, True)
        elif (match := re.match(r"@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@", line)):
            old_line = int(match.group(1))
            new_line = int(match.group(2))
        elif current_path is not None and line.startswith("+") and not line.startswith("+++"):
            paths[current_path][1].add(new_line)
            new_line += 1
        elif current_path is not None and line.startswith("-") and not line.startswith("---"):
            paths[current_path][0].add(old_line)
            old_line += 1
        elif current_path is not None and not line.startswith("\\"):
            old_line += 1
            new_line += 1

    return {
        path: ChangedLines(old=frozenset(old), new=frozenset(new), binary=binary)
        for path, (old, new, binary) in paths.items()
    }


def _categorize_file(
    path: str,
    changed_lines: ChangedLines,
    base: str,
    head: str,
) -> FileResult:
    old_source = _git_show(base, path)
    new_source = _git_show(head, path)
    substantive_old = changed_lines.old & _substantive_lines(old_source)
    substantive_new = changed_lines.new & _substantive_lines(new_source)
    total_lines = len(substantive_old) + len(substantive_new)
    if changed_lines.binary:
        return FileResult(path=path, category="unclassified", test_lines=0, non_test_lines=0)
    if _is_dedicated_test_path(path):
        return FileResult(
            path=path,
            category="test-only",
            test_lines=total_lines,
            non_test_lines=0,
        )

    old_test_lines = _rust_test_lines(old_source) if path.endswith(".rs") else set()
    new_test_lines = _rust_test_lines(new_source) if path.endswith(".rs") else set()
    test_lines = len(substantive_old & old_test_lines) + len(substantive_new & new_test_lines)
    non_test_lines = total_lines - test_lines
    category: Category
    if test_lines and non_test_lines:
        category = "mixed"
    elif test_lines:
        category = "test-only"
    else:
        category = "non-test-only"
    return FileResult(
        path=path,
        category=category,
        test_lines=test_lines,
        non_test_lines=non_test_lines,
    )


def _is_dedicated_test_path(path: str) -> bool:
    pure_path = PurePosixPath(path)
    parts = set(pure_path.parts)
    name = pure_path.name
    return bool(
        parts & {"test", "tests", "test_fixtures"}
        or name.startswith("test_")
        or name.endswith(("_test.dart", "_test.py", "_test.rs", ".test.ts", ".test.js"))
    )


def _substantive_lines(source: str | None) -> set[int]:
    if source is None:
        return set()
    return {
        line_number
        for line_number, line in enumerate(source.splitlines(), start=1)
        if line.strip()
    }


def _rust_test_lines(source: str | None) -> set[int]:
    if source is None:
        return set()
    masked = _mask_rust_comments_and_strings(source)
    lines = masked.splitlines()
    result: set[int] = set()
    index = 0

    while index < len(lines):
        if re.search(r"#\s*\[\s*(?:cfg\s*\(\s*test\s*\)|test)\s*\]", lines[index]):
            start = index
            cursor = index
            depth = 0
            opened = False
            while cursor < len(lines):
                for character in lines[cursor]:
                    if character == "{":
                        depth += 1
                        opened = True
                    elif character == "}" and opened:
                        depth -= 1
                if opened and depth == 0:
                    result.update(range(start + 1, cursor + 2))
                    index = cursor
                    break
                cursor += 1
        index += 1

    return result


def _mask_rust_comments_and_strings(source: str) -> str:
    output = list(source)
    index = 0
    block_depth = 0
    quote: str | None = None
    escaped = False

    while index < len(source):
        if block_depth:
            if source.startswith("/*", index):
                output[index : index + 2] = "  "
                block_depth += 1
                index += 2
            elif source.startswith("*/", index):
                output[index : index + 2] = "  "
                block_depth -= 1
                index += 2
            else:
                if source[index] != "\n":
                    output[index] = " "
                index += 1
        elif quote:
            if source[index] != "\n":
                output[index] = " "
            if escaped:
                escaped = False
            elif source[index] == "\\":
                escaped = True
            elif source[index] == quote:
                quote = None
            index += 1
        elif source.startswith("//", index):
            end = source.find("\n", index)
            end = len(source) if end == -1 else end
            output[index:end] = " " * (end - index)
            index = end
        elif source.startswith("/*", index):
            output[index : index + 2] = "  "
            block_depth = 1
            index += 2
        elif source[index] == '"':
            output[index] = " "
            quote = '"'
            index += 1
        else:
            index += 1

    return "".join(output)


def _git_show(revision: str, path: str) -> str | None:
    process = subprocess.run(
        ["git", "show", f"{revision}:{path}"],
        check=False,
        capture_output=True,
        text=True,
    )
    return process.stdout if process.returncode == 0 else None


def _git(*arguments: str) -> str:
    return subprocess.run(
        ["git", *arguments],
        check=True,
        capture_output=True,
        text=True,
    ).stdout


def _print_text(results: list[FileResult], base: str, head: str) -> None:
    counts = {
        category: sum(result.category == category for result in results)
        for category in ("test-only", "mixed", "non-test-only", "unclassified")
    }
    print(f"Diff: {base}...{head}")
    print(
        "Files: "
        f"{len(results)} total, {counts['test-only']} test-only, "
        f"{counts['mixed']} mixed, {counts['non-test-only']} non-test-only, "
        f"{counts['unclassified']} unclassified"
    )
    print("\nNon-test changes:")
    for result in results:
        if result.category in {"mixed", "non-test-only", "unclassified"}:
            print(
                f"{result.category:13} {result.path} "
                f"(test lines: {result.test_lines}, non-test lines: {result.non_test_lines})"
            )


if __name__ == "__main__":
    app()
