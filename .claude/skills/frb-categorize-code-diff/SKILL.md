---
name: frb-categorize-code-diff
description: Use when classifying a flutter_rust_bridge branch or PR diff into test and non-test changes, including colocated Rust test modules.
---

# 1 When to use

- Use this skill to answer which changes in a branch or PR are tests and which are not.
- Use the repository script instead of classifying paths by hand. Rust tests are commonly colocated in production `.rs` files.

# 2 Run

- Fetch the PR base before relying on `origin/master`.
- Run from the repository root:

```bash
uv run --no-project .claude/skills/frb-categorize-code-diff/categorize_code_diff.py --base origin/master --head HEAD
```

- Add `--json` when another tool will consume the result.
- Pass the PR's actual base revision when it is not `origin/master`.

# 3 Interpret

- `test-only`: every changed line is in a dedicated test path or a recognized colocated Rust test region.
- `non-test-only`: every changed line is outside recognized test code.
- `mixed`: the file contains both test and non-test changed lines.
- `unclassified`: the diff cannot be mapped confidently, such as a binary patch.
- Treat the non-test list as conservative. Unknown code is non-test rather than silently counted as test.

# 4 Reliability

- The script compares `base...head`, matching GitHub pull request diff semantics.
- Dedicated test paths include `test/`, `tests/`, `test_fixtures/`, and conventional test filenames.
- Rust inline classification recognizes brace-delimited items preceded by `#[cfg(test)]` or `#[test]` in both the base and head revisions.
- Re-run after every PR update. Line identities and rename targets may change.
- Inspect `mixed` files manually before claiming that a PR contains no production changes.

# 5 Validation

- Run the script tests after changing its classification logic:

```bash
PYTHONDONTWRITEBYTECODE=1 uv run --no-project --with pytest pytest -q .claude/skills/frb-categorize-code-diff/test_categorize_code_diff.py
```

- Keep `PYTHONDONTWRITEBYTECODE=1`; otherwise pytest leaves an untracked `__pycache__/` inside the skill directory.
