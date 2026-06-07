---
name: frb-ci-filter
description: Use when running focused flutter_rust_bridge GitHub Actions CI via ci_filter, adding/removing the ci-manual-dispatch PR label, choosing exact CI jobs or matrix entries, or documenting intentionally partial CI runs.
---

# FRB CI Filter

Use this skill when CI feedback is too expensive and the current task needs a focused GitHub Actions signal instead of automatic full CI.

The preferred mechanism is the formal `workflow_dispatch` input in `.github/workflows/ci.yaml`:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=<filter>'
```

Do not temporarily hack workflow jobs just to get partial CI unless the formal filter cannot express the target.

## Filter Syntax

```text
full
job_id
job_a,job_b
job_id[dimension=value]
job_id[dimension=value|other_value,other_dimension=value]
job_a,job_b[dimension=value]
```

- `full` or `*` runs the normal full CI plan.
- `job_id` runs the whole job, including its full matrix if it has one.
- `job_id[...]` runs only matching matrix entries.
- Bracket filters currently require `dimension=value`.
- Commas outside brackets combine multiple jobs.
- Explicit `dimension=value` conditions inside brackets are ANDed.
- `|` means OR within one dimension.
- Matrix dimensions include direct keys such as `image`, `package`, `device`, `api-level`, `sanitizer`, `platforms`, and nested `matrix.info` keys such as `platform`, `target`, `version`.
- Numeric matrix values are matched as strings, e.g. `api-level=35`.
- Values may contain spaces, e.g. iOS simulator device names.
- Unknown jobs, unknown dimensions, malformed filters, and filters matching no matrix entries fail in the `Plan :: CI` job.

## Manual Dispatch Label

Use the `ci-manual-dispatch` PR label when an agent or human wants to prevent automatic PR CI from running the full surface while iterating.

```bash
gh pr edit <pr-number> --add-label ci-manual-dispatch
```

With the label present:

- normal PR-triggered CI produces an empty plan for the heavy jobs
- `workflow_dispatch` remains available for explicit focused runs
- the PR must not be treated as merge-ready from partial runs alone

Before final readiness:

```bash
gh pr edit <pr-number> --remove-label ci-manual-dispatch
```

Then let the normal full CI surface run. Removing the label triggers PR CI because the workflow listens to `labeled` and `unlabeled` pull request events.

## Manual Dispatch PR Comment

Manual `workflow_dispatch` runs of `.github/workflows/ci.yaml` comment from `Plan :: CI` on the matching open PR with the selected `ci_filter` and run link; normal `push` and `pull_request` runs do not comment.

## Local Planning

Before dispatching an expensive manual run, smoke the filter locally:

```bash
./frb_internal plan-ci --filter 'test_dart_web[package=frb_example--pure_dart_pde]'
```

The command emits a GitHub Actions output line:

```text
plan={...}
```

The workflow consumes it with paths like:

```yaml
fromJSON(needs.plan_ci.outputs.plan).test_dart_web.enable
fromJSON(needs.plan_ci.outputs.plan).test_dart_web.matrix
```

That output shape is intentional; each CI job has its own top-level entry containing `enable`, and matrix jobs also contain `matrix`.

## Examples

These examples are protected by `tools/frb_internal/test/ci_plan_test.dart`; update that test whenever adding or changing examples here.

Full CI:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=full'
```

One non-matrix job:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=lint_dart_primary'
```

Several whole jobs:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=lint_dart_primary,test_dart_web'
```

One non-matrix job plus one matrix entry:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=lint_dart_primary,test_dart_web[package=frb_example--pure_dart]'
```

One Dart web package:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_dart_web[package=frb_example--pure_dart_pde]'
```

Two Dart web packages:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_dart_web[package=frb_dart|frb_example--pure_dart_pde]'
```

Two Dart web packages plus one non-matrix job:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_dart_web[package=frb_dart|frb_example--pure_dart_pde],lint_rust_primary'
```

Two precise matrix tuples in the same job:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_rust[image=ubuntu-latest,version=nightly],test_rust[image=ubuntu-latest,version=1.85.0]'
```

One Dart native package on Linux:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_dart_native[image=ubuntu-24.04,package=tools--frb_internal]'
```

Rust tests on selected toolchains:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_rust[image=ubuntu-latest,version=nightly|1.85.0]'
```

One Flutter desktop package on Linux:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_flutter_native_desktop[platform=linux,package=frb_example--gallery]'
```

One Android emulator matrix entry:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_flutter_native_android[package=frb_example--flutter_via_create,device=pixel,api-level=35]'
```

One iOS simulator matrix entry:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_flutter_native_ios[device=iPhone 16 Pro Max Simulator (18.6),package=frb_example--rust_ui_counter--ui]'
```

One codegen command-generate entry:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=generate_run_frb_codegen_command_generate[image=ubuntu-24.04,package=frb_example--integrate_third_party]'
```

One OHOS command-integrate entry:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=generate_run_frb_codegen_command_integrate[platforms=ohos]'
```

One sanitizer entry:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=test_dart_sanitizer[sanitizer=asan,package=frb_example--pure_dart]'
```

One benchmark runner:

```bash
gh workflow run ci.yaml --ref <branch> -f 'ci_filter=bench_dart_native[image=ubuntu-24.04]'
```

## Choosing Evidence

For intentional red reproduction PRs:

- Run only the single job or matrix entry that proves the reported bad behavior.
- State the exact `ci_filter` in the PR body.
- State the expected failure text or behavior.
- Do not treat the narrowed red run as fix readiness evidence.

For ordinary iteration branches:

- Prefer the smallest filter that exercises the changed behavior.
- Say in PR status/comments when only filtered CI has run.
- Before final review or merge readiness, remove `ci-manual-dispatch` and run normal CI.

## Validation

Before pushing changes to the filter mechanism or examples:

```bash
cd tools/frb_internal
dart test test/ci_plan_test.dart
```

Useful smoke check:

```bash
./frb_internal plan-ci --filter 'test_dart_web[package=frb_example--pure_dart_pde]'
```
