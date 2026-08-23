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

- Standard `gh` has no `gh pr chain` subcommand. Construct the chain with branch ancestry and GitHub PR base fields.
- For an unpublished main branch, branch it directly from the final predecessor.
- For an already published main PR:
    - create a timestamped backup tag before non-trivial history work;
    - preserve published history by default;
    - merge the final predecessor branch with `git merge --no-ff`;
    - resolve overlaps to preserve the already reviewed main-task result;
    - push normally instead of force-pushing;
    - change the main PR base to the final predecessor branch.
- Create and connect PRs with commands shaped like:

```bash
git push -u origin <predecessor-branch>
gh pr create --base <previous-branch> --head <predecessor-branch> --title "<title>" --body ""
gh pr edit <main-pr-number> --base <final-predecessor-branch>
```

- Verify both representations of the chain:

```bash
git merge-base --is-ancestor <predecessor-branch> <main-task-branch>
gh pr view <predecessor-pr-number> --json headRefName,baseRefName,state
gh pr view <main-pr-number> --json headRefName,baseRefName,state
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
- Do not change only the GitHub PR base; the branch ancestry must also contain the predecessor.
- Do not force-push a published main PR merely to make its history prettier.
- Do not split generated files away from the generator or source change that owns them.
- Do not leave an extracted commit duplicated in the visible main PR diff.
- Do not spend full CI on every dormant predecessor.
- Do not leave `ci-manual-dispatch` on the PR when claiming normal full-CI readiness.
