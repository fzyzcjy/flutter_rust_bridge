---
name: frb-write-changelog
description: Use when updating flutter_rust_bridge CHANGELOG.md for a new release by collecting merged PRs since the previous version and formatting them to match existing entries
---

# FRB Write Changelog

## Overview

Use this skill when filling a new release section in `CHANGELOG.md`.

**Core principle:** Derive the changelog from the PRs merged into `master` since the previous release tag, then rewrite them to match the existing changelog style.

## Workflow

```
1. Read the top of CHANGELOG.md
   |
   +-- Confirm the target version section already exists, or add it
   |
   +-- 2. Identify the previous release tag (usually the latest existing vX.Y.Z tag before the target version)
   |
   +-- 3. Collect merged PRs in the release range
   |      Prefer: git log --merges --first-parent <previous-tag>..HEAD
   |      Then enrich with: gh pr list / gh pr view
   |
   +-- 4. Filter and normalize
   |      Keep PRs merged into master in the release range
   |      Avoid duplicate "continued" PR summaries
   |      Keep docs/CI/chore PRs if they were actually merged in the range
   |
   +-- 5. Write bullets in existing CHANGELOG style
   |
   +-- 6. Review diff and commit
```

## Recommended Commands

Find the previous release tag:

```bash
git tag --sort=-creatordate | head -n 20
```

Get PR numbers actually merged into `master` after the previous release:

```bash
git log --merges --first-parent --format='%s' vX.Y.Z..HEAD
```

Then resolve titles/authors with GitHub CLI:

```bash
gh pr list --state merged --limit 200 --json number,title,author,mergedAt,baseRefName,url
gh pr view <number> --json number,title,author,url
```

## Writing Rules

- Keep the existing header line that links to the V2 "what's new" page.
- Follow the existing bullet style: `* Summary #1234` and optionally append `(thanks @username)`.
- Use concise, user-facing wording instead of raw PR titles when the title is noisy or overly internal.
- Preserve useful capitalization already used in the repo, such as `CI`, `GitHub`, `Flutter`, `Rust`, `DCO`, and `V1`.
- Keep the bullets in merge order from newest to oldest unless the existing surrounding section clearly uses another order.
- Do not include unmerged PRs, branch-only PRs, or PRs outside the release range.
- Avoid manually editing generated files; `CHANGELOG.md` is safe to edit manually.

## Practical Notes

- `git log --merges --first-parent` is the source of truth for what actually landed on `master`.
- `gh pr list` is useful for titles and author attribution, but do not trust it alone for release boundaries.
- If `CHANGELOG.md` already contains a placeholder like `* TODO`, replace only that target section.
- Before editing, inspect `git diff -- CHANGELOG.md` so you do not overwrite user changes already in progress.
- After finishing the text update, review the diff and create a small atomic commit.
