---
name: frb-fix-ci
description: Use when CI fails in flutter_rust_bridge - before deep investigation
---

# FRB Fix CI

> **Note:** Check your user-level `remote-testing` rules before running commands. Tests and codegen may require remote execution.

## Overview

CI failures in flutter_rust_bridge often have simple fixes. Try the appropriate approach below before deep investigation.

**Core principle:** Start with lazy fixes (re-run, copy diff, --fix) before expensive investigation.

## Triage Order

Use this order before diving into individual failure types:

1. Check the latest relevant run or job first. Do not reason from stale CI state.
2. If the failure looks flaky, rerun only the failed jobs.
3. Reproduce the exact failing `./frb_internal ...` command from CI, but first check your user-level `remote-testing` rules instead of assuming local execution is correct.
4. Decide where the failure sits in the dependency graph: is it more likely a prerequisite cause (`Generate` / `Integrate` / `Generate Internal`) or a downstream symptom (`Build :: Flutter`, native tests)?
5. Only do deeper debugging after you have ruled out flakes, stale runs, and failure propagation.

### Checking the Right Run

Do not answer from stale CI state. Read the latest relevant run or job information first.

## Quick Reference

| Symptom | Fix |
|---------|-----|
| Flaky test (passes sometimes) | `gh run rerun --failed` |
| Git diff shown in CI | `git apply` OR regenerate |
| Lint/format errors | Add `--fix` flag |
| Can't reproduce locally | Use same `./frb_internal` command from CI, following `remote-testing` rules |

## Dependency Order

When several related jobs are failing, use this dependency graph instead of treating all failures as peers:

```text
generation logic / templates / toolchain
    |
    +--> Generate / Integrate / Generate Internal
    |         |
    |         +--> generated outputs / example platform files
    |                    |
    |                    +--> Build :: Flutter
    |                               |
    |                               +--> native Flutter tests
    |
    +--> frb_codegen/assets/integration_template/ + cargokit
    |         |
    |         +--> integrate example outputs / platform files
    |                    |
    |                    +--> Build :: Flutter
    |                               |
    |                               +--> native Flutter tests
    |
    +--> frb_example/pure_dart
              |
              +--> frb_example/pure_dart_pde
```

- `generation logic / templates / toolchain` -> generated outputs
- `frb_codegen/assets/integration_template/` and `cargokit` -> integrate example outputs and platform files
- `frb_example/pure_dart` -> `frb_example/pure_dart_pde`
- `Generate` / `Integrate` / `Generate Internal` -> `Build :: Flutter`
- `Build :: Flutter` -> native Flutter tests

Practical rule:

- Prefer fixing the left side of the chain before the right side
- If the left side is still unstable, treat failures on the right side as potentially downstream symptoms rather than independent root causes

## Fixes by Failure Type

### Flaky Test

Sometimes CI fails due to timing issues, not real bugs. Rerun only failed jobs:

```bash
gh run rerun --failed
```

If it passes on retry -> flaky, not your bug.

### Git Diff Errors

When CI shows a diff, you have two options:

**Option A: `git apply` (faster)**

CI already ran the generator. Just apply what it computed:

```bash
# Copy the diff from CI, then:
pbpaste | git apply   # macOS
```

**Option B: Regenerate (slower but more "proper")**

```bash
./frb_internal precommit-generate
```

> **After codegen:** Check your user-level `remote-testing` rules. If codegen was run remotely, pull changes back to local.

Both are correct. Option A is faster; Option B is more thorough.

Do not hand-edit generated files as the final fix.

You may use CI diffs only as a diagnosis aid to understand what changed, but the final accepted output should come from re-running the appropriate generation workflow in a clean matching environment.

### Can't Reproduce Locally

CI shows the command it ran. Before running it, check your user-level `remote-testing` rules to determine whether this repo requires remote execution. Then run the same command:

```bash
# CI shows: ./frb_internal test-dart --package frb_example/pure_dart
./frb_internal test-dart --package frb_example/pure_dart
```

### Lint/Format Errors

For clippy, dart analyze, or format errors, use `--fix`:

```bash
./frb_internal lint --fix
```

This runs:
- `cargo clippy --fix` - Rust lint fixes
- `cargo fmt` - Rust format
- `dart format` - Dart format
- `dart fix --apply` - Dart auto fixes

When lint/format failures happen on generated files, do not default to hand-editing those files just to match formatter output.

Instead:
- Compare the formatter inputs across branch head, PR merge ref, and remote workspace
- Check whether generation order, hidden generation steps, or toolchain/environment drift changed the generated file before formatter ran
- Only accept formatter output after confirming the pre-format generated input is actually the correct one

