---
name: frb-fix-codecov
description: Use when a flutter_rust_bridge PR has Codecov patch or project coverage drops, Codecov comments mention missing lines, or you need to decide whether to add tests or justified coverage-ignore markers.
---

# FRB Fix Codecov

## Overview

Use this skill to turn a vague Codecov PR comment into concrete file/line facts, then decide whether the
right fix is real coverage or a narrow, justified ignore marker.

## Workflow

### First, Understand the Concrete Codecov Report

Start from the latest PR head. Codecov comments can be edited and old comments/checks can describe stale
commits.

Fetch the PR metadata, Codecov comments/checks, and the full Codecov report into a local temporary directory:

```bash
PR=<number>
REPO=fzyzcjy/flutter_rust_bridge
WORKDIR=/private/tmp/frb-codecov-${PR}-$(date +%Y%m%d%H%M%S)
mkdir -p "$WORKDIR"

gh pr view "$PR" --repo "$REPO" \
  --json baseRefOid,headRefOid,comments,statusCheckRollup,url \
  > "$WORKDIR/pr.json"

BASE_SHA=$(jq -r .baseRefOid "$WORKDIR/pr.json")
HEAD_SHA=$(jq -r .headRefOid "$WORKDIR/pr.json")

gh api "repos/${REPO}/commits/${HEAD_SHA}/check-runs" --paginate \
  > "$WORKDIR/check-runs.json"

curl -sS \
  "https://api.codecov.io/api/v2/github/fzyzcjy/repos/flutter_rust_bridge/report?sha=${HEAD_SHA}" \
  > "$WORKDIR/codecov-report.json"
```

The full `report?sha=` endpoint is preferred because it returns all files in one response. Codecov also has
`file_report/<urlencoded-path>?sha=<sha>` for one file; use it only when the full report is too large or you
need to re-check a path precisely.

Save an initial human-readable summary:

```bash
jq -r '
  .comments[]
  | select(.author.login == "codecov")
  | .body
' "$WORKDIR/pr.json" > "$WORKDIR/codecov-comment.md"

jq '
  .check_runs[]
  | select(.name | startswith("codecov/"))
  | {name, conclusion, title: .output.title, summary: .output.summary, text: .output.text, details_url}
' "$WORKDIR/check-runs.json" > "$WORKDIR/codecov-checks.json"

jq '{totals, files: [.files[] | {name, totals}]}' \
  "$WORKDIR/codecov-report.json" > "$WORKDIR/codecov-files-summary.json"
```

Read both Codecov statuses:

- `codecov/patch`: coverage on changed lines only. This is the usual PR blocker.
- `codecov/project`: whole-project coverage delta. A green project status does not make a red patch status
  irrelevant.

The GitHub comment usually lists files and missing counts, not precise line numbers. Compute exact missing
patch lines by intersecting Codecov's missing lines with Git's added/changed lines. In Codecov's
`line_coverage`, FRB's uploaded custom Rust coverage JSON uses:

- `null`: not coverable or ignored
- `0`: covered
- positive number: missing

Use the full report already saved in `$WORKDIR`, and save the exact missing patch lines:

```bash
python3 - "$WORKDIR/codecov-report.json" "$BASE_SHA" "$HEAD_SHA" <<'PY' | tee "$WORKDIR/missing-patch-lines.txt"
import json
import subprocess
import sys

report_path, base_sha, head_sha = sys.argv[1:4]
report = json.load(open(report_path))


def changed_new_lines(file_path: str) -> set[int]:
    diff = subprocess.check_output(
        ["git", "diff", "--unified=0", base_sha, head_sha, "--", file_path],
        text=True,
    )

    ans: set[int] = set()
    current_new_line: int | None = None
    for line in diff.splitlines():
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
            ans.add(current_new_line)
            current_new_line += 1
        elif not line.startswith("-"):
            current_new_line += 1

    return ans


for file_entry in report["files"]:
    file_path = file_entry["name"]
    missing_lines = {
        line_number
        for line_number, value in file_entry["line_coverage"]
        if value is not None and value > 0
    }
    patch_missing_lines = sorted(missing_lines & changed_new_lines(file_path))
    for line_number in patch_missing_lines:
        print(f"{file_path}:{line_number}")
PY
```

If the computed output disagrees with Codecov:

- Check whether the Codecov comment/check describes a different `HEAD_SHA`.
- Check whether Git has both SHAs locally; fetch the PR head/base if needed.
- For a truncated file path in the comment, get the full path from the Codecov URL's `filepath=` query.
- Verify `Misc :: Codecov` downloaded all expected `*-coverage` artifacts before upload; use the
  `gh-actions-live-logs` skill for GitHub Actions logs.

### Then Fix According to the Situation

#### If the Branch Does Not Care About Codecov

In this repo the Rust-side marker is `frb-coverage:ignore-start` /
`frb-coverage:ignore-end` (not `frb-codecov-ignore`). It is implemented by
`tools/frb_internal/lib/src/utils/codecov_transformer.dart`, which removes those Rust lines before upload.

Dart-side code uses normal Dart coverage markers:

```dart
// coverage:ignore-start
...
// coverage:ignore-end
```

Only use ignore markers when the code genuinely should not be judged by Codecov. Existing FRB examples
usually have an explanatory comment immediately above the marker. Acceptable reasons include:

- The code is definitely executed, but the coverage tool cannot see it, especially build-time or macro-time
  execution.
- llvm-cov reports a branch as uncovered even though a focused test exercises it.
- The branch is warning-only, defensive, tool-missing, OS-specific, or otherwise not worth forcing in normal
  CI.
- The line is generated glue, FFI shape glue, or syntactic boilerplate where testing it would assert
  implementation trivia rather than behavior.

When adding Rust ignores, keep the ignored range as small as possible and write the reason first:

```rust
// This is executed at build time, but llvm-cov does not observe that path.
// frb-coverage:ignore-start
...
// frb-coverage:ignore-end
```

Do not use ignore markers merely because adding a test is inconvenient. If the reason cannot be explained in
one short public comment, treat the line as coverage-relevant.

#### If the Branch Cares About Codecov

Investigate why each exact missing line is not covered.

Prefer real coverage when the uncovered line represents product behavior that should keep working. Good
candidates for tests:

- Pure functions, parsers, config decisions, path computations, generated text decisions, and error handling.
- Integration/codegen behavior where the missing line affects generated files or public workflow behavior.
- Regression-prone behavior introduced or modified by the PR.

Choose the smallest meaningful test first:

- Unit test the function directly when the behavior is local and stable.
- Add a codegen or integration test when only the end-to-end generated output proves the behavior.
- Add an E2E test when the missing line is only meaningful through a real FRB workflow.

Use the relevant skills after choosing the test path:

- `frb-develop-feature` for feature/bug regression coverage.
- `frb-test` for selecting local test commands.
- `tom-frb-env` before running FRB tests or setup commands in Tom's environment.

If the behavior matters but a focused test would be brittle, very expensive, or require a product decision,
summarize the exact missing lines, the suspected reason they are uncovered, and the realistic options, then
ask the human which tradeoff to take.

## Verification

After changing tests or ignore markers:

1. Re-read the touched files immediately before editing.
2. Commit the logical fix before long verification, following Tom's commit policy.
3. Run the focused local test command when practical.
4. Push and check the latest `codecov/patch` and `codecov/project` statuses on the new head.
5. If Codecov is still red, repeat the workflow from the latest exact report instead of reusing stale line
   numbers.
