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

Use GitHub CLI to collect merged PRs after the previous version.

```bash
gh pr list --state merged --limit 200 --json number,title,author,mergedAt,baseRefName,url
gh pr view <number> --json number,title,author,url
```

Use the previous version as the lower bound when deciding which merged PRs belong to the target release.

## Step 4: Filter and normalize

Keep all PRs in merged status that belong to the release range.

- Exclude unmerged PRs.
- Exclude PRs outside the release range.
- Do not filter by target branch.
- Keep docs, CI, and chore PRs if they are merged in the range.
- Exclude all-contributors PRs such as `docs: add <name> as a contributor for code/doc`.

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

## Step 7: Run mechanical verification

Run the changelog verifier after finishing the draft.

```bash
gh pr list --state merged --limit 200 --json number,title,author,mergedAt,baseRefName,url > /tmp/frb-merged-prs.json
uv run --script .claude/skills/frb-write-changelog/verify_changelog.py \
  --version <VERSION> \
  --previous-release-time <PREVIOUS_RELEASE_TIMESTAMP> \
  --merged-prs-json /tmp/frb-merged-prs.json
```

The verifier checks that:

- PR numbers in the target section are complete, not duplicated, and not unexpected.
- Third-party thanks authors are complete, not duplicated, and not unexpected.
- `docs: add <name> as a contributor ...` all-contributors PRs are ignored.

Use `--ignore-pr <NUMBER>` only for a documented intentional exclusion. Use `--extra-local-pr <NUMBER>` for a stacked local maintainer PR that belongs in the changelog but is not present in the merged PR JSON yet.

Apply any confirmed fixes, then re-run the verifier.

## Step 8: Ask the user to review ordering

Tell the user the changelog draft is complete and ask for a manual review.

- Ask the user to review the wording.
- Ask the user to review the ordering of entries.
- Adjust the ordering if the user wants a different presentation from the mechanically collected order.

## Step 9: Re-verify after human edits

Run the mechanical verifier again after the user finishes manual edits.

- Confirm there are no missing, duplicated, or extra PR numbers.
- Confirm there are no missing, duplicated, or extra third-party thanks authors.
- Apply any confirmed fixes, then do one final diff check.

If the user explicitly wants an independent review, ask a separate reviewer or subagent to compare the final `CHANGELOG.md` against the same merged PR list.
