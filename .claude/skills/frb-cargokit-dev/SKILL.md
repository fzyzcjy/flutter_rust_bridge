---
name: frb-cargokit-dev
description: Develop or update CargoKit for flutter_rust_bridge. Use when a bug or feature belongs in fzyzcjy/Cargokit, when updating the FRB CargoKit submodule pointers, or when preparing the paired CargoKit and FRB PRs that synchronize CargoKit changes into FRB integrate templates and examples.
---

# FRB CargoKit Development

## Overview

Use this skill when the actual fix belongs in the external `fzyzcjy/Cargokit` repository and FRB needs a follow-up PR that points its CargoKit submodules at the updated CargoKit branch or commit.

The usual shape is two PRs:

1. A CargoKit PR with the real implementation.
2. An FRB PR that updates the two CargoKit submodule pointers and regenerates copied integrate outputs.

Do not implement CargoKit behavior only in copied FRB example output. The source of truth is the CargoKit repository referenced by the template submodules.

## Source Locations

FRB embeds CargoKit as git submodules in two template locations:

- `frb_codegen/assets/integration_template/app/rust_builder/cargokit`
- `frb_codegen/assets/integration_template/plugin/cargokit`

Both submodules should normally point at the same CargoKit branch or commit for the same logical update. Downstream generated copies may appear under:

- `frb_example/**/cargokit/**`
- `frb_example/**/rust_builder/cargokit/**`

Read `frb-cargokit` first if you need to classify an existing diff before deciding where the change belongs.

## Workflow

### 1. Prepare The CargoKit Change

Work in `fzyzcjy/Cargokit` for the actual CargoKit implementation.

1. Create a CargoKit branch whose name is easy to reference from the FRB PR.
2. Implement the fix in CargoKit and run the relevant CargoKit checks from that repository.
3. Push the branch and open a CargoKit PR.
4. Keep the branch alive until the FRB PR has passed CI.

If you are editing through an FRB submodule checkout, remember that commits belong to the nested CargoKit repository, not the outer FRB repository. Push the nested repository branch before updating the outer FRB submodule pointer.

### 2. Update FRB To Point At The CargoKit Branch

In the FRB repository:

1. Initialize submodules if needed:

   ```bash
   git submodule update --init --recursive
   ```

2. Fetch and check out the CargoKit branch or commit in both submodule checkouts:

   ```bash
   git -C frb_codegen/assets/integration_template/app/rust_builder/cargokit fetch origin <branch-or-ref>
   git -C frb_codegen/assets/integration_template/app/rust_builder/cargokit checkout <commit-or-branch>
   git -C frb_codegen/assets/integration_template/plugin/cargokit fetch origin <branch-or-ref>
   git -C frb_codegen/assets/integration_template/plugin/cargokit checkout <commit-or-branch>
   ```

3. Stage the submodule pointer updates in the outer FRB repository:

   ```bash
   git add frb_codegen/assets/integration_template/app/rust_builder/cargokit
   git add frb_codegen/assets/integration_template/plugin/cargokit
   ```

4. Confirm the outer FRB diff shows submodule pointer changes for both paths.

Useful checks:

```bash
git submodule status --recursive
git diff --submodule
```

Do not leave one template submodule on the old CargoKit commit unless the app and plugin templates intentionally need different CargoKit revisions.

### 3. Regenerate FRB Integrate Outputs

After changing either CargoKit submodule pointer, run:

```bash
./frb_internal precommit-integrate
```

This updates copied CargoKit output and platform scaffold files under `frb_example/**`. Those downstream diffs are expected when CargoKit files are copied from the updated template.

Stage and commit the regenerated integrate output together with the submodule pointer update:

```bash
git add frb_example frb_codegen/assets/integration_template
git commit -m "Update CargoKit template pointers"
```

Then run the normal PR checks from `frb-prepare-pr`, especially:

```bash
./frb_internal lint --fix
```

Run focused tests when the CargoKit change affects a platform that can be exercised locally. Otherwise let FRB CI cover the platform matrix and use `frb-fix-ci` for failures.

### 4. Open The FRB PR

The FRB PR should mention that CargoKit is intentionally pointed at the temporary CargoKit PR branch or commit. Keep the PR body empty if the local PR workflow requires it, but use GitHub comments when you need to leave coordination notes.

Before asking for final review:

1. Wait for the FRB PR CI to pass with the temporary CargoKit pointer.
2. Ask reviewers to merge the CargoKit PR first.
3. Update the FRB submodule pointers from the temporary branch commit to the merged CargoKit commit on the canonical branch.
4. Rerun `./frb_internal precommit-integrate` if the CargoKit commit changed.
5. Push the refreshed FRB PR and wait for CI again.
6. Ask reviewers to merge the FRB PR.

## CI Failure Triage

When FRB CI fails after a CargoKit update, classify the failure before editing:

- CargoKit behavior failure: fix it in `fzyzcjy/Cargokit`, then update both FRB submodule pointers.
- Missing integrate refresh: run `./frb_internal precommit-integrate` and commit the copied output.
- Lint or formatting drift: run `./frb_internal lint --fix`.
- Platform-specific FRB scaffold issue: fix the FRB integration template, then run `./frb_internal precommit-integrate`.

Do not patch copied `frb_example/**/cargokit/**` files directly as the final fix. They should be produced from the updated CargoKit submodule and integrate templates.

## Related Skills

- `frb-cargokit` - Understand CargoKit ownership and diff layers
- `frb-code-generation` - Choose the correct generation command
- `frb-prepare-pr` - Prepare an FRB PR
- `frb-fix-ci` - Diagnose CI failures after the PR is opened