### `Command Integrate` Failures

When `Generate :: FRB Codegen :: Command Integrate` fails because integrated output is wrong, do not hand-edit the generated integrate example outputs.

Instead:
- Fix the source templates under `frb_codegen/assets/integration_template/`
- Re-run integrate generation after updating the templates

### Generate-caused Failures

If CI previously failed mainly in `Generate` while other jobs passed, and after accepting generated changes additional non-`Generate` jobs start failing, treat this as strong evidence that the accepted generated outputs are incorrect or incomplete.

In that situation:

- Do not continue fixing downstream failures one by one first
- First validate the generation logic or generation workflow
- Re-generate from a clean environment
- Only accept generated outputs after confirming they do not introduce new non-`Generate` regressions

### Failure Propagation

When CI starts failing in several adjacent categories at once, do not assume they are independent.

In that situation:

- Treat `Generate` / `Integrate` / `Generate Internal` as earlier nodes in the dependency graph than `Build :: Flutter` and native tests
- If earlier nodes are still unstable, do not spend most of your effort fixing later nodes one by one yet
- First stabilize the earlier generation or template inputs, then re-check the later jobs

### Dart Web Browser Startup Flakes

When `Test :: Dart :: Web (...)` fails after the web build and server startup already succeeded, and the failure is:

```text
Exception: Websocket url not found.
```

treat it as a likely browser / puppeteer startup flake first, not an immediate code regression.

In that situation:

- Do not assume the generated code or test logic is broken just from this error
- Check whether wasm build, `dart compile js`, and local web server startup already succeeded
- Prefer rerunning only the failed job first
- Only start code investigation if the same job keeps failing with the same error repeatedly

## Special Chains

### `pure_dart_pde` Related Failures

When the failing path involves `frb_example/pure_dart_pde`, remember that `pure_dart_pde` is derived from `frb_example/pure_dart`.

In that situation:

- Do not only refresh `pure_dart_pde`
- First check whether `./frb_internal generate-internal --set-exit-if-changed ...` is still changing `frb_example/pure_dart`
- Treat `frb_example/pure_dart` as the upstream source and `frb_example/pure_dart_pde` as the downstream copy
- If `pure_dart` still changes, sync that upstream output first, then re-check `pure_dart_pde`

### Flutter Integrate Template Chain

When Flutter integrate examples, example platform files, `Build :: Flutter`, and native Flutter tests regress together:

- Treat `frb_codegen/assets/integration_template/` and `cargokit` as upstream of generated example platform files
- Suspect the template chain before assuming the generated outputs are independently wrong
- Prefer fixing template inputs over hand-editing generated example outputs

## Whack-a-Mole Prevention

When the same path or package repeatedly shows commits like `refresh`, `regenerate`, `sync`, and `revert`, treat that as a sign you may be chasing outputs instead of fixing inputs.

In that situation:

- Stop adding more generated-output-only sync commits by default
- Identify the upstream source of truth first: generation logic, templates, toolchain version, generation order, or package relationship
- Only accept regenerated outputs after the upstream source is stable in a clean matching environment

For Flutter integrate examples specifically:

- Treat `frb_codegen/assets/integration_template/` and `cargokit` as prerequisite nodes for generated example platform files
- If `Generate :: FRB Codegen :: Command Integrate`, `Build :: Flutter`, and native Flutter tests all regress together, suspect the template chain first
- Prefer fixing template inputs over hand-editing generated example outputs

For `pure_dart` specifically:

- Treat `frb_example/pure_dart` as a prerequisite node for `frb_example/pure_dart_pde`
- If both are moving, stabilize `pure_dart` first
- Do not conclude the problem is fixed just because one downstream regeneration temporarily makes CI greener once

## Common Mistakes

- Investigating root cause when a simple re-run would work
- Not trying `git apply` first when CI provides a diff
- Fixing many new downstream test/build failures one by one after accepting generated changes, when CI previously failed mainly in `Generate`
- Hand-editing generated files to chase CI formatter output before checking whether CI, merge ref, and remote environments are formatting the same input
- Hand-editing integrate-generated example outputs instead of fixing `frb_codegen/assets/integration_template/`
- Chasing repeated `refresh/regenerate/sync` diffs without re-checking the upstream generation inputs
- Fixing downstream build/test jobs before upstream generate/integrate jobs are stable
- Answering from stale CI state instead of reading the latest relevant run or job information first

## Related Skills

- `frb-code-generation` - Which generation commands to run
- `frb-debugging` - Deep debugging when simple fixes don't work
