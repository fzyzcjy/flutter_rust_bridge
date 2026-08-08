---
name: frb-add-contributor
description: Add or reconcile flutter_rust_bridge contributors through all-contributors PRs. Use when finding contributor PRs, posting all-contributors comments, validating contributor metadata, resolving contributor PR merge conflicts, or merging the generated contributor PRs.
---

# FRB Add Contributor

Use this skill when contributors need to be added to `flutter_rust_bridge` via `all-contributors`, especially when multiple recent PR authors should be credited in batch.

## Step 1: Find the correct source PRs

For release reconciliation, start from the same authoritative merged-PR list and timestamp bounds used by `frb-write-changelog`:

```bash
gh release view <previous-release-tag> --json publishedAt
gh pr list --state merged --limit 200 --json number,title,author,mergedAt,baseRefName,url
```

- Use the previous release's `publishedAt` as the exclusive lower bound.
- Freeze the current time as the target release's inclusive upper bound. When refreshing an existing release, use that release's published timestamp instead.
- Keep every merged PR inside the bounds regardless of target branch.
- Increase the query limit or paginate until the result reaches the lower bound; do not silently truncate a busy release range.
- Exclude bots and all-contributors PRs from the human contributor audit.
- Save this exact PR list and bounds for `frb-write-changelog`; do not independently reconstruct a different release range later.

For each GitHub username found in that bounded list, find all of their PRs in `fzyzcjy/flutter_rust_bridge` when you need to identify their first source PR:

```bash
gh search prs --repo fzyzcjy/flutter_rust_bridge --author <username> --limit 100 --json number,title,author,url,state
```

Important:

- Deduplicate by contributor. A single person may have multiple merged PRs.
- If the task is about `new contributor PR`s, use the contributor's first PR only.
- For release reconciliation, enumerate every third-party human-authored PR in the release range, including docs, CI, chore, and tooling PRs.
- For each release-range PR, inspect the contributor's entry in `.all-contributorsrc`, `README.md`, and `.all-contributors-custom.yaml`.
- Presence in the contributor list is not sufficient. Confirm that the human-written `.all-contributors-custom.yaml` summary semantically covers every release-range contribution from that person.
- If an existing summary does not cover a release-range PR, treat that contributor as needing reconciliation even though they are already listed.
- If the contributor already exists in `.all-contributorsrc` or `README.md`, do not expect a new all-contributors PR to be created. Update the human-written custom summary instead.

## Step 2: Stop for human confirmation

After determining which contributors or custom descriptions need reconciliation, stop and ask a human to confirm the contributor list and source PRs before taking any mutating action.

Show the human, for each candidate:

- GitHub username.
- Every release-range source PR that is not covered.
- The current `.all-contributors-custom.yaml` summary, or `missing`.
- Whether the contributor is new or only needs their existing description extended.
- For a new contributor, the proposed deduplicated set of all-contributors contribution types covering all source PRs. Use `doc` for documentation-only work and choose the closest supported types such as `code`, `infra`, `maintenance`, `test`, or `tool` for other work; do not default every contribution to `code`.

This stop is mandatory.

- Do not edit `.all-contributors-custom.yaml`.
- Do not regenerate contributor artifacts.
- Do not post all-contributors comments.
- Do not create or merge any PRs.
- Do not perform any other GitHub mutation.

Only continue after the human explicitly confirms which contributors or descriptions to update, which source PRs those updates cover, and the contribution-type set for every new contributor.

## Step 3: Prepare `.all-contributors-custom.yaml` and stop for human input

Create a dedicated contributor-reconciliation branch or worktree from fresh `origin/master`. Keep these edits separate from the release version-bump branch.

Before regenerating contributor artifacts, add a `TODO` marker for every confirmed gap.

For a new contributor, append them to the end of `.all-contributors-custom.yaml`:

```yaml
- <username>: TODO
```

For an existing contributor who is already credited but has no `.all-contributors-custom.yaml` entry, append an entry with a marker for each uncovered PR:

```yaml
- <username>: TODO(#<pr-number>)
```

For an existing contributor whose summary does not cover one or more release-range PRs, preserve the human-written text and append a marker for each uncovered PR:

```yaml
- <username>: <existing human-written summary> TODO(#<pr-number>)
```

Do not write the missing contribution description yourself. The marker makes the required human edit explicit without discarding the existing summary.

Then stop and ask a human to replace each `TODO` or `TODO(#<pr-number>)` with a concise contribution summary that covers the named PR.

This stop is mandatory.

- Do not continue to Step 4 or any later step in this skill until the human has replaced every `TODO` marker.
- Do not run contributor generation.
- Do not post all-contributors comments.
- Do not resolve or merge contributor PRs.

