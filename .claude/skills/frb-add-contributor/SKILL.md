---
name: frb-add-contributor
description: Add or reconcile flutter_rust_bridge contributors through all-contributors PRs. Use when finding contributor PRs, posting all-contributors comments, validating contributor metadata, resolving contributor PR merge conflicts, or merging the generated contributor PRs.
---

# FRB Add Contributor

Use this skill when contributors need to be added to `flutter_rust_bridge` via `all-contributors`, especially when multiple recent PR authors should be credited in batch.

## Step 1: Find the correct source PRs

For each GitHub username, find their PRs in `fzyzcjy/flutter_rust_bridge`.

```bash
gh search prs --repo fzyzcjy/flutter_rust_bridge --author <username> --limit 100 --json number,title,author,url,state
```

Important:

- Deduplicate by contributor. A single person may have multiple merged PRs.
- If the task is about `new contributor PR`s, use the contributor's first PR only.
- If the contributor already exists in `.all-contributorsrc` or `README.md`, do not expect a new all-contributors PR to be created.

## Step 2: Prepare `.all-contributors-custom.yaml` and stop for human input

Before regenerating contributor artifacts, append each new contributor to the end of `.all-contributors-custom.yaml`:

```yaml
- <username>: TODO
```

Do this only for contributors who are not already present in `.all-contributors-custom.yaml`.

Then stop and ask a human to replace each `TODO` with a concise contribution summary.

This stop is mandatory.

- Do not continue to Step 3 or any later step in this skill until the human has replaced every `TODO`.
- Do not run contributor generation.
- Do not post all-contributors comments.
- Do not resolve or merge contributor PRs.

Only resume after the human-written descriptions are present in `.all-contributors-custom.yaml`.

## Step 3: Trigger all-contributors

Post the comment on the chosen source PR:

```bash
gh pr comment <pr-number> --repo fzyzcjy/flutter_rust_bridge --body '@all-contributors please add <username> for code'
```

Then inspect the bot response on that PR.

- If the bot says the user already contributed before, stop there for that user.
- Otherwise, wait for the bot to open a PR named `docs: add <username> as a contributor for code`.

## Step 4: Validate contributor metadata

When a contributor PR exists, check the generated contributor data before merging.

Files to inspect:

- `.all-contributorsrc`
- `README.md`

Required validation:

- Homepage URLs must use `https://`, never `http://`.
- If the generated homepage is `http://`, change it to `https://` if the site supports it.
- If the personal site does not support `https://`, prefer the contributor's GitHub profile URL instead of keeping `http://`.

## Step 5: Regenerate contributor output

After the human has replaced all `TODO` messages, regenerate the contributor artifacts instead of hand-editing `README.md`.

Use:

```bash
./frb_internal generate-internal-contributor
```

This command updates:

- `.all-contributorsrc`
- `README.md` via `all-contributors generate`
- `website/docs/index.md` via `generate-internal-readme`

If you only need to refresh the website copy of the README, use:

```bash
./frb_internal generate-internal-readme
```

## Step 6: Resolve merge conflicts in contributor PRs

These PRs usually conflict only in:

- `.all-contributorsrc`
- `README.md`

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
4. Re-run `./frb_internal generate-internal-contributor` if you edited contributor metadata or finished filling in `.all-contributors-custom.yaml`.
5. `git add` the resolved files and commit the merge.
6. Push the contributor branch back to origin.

Do not revert unrelated contributor entries that were merged by other PRs.

## Step 7: Merge the contributor PRs

After the branch is updated and GitHub reports it mergeable:

```bash
gh pr merge <pr-number> --repo fzyzcjy/flutter_rust_bridge --merge --delete-branch
```

Notes:

- GitHub may temporarily report `DIRTY` or reject merge immediately after push.
- If that happens, wait briefly, re-check the PR state, and retry the merge.
- After merging one contributor PR, fetch `origin/master` again before resolving the next one.

## Step 8: Final verification

At the end, verify there are no remaining open all-contributors PRs:

```bash
gh pr list --repo fzyzcjy/flutter_rust_bridge --search "author:allcontributors[bot] is:open" --limit 50 --json number,title,mergeStateStatus,url
```

Also verify the credited contributors appear in:

- `.all-contributorsrc`
- `README.md`
- `website/docs/index.md` if contributor regeneration was run
