---
name: frb-fix-codecov
description: Use when a flutter_rust_bridge PR has Codecov patch or project coverage drops, Codecov comments mention missing lines, or you need to decide whether to add tests or justified coverage-ignore markers.
---

# FRB Fix Codecov

## Overview

Triage Codecov as a coverage signal, not as a generic CI failure. First identify the exact missing patch
lines from the latest Codecov report, then decide whether those lines deserve real coverage or a justified
ignore marker.

## Triage

Start from the latest PR head, not an old Codecov comment. Codecov comments may be edited and checks may
arrive after the main workflow finishes.

Useful commands:

```bash
gh pr view <number> --repo fzyzcjy/flutter_rust_bridge \
  --json headRefOid,comments,statusCheckRollup,url

gh api repos/fzyzcjy/flutter_rust_bridge/commits/<head-sha>/check-runs --paginate \
  | jq '.check_runs[]
      | select(.name | startswith("codecov/"))
      | {name, conclusion, title: .output.title, summary: .output.summary, text: .output.text, details_url}'
```

Read both Codecov statuses:

- `codecov/patch`: coverage on changed lines only. This is the usual PR blocker.
- `codecov/project`: whole-project coverage delta. A green project status does not make a red patch status
  irrelevant.

The GitHub comment usually lists only files and counts, for example `2 Missing`, not precise line numbers.
Do not stop there. Obtain exact missing patch lines by combining the PR diff with Codecov's per-file report
API.

### Exact Missing Patch Lines

Use Codecov's `file_report` endpoint for each file listed in the Codecov comment or check output:

```bash
curl -sS \
  "https://api.codecov.io/api/v2/github/fzyzcjy/repos/flutter_rust_bridge/file_report/${URLENCODED_FILE}?sha=${HEAD_SHA}"
```

The `URLENCODED_FILE` must encode `/` as `%2F`; otherwise Codecov treats the path as multiple URL segments.
In the returned JSON, `line_coverage` is a list of `[line_number, value]`. For FRB's uploaded custom Rust
coverage JSON, interpret it this way:

- `value == null`: not coverable or ignored
- `value == 0`: covered
- `value > 0`: missing

Then intersect those missing line numbers with the changed line numbers from the PR diff. This gives the
exact uncovered patch lines that Codecov is complaining about.

Concrete command sequence:

```bash
PR=3065
REPO=fzyzcjy/flutter_rust_bridge
FILE=frb_codegen/src/library/codegen/polisher/auto_upgrade.rs

BASE_SHA=$(gh pr view "$PR" --repo "$REPO" --json baseRefOid --jq .baseRefOid)
HEAD_SHA=$(gh pr view "$PR" --repo "$REPO" --json headRefOid --jq .headRefOid)
URLENCODED_FILE=$(python3 -c 'import sys, urllib.parse; print(urllib.parse.quote(sys.argv[1], safe=""))' "$FILE")

curl -sS \
  "https://api.codecov.io/api/v2/github/fzyzcjy/repos/flutter_rust_bridge/file_report/${URLENCODED_FILE}?sha=${HEAD_SHA}" \
  | python3 -c '
import json
import subprocess
import sys

file_path = sys.argv[1]
base_sha = sys.argv[2]
head_sha = sys.argv[3]
report = json.load(sys.stdin)

missing_lines = {
    line_number
    for line_number, value in report["line_coverage"]
    if value is not None and value > 0
}

diff = subprocess.check_output(
    ["git", "diff", "--unified=0", base_sha, head_sha, "--", file_path],
    text=True,
)

changed_lines = set()
current_new_line = None
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
        changed_lines.add(current_new_line)
        current_new_line += 1
    elif not line.startswith("-"):
        current_new_line += 1

for line_number in sorted(missing_lines & changed_lines):
    print(f"{file_path}:{line_number}")
' "$FILE" "$BASE_SHA" "$HEAD_SHA"
```

If the command prints nothing but Codecov still reports missing patch lines, check whether:

- The Codecov comment is stale and points to an older head SHA.
- The file path from the comment is truncated; use the full `filepath=` query parameter from the Codecov URL.
- The local git repo does not have both SHAs; fetch the PR/head and base branch, then rerun.
- Codecov's UI is showing an indirect project-coverage change rather than uncovered changed lines.

In FRB CI, coverage is uploaded only by `Misc :: Codecov` after downloading all `*-coverage` artifacts. If
upstream coverage jobs were cancelled or missing, treat the report as partial until you verify the artifact
download in that job. Use the `gh-actions-live-logs` skill when reading GitHub Actions logs.

## Understand FRB Coverage Inputs

Rust coverage is transformed before upload by
`tools/frb_internal/lib/src/utils/codecov_transformer.dart`.

Important effects:

- `frb-coverage:ignore-start` / `frb-coverage:ignore-end` remove Rust lines from the custom Codecov JSON.
- Some syntax-only Rust lines are filtered by transformer patterns.
- Dart-side coverage uses normal Dart/lcov markers such as `// coverage:ignore-start` and
  `// coverage:ignore-end`.
- `codecov.yml` ignores examples, tests, generated Dart files, and `frb_example/`; do not expect those paths
  to repair patch coverage for `frb_codegen`, `frb_rust`, or `frb_dart` changes.

## Decide the Fix

Prefer real coverage when the uncovered line represents behavior that should keep working.

Good candidates for tests:

- Pure functions, parsers, config decisions, path computations, generated text decisions, and error handling
  that can be exercised in Rust or Dart unit tests.
- Regression-prone behavior that caused the PR change.
- Logic where a small focused test can call the function directly after making the function test-visible or
  private-in-module accessible from an existing `#[cfg(test)]` module.

Use the relevant testing skills after choosing the test:

- `frb-develop-feature` for adding feature or bug regression coverage.
- `frb-test` for selecting local test commands.
- `tom-frb-env` before running FRB tests or setup commands in Tom's environment.

Ignore coverage only when the line is genuinely hard or misleading to execute and the source already makes
that reason clear. Acceptable examples seen in this repo include:

- Build-time or macro-time code that is certainly executed but invisible to llvm-cov.
- OS-specific fallback or missing-tool branches that are impractical or flaky to force in normal CI.
- Warning-only or defensive branches where testing would mostly assert implementation trivia.

When adding Rust ignores, use:

```rust
// Short reason why coverage cannot see this or why the branch is not worth executing.
// frb-coverage:ignore-start
...
// frb-coverage:ignore-end
```

When adding Dart ignores, use the native Dart coverage markers:

```dart
// coverage:ignore-start
...
// coverage:ignore-end
```

Do not add ignore markers merely because adding a test is inconvenient. If the line is product behavior and
no focused test path is obvious, summarize the missing lines and ask the human which tradeoff they want.

## Verification

After changing code or markers:

1. Re-read the touched files before editing and stage only your files.
2. Commit the logical fix before running long tests, following Tom's commit policy.
3. Run the focused local test command that covers the changed package when practical.
4. Push and check the latest `codecov/patch` and `codecov/project` statuses on the new head.
5. If Codecov is still red, repeat from the exact latest missing-line report rather than reusing stale line
   numbers.
