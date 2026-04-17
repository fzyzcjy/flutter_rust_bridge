---
name: frb-fix-ci
description: Use when CI fails in flutter_rust_bridge - before deep investigation
---

# FRB Fix CI

> **Note:** Check your user-level `remote-testing` rules before running commands. Tests and codegen may require remote execution.
> **Note:** When reproducing FRB failures remotely, do not run multiple remote commands in parallel against the same workspace/container. Serialize them unless you have isolated workspaces.

## Overview

CI failures in flutter_rust_bridge often have simple fixes. Try the appropriate approach below before deep investigation.

**Core principle:** Start with lazy fixes (re-run, copy diff, --fix) before expensive investigation.

## Triage Order

Use this order before diving into individual failure types:

1. Check the latest relevant run or job first. Do not reason from stale CI state.
2. If the failure looks flaky, rerun only the failed jobs.
3. Reproduce the exact failing `./frb_internal ...` command from CI, but first check your user-level `remote-testing` rules instead of assuming local execution is correct.
   If reproducing remotely, keep one remote workspace single-writer: do not run multiple FRB reproduction commands in parallel against that same workspace.
   Also do not trust a dirty remote workspace by default: FRB remote runs often leave stray generated or untracked files behind, so prefer `git reset --hard HEAD && git clean -fdx` before a meaningful reproduction.
4. Decide where the failure sits in the dependency graph: is it more likely a prerequisite cause (`Generate`, `Integrate`, or a high-relevance `Generate Internal` stage) or a downstream symptom (`Build :: Flutter`, native tests)?
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

Legend: rectangles are files or directories. Ovals are CI operations. In each oval, the first line is the CI job name and the second line in parentheses is the corresponding `./frb_internal ...` command when there is one.

```mermaid
flowchart LR
    CodegenSources["frb_codegen/src/** + codegen config"]
    Versions["pinned Flutter / Dart / Rust versions"]
    Templates["frb_codegen/assets/integration_template/**"]
    Cargokit["cargokit"]
    PureDart["frb_example/pure_dart/**"]
    GeneratedOutputs["frb_example/**/frb_generated.*"]
    PureDartGenerated["frb_example/pure_dart/**/frb_generated.*"]
    ExampleOutputs["integrate outputs under frb_example/**"]
    PureDartPde["frb_example/pure_dart_pde/**"]
    PureDartPdeGenerated["frb_example/pure_dart_pde/**/frb_generated.*"]
    NativeTests(["Test :: Flutter :: Native::*\n(./frb_internal test-flutter-native ...)"])

    Generate(["Generate :: FRB Codegen :: Command Generate\n(./frb_internal generate-run-frb-codegen-command-generate)"])
    Integrate(["Generate :: FRB Codegen :: Command Integrate\n(./frb_internal generate-run-frb-codegen-command-integrate)"])
    Build(["Build :: Flutter\n(./frb_internal build-flutter ...)"])

    subgraph GenerateInternal["Generate Internal stages"]
        GenerateInternalPureDart(["Generate Internal :: frb_example pure_dart chain\n(./frb_internal generate-internal-frb-example-pure-dart)"])
        GenerateInternalRust(["Generate Internal :: Rust\n(./frb_internal generate-internal-rust)"])
        GenerateInternalOther(["Other internal stages\n(book-help / dart-source /\nbuild-runner / contributor / readme)"])
    end

    CodegenSources -->|used by| Generate
    Versions -->|used by| Generate
    Generate -->|writes| GeneratedOutputs

    CodegenSources -->|used by| GenerateInternalPureDart
    Versions -->|used by| GenerateInternalPureDart
    PureDart -->|read by| GenerateInternalPureDart
    GenerateInternalPureDart -->|rewrites| PureDart
    GenerateInternalPureDart -->|derives| PureDartPde
    GenerateInternalPureDart -->|writes| PureDartGenerated
    GenerateInternalPureDart -->|writes| PureDartPdeGenerated

    CodegenSources -->|used by| GenerateInternalRust
    Versions -->|used by| GenerateInternalRust
    Versions -->|used by| GenerateInternalOther

    CodegenSources -->|used by| Integrate
    Versions -->|used by| Integrate
    Templates -->|used by| Integrate
    Cargokit -->|used by| Integrate
    Integrate -->|writes| ExampleOutputs

    GeneratedOutputs -->|consumed by| Build
    GeneratedOutputs -->|consumed by| NativeTests
    PureDartGenerated -->|consumed by| Build
    PureDartPdeGenerated -->|consumed by| Build
    ExampleOutputs -->|consumed by| Build
    ExampleOutputs -->|consumed by| NativeTests
```

