---
name: frb-write-changelog
description: Update flutter_rust_bridge CHANGELOG.md for a new release by collecting merged PRs since the previous version tag, mapping them to the target release section, and formatting entries to match the existing changelog style. Use when writing or refreshing a release section in CHANGELOG.md.
---

# FRB Write Changelog

## Step 1: Inspect `CHANGELOG.md`

Read the top of `CHANGELOG.md`.

- Confirm the target version section exists, or add it.
- Keep the existing V2 "what's new" header line.
- Inspect `git diff -- CHANGELOG.md` before editing.
- Replace only the target section if it already contains a placeholder such as `* TODO`.

## Step 2: Identify the previous release tag

Find the latest release tag before the target version.

```bash
git tag --sort=-creatordate | head -n 20
```

Use that tag as the lower bound for the new changelog entry.

## Step 3: Collect merged PRs

Use Git history as the source of truth.

```bash
git log --merges --first-parent --format='%s' vX.Y.Z..HEAD
```

Resolve PR metadata with GitHub CLI.

```bash
gh pr list --state merged --limit 200 --json number,title,author,mergedAt,baseRefName,url
gh pr view <number> --json number,title,author,url
```

Do not use `gh pr list` alone to define the release range.

## Step 4: Filter and normalize

Keep only PRs that were merged into `master` in the release range.

- Exclude unmerged PRs.
- Exclude PRs outside the release range.
- Exclude branch-only PRs that did not land on `master`.
- Keep docs, CI, and chore PRs if they landed in the range.

Normalize titles before writing.

- Rewrite noisy internal titles into concise changelog language.
- Avoid duplicate summaries for split or "continued" PRs.
- Preserve repo-specific capitalization such as `CI`, `GitHub`, `Flutter`, `Rust`, `DCO`, and `V1`.

## Step 5: Write the section

Match the existing changelog style.

- Write each item as `* Summary #1234`.
- Append `(thanks @username)` when attribution is appropriate.
- Keep items in merge order from newest to oldest unless the surrounding section clearly uses another order.

Edit only `CHANGELOG.md`. Do not manually edit generated files for this task.

## Step 6: Review and commit

Review the final diff.

- Confirm the change is limited to the target release section.
- Confirm wording and ordering match nearby release sections.
- Create a small atomic commit after finishing the edit.