Only resume after the human-written descriptions are present in `.all-contributors-custom.yaml`.

Commit the completed `.all-contributors-custom.yaml` edit on the dedicated reconciliation branch before handling bot PRs. Do not push or open the reconciliation PR until the generated artifacts are ready.

## Step 4: Trigger all-contributors

For each new contributor, post the comment on the chosen source PR:

```bash
gh pr comment <pr-number> --repo fzyzcjy/flutter_rust_bridge --body '@all-contributors please add <username> for <type1>, <type2>'
```

Use the human-confirmed deduplicated type set in one comment. Omit the comma when only one type applies.

Then inspect the bot response on that PR.

- Skip this step for an existing contributor whose only gap was an incomplete custom description.
- If the bot says the user already contributed before, stop there for that user.
- Otherwise, query open PRs from `allcontributors[bot]` and match the username in the PR title or diff. Do not rely on an exact title containing one contribution type.

## Step 5: Validate contributor metadata

When a contributor PR exists, check the generated contributor data before merging.

Files to inspect:

- `.all-contributorsrc`
- `README.md`

Required validation:

- Homepage URLs must use `https://`, never `http://`.
- If the generated homepage is `http://`, change it to `https://` if the site supports it.
- If the personal site does not support `https://`, prefer the contributor's GitHub profile URL instead of keeping `http://`.

## Step 6: Resolve and merge new-contributor metadata PRs

These PRs usually conflict only in:

- `.all-contributorsrc`
- `README.md`

For each new-contributor bot PR, validate its metadata, resolve conflicts, and merge it before regenerating custom descriptions. The generator requires every custom-summary login to exist in `.all-contributorsrc`.

Handle bot PRs in a separate clean worktree. Do not checkout a bot branch in the dedicated reconciliation worktree that contains the committed human-written custom summaries.

Recommended workflow for each open contributor PR:

```bash
gh pr checkout <pr-number> --repo fzyzcjy/flutter_rust_bridge
git fetch origin master
git merge --no-ff origin/master
```

If the merge conflicts:

1. Keep the latest `origin/master` content.
2. Re-apply the contributor entry from the bot branch.
3. Ensure the contributor card remains present in `README.md`.
4. `git add` the resolved files and commit the merge.
5. Push the contributor branch back to origin.

Do not revert unrelated contributor entries that were merged by other PRs.

After the branch is updated and GitHub reports it mergeable:

```bash
gh pr merge <pr-number> --repo fzyzcjy/flutter_rust_bridge --merge --delete-branch
```

Notes:

- GitHub may temporarily report `DIRTY` or reject merge immediately after push.
- If that happens, wait briefly, re-check the PR state, and retry the merge.
- After merging one contributor PR, fetch `origin/master` again before resolving the next one.

## Step 7: Regenerate and land custom descriptions

Use a dedicated contributor-reconciliation branch based on `origin/master` for `.all-contributors-custom.yaml` changes. Do not leave description-only reconciliation on an unpublished local branch or mix it into the release version-bump commit.

After all required bot metadata PRs are merged:

1. Return to the dedicated reconciliation worktree and branch.
2. Fetch `origin master` and merge it into the reconciliation branch with `git merge --no-ff origin/master`.
3. Preserve the human-written custom summaries and confirm that no `TODO` marker remains.
4. Re-check that every third-party release-range PR is covered by the resulting custom summaries.
5. Regenerate contributor artifacts instead of hand-editing `README.md`:

   ```bash
   ./frb_internal generate-internal-contributor
   ```

6. Commit `.all-contributorsrc`, `README.md`, and `website/docs/index.md` as applicable in a follow-up commit after the earlier custom-summary commit.
7. Push the reconciliation branch, open a PR, merge it, and fetch the updated `origin/master`.

This step also applies when every candidate was already credited and only custom descriptions changed. Do not trigger all-contributors for that case.

The generator updates:

- `.all-contributorsrc`.
- `README.md` via `all-contributors generate`.
- `website/docs/index.md` via `generate-internal-readme`.

## Step 8: Final verification

At the end, verify there are no remaining open all-contributors PRs:

```bash
gh pr list --repo fzyzcjy/flutter_rust_bridge --search "author:allcontributors[bot] is:open" --limit 50 --json number,title,mergeStateStatus,url
```

Also verify the credited contributors appear in:

- `.all-contributorsrc`
- `README.md`
- `website/docs/index.md` if contributor regeneration was run
- `.all-contributors-custom.yaml` with human-written summaries covering every third-party release-range PR and no remaining `TODO` markers.

Confirm the merged reconciliation commit is present in fresh `origin/master` before returning to changelog preparation or release publishing.
