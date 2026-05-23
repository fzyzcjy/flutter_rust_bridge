from __future__ import annotations

import json
import logging
import shlex
import subprocess
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Annotated, Any, Optional

import typer

logger = logging.getLogger(__name__)

app = typer.Typer(no_args_is_help=True)


@dataclass(frozen=True)
class PullRequestInfo:
    __slots__ = ("base_sha", "head_sha", "raw")

    base_sha: str
    head_sha: str
    raw: dict[str, Any]


@dataclass(frozen=True)
class MissingPatchLine:
    __slots__ = ("file_path", "line_number")

    file_path: str
    line_number: int


@app.callback()
def main() -> None:
    """Analyze flutter_rust_bridge Codecov reports."""


@app.command()
def download(
    pr: Annotated[
        Optional[int],
        typer.Option("--pr", help="Pull request number to analyze."),
    ] = None,
    repo: Annotated[
        str,
        typer.Option("--repo", help="GitHub repository in owner/name form."),
    ] = "fzyzcjy/flutter_rust_bridge",
    output_dir: Annotated[
        Optional[Path],
        typer.Option(
            "--output-dir",
            help="Directory where raw and summarized report files are written.",
        ),
    ] = None,
) -> None:
    """Download Codecov PR data into a local directory."""

    logging.basicConfig(level=logging.INFO)

    if pr is None:
        raise typer.BadParameter("--pr is required")

    resolved_output_dir = output_dir or _default_output_dir(pr)
    resolved_output_dir.mkdir(parents=True, exist_ok=True)

    pr_info = _fetch_pr(repo=repo, pr=pr, output_dir=resolved_output_dir)
    check_runs = _fetch_check_runs(
        repo=repo,
        head_sha=pr_info.head_sha,
        output_dir=resolved_output_dir,
    )
    codecov_report = _fetch_codecov_report(
        repo=repo,
        head_sha=pr_info.head_sha,
        output_dir=resolved_output_dir,
    )

    _write_human_summaries(
        pr_info=pr_info,
        check_runs=check_runs,
        codecov_report=codecov_report,
        output_dir=resolved_output_dir,
    )

    typer.echo(f"output_dir={resolved_output_dir}")
    typer.echo(f"base_sha={pr_info.base_sha}")
    typer.echo(f"head_sha={pr_info.head_sha}")
    typer.echo(
        "next_command="
        f"python3 .claude/skills/frb-fix-codecov/codecov_analyzer.py analyze "
        f"--input-dir {shlex.quote(str(resolved_output_dir))}"
    )


@app.command()
def analyze(
    input_dir: Annotated[
        Optional[Path],
        typer.Option("--input-dir", help="Directory created by the download command."),
    ] = None,
) -> None:
    """Print exact missing patch lines from a downloaded Codecov report."""

    logging.basicConfig(level=logging.INFO)

    if input_dir is None:
        raise typer.BadParameter("--input-dir is required")

    pr_info = _load_pr_info(input_dir / "pr.json")
    codecov_report: dict[str, Any] = json.loads((input_dir / "codecov-report.json").read_text())

    missing_patch_lines = _compute_missing_patch_lines(
        base_sha=pr_info.base_sha,
        head_sha=pr_info.head_sha,
        codecov_report=codecov_report,
    )
    _write_missing_patch_lines(
        output_dir=input_dir,
        missing_patch_lines=missing_patch_lines,
    )

    typer.echo(f"input_dir={input_dir}")
    typer.echo(f"base_sha={pr_info.base_sha}")
    typer.echo(f"head_sha={pr_info.head_sha}")
    typer.echo("missing_patch_lines:")
    for missing_line in missing_patch_lines:
        typer.echo(f"{missing_line.file_path}:{missing_line.line_number}")


def exec_command(cmd: list[str], *, capture_output: bool = True) -> Optional[str]:
    print(f"EXEC: {shlex.join(cmd)}", flush=True)

    try:
        result = subprocess.run(
            cmd,
            check=True,
            capture_output=capture_output,
            text=True,
        )
    except subprocess.CalledProcessError as error:
        if capture_output:
            print(f"FAILED: stdout={error.stdout} stderr={error.stderr}", flush=True)
        raise

    if capture_output:
        return result.stdout

    return None


def _write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def _load_json_objects(raw: str) -> list[Any]:
    decoder = json.JSONDecoder()
    result: list[Any] = []
    position = 0

    while position < len(raw):
        while position < len(raw) and raw[position].isspace():
            position += 1

        if position >= len(raw):
            break

        value, position = decoder.raw_decode(raw, position)
        result.append(value)

    return result


def _merge_check_run_pages(pages: list[dict[str, Any]]) -> dict[str, Any]:
    check_runs = [
        check_run
        for page in pages
        for check_run in page["check_runs"]
    ]

    return {
        "total_count": len(check_runs),
        "check_runs": check_runs,
    }


def _load_pr_info(path: Path) -> PullRequestInfo:
    raw: dict[str, Any] = json.loads(path.read_text())
    return PullRequestInfo(
        base_sha=raw["baseRefOid"],
        head_sha=raw["headRefOid"],
        raw=raw,
    )