#### How to Read

Read the graph as artifact and input dependencies, not as a literal GitHub Actions job graph.

#### Key Chains

- `frb_codegen/assets/integration_template/` + `cargokit` -> integrate outputs under `frb_example/**`
  If Flutter integrate examples, example platform files, `Build :: Flutter`, and native Flutter tests regress together, suspect these template inputs first. Do not hand-edit generated example outputs. If the bug is actually inside the embedded `cargokit` submodule, it is acceptable to edit that submodule directly and push to `fzyzcjy/cargokit`, then update the submodule ref.
- Apple scaffold under integrate examples:
  when Linux-side raw `create/integrate` does not reproduce checked-in Apple files such as `.metadata` iOS stanzas, `ios/**`, `example/ios/**`, `macos/Podfile`, or Apple-specific `pubspec.yaml` fragments, do not treat that as proof the checked-in files are wrong. In this repo, checked-in mac-generated Apple scaffold may be the source of truth, and Linux integrate may explicitly apply it before diff comparison.
- `Generate Internal` + `frb_example/pure_dart/**` -> `frb_example/pure_dart_pde/**`
  If `pure_dart_pde` is failing, do not only refresh `pure_dart_pde`. First check whether `./frb_internal generate-internal-frb-example-pure-dart --set-exit-if-changed ...` is still changing `frb_example/pure_dart`.

#### When to Consult

Use this graph when several nearby categories start failing together in the same run, especially when earlier nodes such as `Generate`, `Integrate`, or the high-relevance `Generate Internal` stages are already red and later failures look consistent with missing, stale, or mismatched generated files or platform files.

#### Rule

Prefer fixing prerequisite nodes before symptom nodes. If a prerequisite node is still unstable, treat later failures as propagated symptoms until proven otherwise. If this pattern keeps repeating across multiple commits or CI runs, jump to `Whack-a-Mole Prevention`.

## Whack-a-Mole Prevention

This section is about history across multiple commits or CI runs, not a single failing job.

Use it when the same area keeps becoming green and then red again, especially with commits like `refresh`, `regenerate`, `sync`, and `revert`.

What this usually means:

- You may be chasing generated outputs instead of fixing their source inputs
- A prerequisite node on the dependency graph is still unstable
- A temporary green run may only mean one symptom was suppressed, not that the root cause was fixed

What to do:

- Stop adding more generated-output-only sync commits by default
- Go back to the dependency graph and identify the unstable source of truth first: `frb_codegen/src/**`, `frb_codegen/assets/integration_template/**`, pinned Flutter/Dart/Rust versions, generation order, or package relationship
- Only accept regenerated outputs after that source node is stable in a clean matching environment
- If you have already accepted two package-level `Generate` sync commits with very similar generated Dart drift in one session or across adjacent CI runs, stop the package-by-package loop and escalate to a clean remote `./frb_internal precommit-generate`

Common FRB patterns:

- Flutter integrate examples:
  suspect `frb_codegen/assets/integration_template/` and `cargokit`; do not hand-edit generated example outputs. If the real fix belongs in `cargokit`, patch the submodule and update its ref instead of only patching copied outputs.
- `pure_dart` and `pure_dart_pde`:
  if both are moving, stabilize `frb_example/pure_dart` first and treat `pure_dart_pde` as a dependent output
- Repeated `Generate` failures across different example packages:
  if the same `Generate :: FRB Codegen :: Command Generate` symptom keeps rotating across packages with similar generated Dart diffs, stop fixing one package at a time. Re-run clean remote `./frb_internal precommit-generate` and treat that full run as the source of truth for the whole `Generate` chain. Only after that full run stabilizes should you accept any remaining per-package tail diff.
  Practical cutoff:
  after two similar package-level sync fixes, the burden of proof switches. Do not add a third similar package sync until you have checked whether clean remote `precommit-generate` collapses the whole diff surface.

