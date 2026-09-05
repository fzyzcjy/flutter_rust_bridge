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
- When refreshing an already-published release, use both the previous release timestamp and the target release tag timestamp so later post-release PRs are not pulled into the published section.
- Do not filter by target branch.
- Keep docs, CI, and chore PRs if they are merged in the range.
- Exclude only all-contributors PRs such as `docs: add <name> as a contributor for code/doc`; do not exclude ordinary documentation PRs whose titles start with `docs: add`.

Normalize titles before writing.

- Rewrite noisy internal titles into concise changelog language.
- Avoid duplicate summaries for split or "continued" PRs.
- Preserve repo-specific capitalization such as `CI`, `GitHub`, `Flutter`, `Rust`, `DCO`, and `V1`.

## Step 5: Write the section

Match the existing changelog style.

- Write each item as `* Summary #1234`.
- Append `(thanks @username)` for every third-party human-authored PR in the release range, including docs, CI, chore, tooling, and feature PRs.
- When a changelog entry combines local maintainer PRs with a third-party PR, append the third-party thanks to that combined entry.
- If multiple third-party authors appear in one combined entry, include each author in the same entry.
- For an ordinary beta release section, place entries with `(thanks @username)` before entries without thanks. Within each group, keep merge order from newest to oldest unless the surrounding section clearly uses another order.

For a stable release that follows betas of the same version series, use a grouped stable section:

- Put only changes merged after the latest beta release first as ordinary top-level entries. Do not include an older change solely because the beta changelogs omitted it.
- Then add one top-level parent entry per beta, using the canonical version label such as `* 2.13.0-beta.6`, ordered from the latest beta to the earliest beta.
- Copy that beta's substantive entries exactly beneath its parent as nested bullets. Preserve wording, PR references, and thanks attribution.
- Keep the generic "what's new" entry only once at the top of the stable section.
- Never fold a post-latest-beta PR into a copied beta entry. Keep it in the leading group even when its subject is related to a beta entry.
- Apply thanks-first ordering independently within each copied beta group. In the post-latest-beta group, user-visible-before-internal ordering takes precedence over thanks placement.

For newly written entries, keep every user-visible feature, fix, behavior change, published-package change, and user-facing documentation PR as its own entry. Do not join such changes with `and` or fold multiple PRs into a shared summary. Combining multiple PRs is allowed only for user-invisible internal work such as CI, tests, agent or release tooling, and code outside published packages. Order the user-visible entries first, then put all user-invisible internal entries at the end of the post-latest-beta group immediately before the first beta parent entry. Copied beta entries remain exact copies even if they predate this rule.

Edit only `CHANGELOG.md`. Do not manually edit generated files for this task.

## Step 6: Review the draft

Review the final diff.

- Confirm the change is limited to the target release section.
- Confirm wording and ordering match nearby release sections.
- Do not commit yet. Human review can still change the text, and the package changelog copies must be regenerated from the final root changelog.

## Step 7: Run mechanical verification

Run the changelog verifier after finishing the draft.

```bash
gh pr list --state merged --limit 200 --json number,title,author,mergedAt,baseRefName,url > /tmp/frb-merged-prs.json
uv run --script .claude/skills/frb-write-changelog/verify_changelog.py \
  --version <VERSION> \
  --previous-release-time <PREVIOUS_RELEASE_TIMESTAMP> \
  --release-time <TARGET_RELEASE_TIMESTAMP> \
  --merged-prs-json /tmp/frb-merged-prs.json
```

The verifier checks that:

- PR numbers in the target section are complete, not duplicated, and not unexpected.
- Third-party thanks authors are complete and not unexpected, including ordinary documentation PR authors. The same author may be thanked on multiple entries.
- Entries with third-party thanks appear before entries without thanks within each copied beta group.
- `docs: add <name> as a contributor ...` all-contributors PRs are ignored.

Use `--ignore-pr <NUMBER>` only for a documented intentional exclusion. Use `--extra-local-pr <NUMBER>` for a stacked local maintainer PR that belongs in the changelog but is not present in the merged PR JSON yet. Use `--extra-thanks-author <LOGIN>` only for a verified co-author or contributor credited by a release-range PR even though their source PR falls outside the release range; record the source PR and reason in the release journal.

Apply any confirmed fixes, then re-run the verifier.

## Step 8: Ask the user to review ordering

Tell the user the changelog draft is complete and ask for a manual review.

- Ask the user to review the wording.
- Ask the user to review the ordering of entries.
- Adjust the ordering if the user wants a different presentation from the mechanically collected order.

## Step 9: Re-verify after human edits

Run the mechanical verifier again after the user finishes manual edits.

- Confirm there are no missing, duplicated, or extra PR numbers.
- Confirm there are no missing or unexpected third-party thanks authors.
- Confirm entries with third-party thanks appear before entries without thanks within each copied beta group.
- Apply any confirmed fixes, then do one final diff check.

If the user explicitly wants an independent review, ask a separate reviewer or subagent to compare the final `CHANGELOG.md` against the same merged PR list.

## Step 10: Regenerate published-package changelogs

Regenerate the published-package changelog copies from the final root `CHANGELOG.md`:

```bash
./frb_internal generate-internal-readme
```

- Confirm `frb_dart/CHANGELOG.md` and `frb_hooks/CHANGELOG.md` match the root `CHANGELOG.md`.
- Commit the root changelog and both generated copies together as one atomic release-preparation change.
- Do not manually edit either generated package changelog.
- From the clean post-commit tree, run `./frb_internal generate-internal-readme --set-exit-if-changed` as the final drift gate. Do not run this flag while the changelog draft is still dirty because it checks the entire worktree.