def _fetch_pr(repo: str, pr: int, output_dir: Path) -> PullRequestInfo:
    raw = exec_command(
        [
            "gh",
            "pr",
            "view",
            str(pr),
            "--repo",
            repo,
            "--json",
            "baseRefOid,headRefOid,comments,statusCheckRollup,url",
        ],
    )
    if raw is None:
        raise RuntimeError("gh pr view unexpectedly produced no output")

    pr_path = output_dir / "pr.json"
    pr_path.write_text(raw)
    return _load_pr_info(pr_path)


def _fetch_check_runs(repo: str, head_sha: str, output_dir: Path) -> dict[str, Any]:
    raw = exec_command(
        [
            "gh",
            "api",
            f"repos/{repo}/commits/{head_sha}/check-runs",
            "--paginate",
        ],
    )
    if raw is None:
        raise RuntimeError("gh api unexpectedly produced no output")

    pages = _load_json_objects(raw)
    check_runs = _merge_check_run_pages(pages)
    _write_json(output_dir / "check-runs.json", check_runs)
    return check_runs


def _fetch_codecov_report(repo: str, head_sha: str, output_dir: Path) -> dict[str, Any]:
    owner, repo_name = repo.split("/", 1)
    url = f"https://api.codecov.io/api/v2/github/{owner}/repos/{repo_name}/report?sha={head_sha}"

    raw = exec_command(["curl", "-sS", url])
    if raw is None:
        raise RuntimeError("curl unexpectedly produced no output")

    report: dict[str, Any] = json.loads(raw)
    _write_json(output_dir / "codecov-report.json", report)
    return report


def _write_human_summaries(
    *,
    pr_info: PullRequestInfo,
    check_runs: dict[str, Any],
    codecov_report: dict[str, Any],
    output_dir: Path,
) -> None:
    codecov_comments = [
        comment["body"]
        for comment in pr_info.raw["comments"]
        if comment["author"]["login"] == "codecov"
    ]
    (output_dir / "codecov-comment.md").write_text("\n\n---\n\n".join(codecov_comments))

    codecov_checks = [
        {
            "details_url": check_run["details_url"],
            "name": check_run["name"],
            "conclusion": check_run["conclusion"],
            "summary": check_run["output"]["summary"],
            "title": check_run["output"]["title"],
        }
        for check_run in check_runs["check_runs"]
        if check_run["name"].startswith("codecov/")
    ]
    _write_json(output_dir / "codecov-checks.json", codecov_checks)

    files_summary = {
        "totals": codecov_report["totals"],
        "files": [
            {
                "name": file_entry["name"],
                "totals": file_entry["totals"],
            }
            for file_entry in codecov_report["files"]
        ],
    }
    _write_json(output_dir / "codecov-files-summary.json", files_summary)


def _changed_new_lines(*, base_sha: str, head_sha: str, file_path: str) -> set[int]:
    raw = exec_command(
        ["git", "diff", "--unified=0", base_sha, head_sha, "--", file_path],
    )
    if raw is None:
        raise RuntimeError("git diff unexpectedly produced no output")

    result: set[int] = set()
    current_new_line: Optional[int] = None

    for line in raw.splitlines():
        if line.startswith("@@"):
            new_range = line.split(" +", 1)[1].split(" ", 1)[0]
            start_text, _, count_text = new_range.partition(",")
            current_new_line = int(start_text)
            if count_text == "0":
                current_new_line = None
            continue

        if current_new_line is None:
            continue

        if line.startswith("+") and not line.startswith("+++"):
            result.add(current_new_line)
            current_new_line += 1
        elif not line.startswith("-"):
            current_new_line += 1

    return result


def _compute_missing_patch_lines(
    *,
    base_sha: str,
    head_sha: str,
    codecov_report: dict[str, Any],
) -> list[MissingPatchLine]:
    result: list[MissingPatchLine] = []

    for file_entry in codecov_report["files"]:
        file_path = file_entry["name"]
        missing_lines = {
            line_number
            for line_number, value in file_entry["line_coverage"]
            if value is not None and value > 0
        }
        if not missing_lines:
            continue

        patch_missing_lines = sorted(
            missing_lines
            & _changed_new_lines(
                base_sha=base_sha,
                head_sha=head_sha,
                file_path=file_path,
            )
        )

        result.extend(
            MissingPatchLine(file_path=file_path, line_number=line_number)
            for line_number in patch_missing_lines
        )

    return result


def _write_missing_patch_lines(
    output_dir: Path,
    missing_patch_lines: list[MissingPatchLine],
) -> None:
    text = "".join(
        f"{missing_line.file_path}:{missing_line.line_number}\n"
        for missing_line in missing_patch_lines
    )
    (output_dir / "missing-patch-lines.txt").write_text(text)


def _default_output_dir(pr: int) -> Path:
    timestamp = datetime.now().strftime("%Y%m%d%H%M%S")
    return Path(f"/private/tmp/frb-codecov-{pr}-{timestamp}")


if __name__ == "__main__":
    app()
