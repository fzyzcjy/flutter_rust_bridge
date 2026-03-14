---
name: frb-write-changelog
description: Use when updating flutter_rust_bridge CHANGELOG.md for a new release by collecting merged PRs since the previous version and formatting them to match existing entries
---

# FRB Write Changelog

## Overview

Use this skill when filling a new release section in `CHANGELOG.md`.

**Core principle:** Derive the changelog from the PRs merged into `master` since the previous release tag, then rewrite them to match the existing changelog style.

## Step 1: Inspect `CHANGELOG.md`

Read the top of `CHANGELOG.md` and confirm the target version section already exists. If the file already contains a placeholder such as `* TODO`, replace only that target section.

Before editing, inspect the current diff so you do not overwrite user changes already in progress:

```bash
git diff -- CHANGELOG.md
```

Keep the existing header line that links to the V2 "what's new" page.

## Step 2: Identify the previous release tag

Find the latest existing release tag before the target version. In most cases this is the boundary for the new changelog entry.

```bash
git tag --sort=-creatordate | head -n 20
```

## Step 3: Collect PRs in the release range

Use Git history as the source of truth for what actually landed on `master`.

```bash
git log --merges --first-parent --format='%s' vX.Y.Z..HEAD
```

This gives the PR numbers that were merged after the previous release.

Then use GitHub CLI to resolve titles and authors:

```bash
gh pr list --state merged --limit 200 --json number,title,author,mergedAt,baseRefName,url
gh pr view <number> --json number,title,author,url
```

Do not rely on `gh pr list` alone for the release boundary. It is useful for metadata, not for deciding what shipped.

## Step 4: Filter and normalize the PR list

Keep only PRs that satisfy all of the following:

- Do not include unmerged PRs, branch-only PRs, or PRs outside the release range.
- `git log --merges --first-parent` is the source of truth for what actually landed on `master`.
- Keep docs/CI/chore PRs if they were actually merged in the range.

When normalizing titles:

- Avoid duplicate summaries for split or "continued" PRs.
- Rewrite noisy or overly internal PR titles into concise user-facing changelog entries.
- Preserve useful capitalization already used in the repo, such as `CI`, `GitHub`, `Flutter`, `Rust`, `DCO`, and `V1`.

## Step 5: Write the changelog bullets

Follow the existing changelog style:

- Use `* Summary #1234`.
- Optionally append `(thanks @username)` for contributor attribution.
- Keep bullets in merge order from newest to oldest unless the surrounding changelog section clearly uses another order.

`CHANGELOG.md` is safe to edit manually. Do not manually edit generated files in the repo for this task.

## Step 6: Review and commit

After writing the section:

- Review the diff carefully.
- Make sure the edit is limited to the target release section.
- Create a small atomic commit.
