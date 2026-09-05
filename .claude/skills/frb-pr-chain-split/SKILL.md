---
name: frb-pr-chain-split
description: Use when an FRB PR contains independently landable work that should be extracted into predecessor PRs and connected as a GitHub PR chain without wasting full automatic CI.
---

# 1 When to use

- Use this skill while developing or reviewing a `flutter_rust_bridge` PR when part of its diff is independent of the PR's stated objective.
- Split the independent work proactively. Do not wait for the user to notice the mixed scope.
- Treat a change as independently landable when it:
    - has a coherent behavior or maintenance objective of its own;
    - can be implemented and validated against the current default branch;
    - does not require the main task's API, schema, generated output, or version change;
    - leaves the main PR smaller and easier to review.
- Keep inseparable companion changes in the main PR. Generated snapshots, tests, and documentation required by the main change are not independent merely because they touch different files.

# 2 Choose the chain boundary

Classify every suspicious change before moving commits:

| Classification | Destination | Reason |
| --- | --- | --- |
| Independent bug fix or hardening | Predecessor PR | It can land and provide value before the main task |
| Independent mechanical refactor | Predecessor PR | It removes review noise from the main task |
| Main-task prerequisite with its own contract | Predecessor PR | The main PR can depend on a separately reviewable foundation |
| Generated output caused by the main task | Main PR | It cannot land meaningfully by itself |
| Test or documentation for the main behavior | Main PR | Splitting would separate the contract from its implementation |

- Prefer the smallest coherent predecessor, not the smallest textual diff.
- Create several predecessor PRs when the extracted work contains multiple independent contracts.
- Order predecessors by dependency. Do not infer order from commit timestamps.

# 3 Build each predecessor

- Fetch the canonical default branch before creating branches.
- Create the first predecessor from the current default branch.
- Create each later predecessor from the preceding predecessor branch.
- Reconstruct the coherent final delta. Do not blindly cherry-pick commits that mix predecessor and main-task changes.
- Include the predecessor's own tests and documentation.
- Validate the predecessor independently before pushing.
- Read `frb-prepare-pr` for generation, lint, test, and review requirements.
- Use one branch and one PR per independently landable unit.
- Push every chain branch to the same repository as the main PR. GitHub cannot use an unpushed local branch as a PR base.

# 4 Construct the GitHub PR chain

- Build a stacked branch chain, not sibling PRs that all target the default branch:

```text
master <- predecessor-a <- predecessor-b <- main-task
```

- Use GitHub's official `github/gh-stack` extension through `gh stack`.
- Use `gh stack` for every stack operation: initialization, restructuring, linking, submission, synchronization, rebasing, pushing, and verification.
- Never construct or maintain a stack with `gh pr create --base`, `gh pr edit --base`, direct API base edits, or branch ancestry alone.
- Git commands may prepare the commits and linear branch ancestry, but the result is not complete until `gh stack` creates or updates the native GitHub stack object.
- Check and install the extension when necessary. The extension requires `gh` 2.0 or later; a missing `gh stack` command normally means the extension is absent, not that core `gh` is outdated.

```bash
gh stack --help
gh extension install github/gh-stack
```

- For an unpublished main branch, branch it directly from the final predecessor.
- Initialize and submit a new local stack with `gh stack init` and `gh stack submit`.
- For an already published main PR:
    - create a timestamped backup tag before non-trivial history work;
    - record the original final tree and three-dot PR diff;
    - restack the main branch onto the final predecessor with a fully linear history;
    - do not introduce merge commits because native GitHub stacks require linear ancestry;
    - compare the rebuilt tree and PR delta with the recorded originals before pushing;
    - use `--force-with-lease` only after those checks when published history must be rewritten.
- Adopt existing branches or PRs with `gh stack link`. It pushes branches, creates missing PRs, corrects their base branches, and creates the native GitHub stack object.
- Create and connect a new stack with commands shaped like:

```bash
gh stack init --base master <predecessor-branch> <main-task-branch>
gh stack submit
```

- Adopt already prepared branches or existing PRs with:

```bash
gh stack link --base master <predecessor-branch> <main-task-branch>
```

- Verify local ancestry, native stack metadata, and PR bases:

```bash
git merge-base --is-ancestor <predecessor-branch> <main-task-branch>
gh stack view --json
```

- Compare three-dot diffs after constructing the chain:
    - `<previous-branch>...<predecessor-branch>` contains only that predecessor;
    - `<final-predecessor-branch>...<main-task-branch>` no longer contains the extracted work;
    - the final main-task tree is unchanged except for intentional cleanup.

# 5 Conserve CI on predecessor PRs

- Read `frb-ci-filter` before manipulating CI.
- Immediately add `ci-manual-dispatch` after creating every predecessor PR:

```bash
gh pr edit <predecessor-pr-number> --add-label ci-manual-dispatch
```

- Add the label even when local validation is already green. Predecessor PRs must not spend the full CI matrix merely because they were split for reviewability.
- The label action triggers a new `pull_request:labeled` run. The workflow concurrency group cancels the earlier automatic PR run, and the labeled run produces an empty heavy-job plan.
- Do not dispatch GitHub CI when local validation is sufficient.
- When a predecessor specifically needs GitHub-only evidence:
    - keep `ci-manual-dispatch` on the PR;
    - validate the filter locally;
    - dispatch only the smallest relevant job or matrix entry.

```bash
./frb_internal plan-ci --filter '<filter>'
gh workflow run ci.yaml --ref <predecessor-branch> -f 'ci_filter=<filter>'
```

- Do not treat filtered CI as full merge-readiness evidence.
- When a predecessor becomes the next PR to merge, remove `ci-manual-dispatch` and obtain the normal required CI unless the maintainer explicitly accepts narrower evidence.

# 6 Completion checks

- Confirm each predecessor can be reviewed without understanding the main task.
- Confirm each PR's GitHub base matches its immediate predecessor branch.
- Confirm Git ancestry matches the GitHub base chain.
- Confirm the main PR diff excludes all extracted changes.
- Confirm every predecessor PR has `ci-manual-dispatch` while it is not awaiting final full CI.
- Report the PR numbers, branch order, validation performed, and any intentionally dispatched `ci_filter`.

# 7 Reliability pitfalls

- Do not create independent sibling PRs and merely call them a chain.
- Do not substitute manual PR base edits for `gh stack link` or `gh stack submit`.
- Do not treat matching branch ancestry as sufficient; verify the native stack with `gh stack view --json`.
- Do not force-push a published main PR merely to make its history prettier.
- Do not split generated files away from the generator or source change that owns them.
- Do not leave an extracted commit duplicated in the visible main PR diff.
- Do not spend full CI on every dormant predecessor.
- Do not leave `ci-manual-dispatch` on the PR when claiming normal full-CI readiness.