### Escalation Protocol for Repeated Generate Drift

Trigger this protocol if both are true:

- Two or more example packages fail in the same `Generate :: FRB Codegen :: Command Generate` family across the current session or adjacent CI runs
- The diffs are mostly generated Dart outputs such as `frb_generated.dart`, `frb_generated.io.dart`, `frb_generated.web.dart`, generated API wrapper files, or nearby formatting/layout churn, with no evidence yet of a package-specific semantic bug or hand-written file change

Then do this, in order:

1. Stop accepting more package-by-package sync commits
2. Run clean remote `./frb_internal precommit-generate`
3. Pull back the remote workspace
4. Inspect the full diff surface
5. If the full diff surface collapses to one small tail package, accept that tail from this same clean run
6. If the diff surface is still broad, treat `precommit-generate` as the authoritative `Generate` source and debug the shared generation workflow before touching more package-level symptoms
7. Do not resume package-by-package syncs unless a clearly different failure class appears

Example:

- If `dart_minimal`, then `rust_ui_counter--ui`, then `gallery` fail in separate `Generate` jobs with similar generated Dart drift, stop after the second one and escalate to clean remote `precommit-generate`

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

CI shows the command it ran. Before running it, check your user-level `remote-testing` rules to determine whether this repo requires remote execution.

Before reproducing, make sure the toolchain versions match CI closely enough to be meaningful. In practice this usually means Flutter, Dart, Rust, cargo subcommands, and any pinned template or helper dependency should match the versions used by CI.

If reproducing remotely, clean the remote workspace first unless you have a specific reason not to. In FRB this often matters because old generated files and untracked outputs can survive earlier runs and create fake diffs:

```bash
git reset --hard HEAD && git clean -fdx
```

Then run the same command:

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
- If the bad logic is inside the embedded `cargokit` submodule, fix and push `fzyzcjy/cargokit`, then update the submodule ref
- Re-run integrate generation after updating the templates

Special case: Apple scaffold

- If Linux-side raw `create/integrate` drops checked-in Apple files, do not describe the current fix as "restoring old scaffold" unless it really is an accidental workaround.
- First check whether the repo intentionally treats checked-in mac-generated Apple scaffold as source of truth.
- If that is the contract, prefer names and comments like "apply checked-in Apple scaffold source of truth" over vague restore/preserve wording.

### Generate-caused Failures

If CI previously failed mainly in `Generate` while other jobs passed, and after accepting generated changes additional non-`Generate` jobs start failing, treat this as strong evidence that the accepted generated outputs are incorrect or incomplete.

In that situation:

- Do not continue fixing downstream failures one by one first
- First validate the generation logic or generation workflow
- Re-generate from a clean environment
- Only accept generated outputs after confirming they do not introduce new non-`Generate` regressions

If the symptom is not one package but a sequence of packages failing with very similar generated Dart drift, prefer validating clean remote `./frb_internal precommit-generate` before accepting more package-by-package sync commits.

If clean remote `precommit-generate` reduces the remaining diff surface to one small tail package, accept that tail from the same clean run and rerun CI. Do not go back into a long per-package loop unless a new failure class appears.

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

## Common Mistakes

- Investigating root cause when a simple re-run would work
- Not trying `git apply` first when CI provides a diff
- Fixing many new downstream test/build failures one by one after accepting generated changes, when CI previously failed mainly in `Generate`
- Hand-editing generated files to chase CI formatter output before checking whether CI, merge ref, and remote environments are formatting the same input
- Hand-editing integrate-generated example outputs instead of fixing `frb_codegen/assets/integration_template/`
- Assuming `cargokit` submodule changes are off-limits when the real bug is there
- Chasing repeated `refresh/regenerate/sync` diffs without re-checking the upstream generation inputs
- Continuing package-by-package `Generate` sync commits after two similar generated Dart drifts, instead of escalating to clean remote `precommit-generate`
- Fixing downstream build/test jobs before upstream generate/integrate/high-relevance generate-internal stages are stable
- Answering from stale CI state instead of reading the latest relevant run or job information first

## Related Skills

- `frb-code-generation` - Which generation commands to run
- `frb-debugging` - Deep debugging when simple fixes don't work
