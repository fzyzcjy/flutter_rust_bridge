---
name: frb-code-generation
description: Use when modifying Rust APIs, codegen, generated examples, or platform scaffolds in flutter_rust_bridge to select generation commands and preserve source-of-truth and convergence rules
---

# FRB Code Generation

> **Note:** Check your user-level `remote-testing` rules before running commands. Codegen may require remote execution.

## Overview

flutter_rust_bridge requires code generation when Rust APIs change. This skill maps change types to the minimal generation commands needed.

**Core principle:** Run only the generation commands needed for your change type.

**Validation rule:** If regenerated outputs cause previously green non-`Generate` jobs to fail, treat the generated outputs as suspect until they are validated from a clean environment.

> **After codegen:** Check your user-level `remote-testing` rules. If codegen was run remotely, pull changes back to local.

## Quick Reference

| Change Type | Command |
|-------------|---------|
| Example Rust API (`frb_example/**/rust/src/api/*.rs`) | `./frb_internal precommit-generate` |
| Flutter integrate examples | `./frb_internal precommit-integrate` |
| `frb_codegen/` generation logic | `./frb_internal precommit-generate` + `generate-internal-rust` |
| `frb_rust/src/` core API | `./frb_internal generate-internal-rust` |
| `frb_example/pure_dart` generator | `./frb_internal generate-internal-frb-example-pure-dart` |
| CLI help documentation | `./frb_internal generate-internal-book-help` |
| Root `README.md` or `CHANGELOG.md` | `./frb_internal generate-internal-readme` |
| Other non-generated docs, comments, tests, or `frb_dart/` source | No generation needed |

## Important Rules

### Root Documentation Fan-Out

Treat the root `README.md` and `CHANGELOG.md` as sources of truth. After changing either file, run:

```bash
./frb_internal generate-internal-readme
```

Commit the generated package documentation in the same change. Do not manually edit `frb_dart/README.md`, `frb_dart/CHANGELOG.md`, or `frb_hooks/CHANGELOG.md`.

### Integrate Template Drift

If a diff touches Flutter integrate example outputs, platform scaffolds, or copied `cargokit` files under `frb_example/**`, check whether the real source of truth should also change under `frb_codegen/assets/integration_template/`.

Do not submit only downstream integrate output changes when the behavior belongs in the template. Update the template first, then run:

```bash
./frb_internal precommit-integrate
```

Treat `frb_codegen/assets/integration_template/**/cargokit` as the source of truth for copied `cargokit` output. If the actual bug belongs inside the external `cargokit` submodule, read the `frb-cargokit` skill before deciding whether to patch the submodule and update its pointer.

For CI diagnosis rules about generated-file format/lint failures, repeated package-level `Generate` drift, or `Generate :: FRB Codegen :: Command Integrate` failures, you MUST read `frb-fix-ci` first. This skill is for command selection, not CI failure-propagation diagnosis.

For `pure_dart` / `pure_dart_pde` generation issues, treat `frb_example/pure_dart` as the upstream source and `frb_example/pure_dart_pde` as the derived copy. See `frb-fix-ci` for the CI diagnosis workflow.

If CI repair has already entered repeated package-level drift, you MUST stop choosing narrower commands and switch to `frb-fix-ci`.

Do not manually patch generated files as the final fix. The final accepted result should be produced by the corresponding generation command in a clean matching environment.

### Generation Convergence

- Run the narrowest owning generator before broad generation.
- Normalize the intended source inputs, resolve dependencies, run final code generation, and format with the target toolchain. Do not hand formatter-unstable intermediate text to a different generation lane.
- Run the owning generation command a second time and require a clean diff. A single successful run is not convergence.
- Preserve semantic equivalence in templates. Do not move construction, scope, lifetime, or callback boundaries merely to make new formatter output look smaller.

### Generated Output Provenance

Classify every changed path before accepting generated drift:

| Provenance | Required action |
| --- | --- |
| Integration template | Change the template source and regenerate consumers |
| Apple scaffold asset | Refresh through the owning scaffold workflow |
| CargoKit copy | Read `frb-cargokit` and follow its ownership and synchronization rules |
| Generated example output | Keep the generator and snapshot together |
| Flutter migrator edit in a legacy example | Apply the exact current scaffold migration and record it as direct |
| Unexplained manual edit | Revert it or identify its owning source before proceeding |

- When an old example has no supported regeneration entry point, compare it with a fresh target-Flutter scaffold and directly apply only required tool-defined migrations.
- Record direct legacy scaffold migrations separately from automatic output.

### OHOS Integrate Composition

- Treat the checked-in package and the older OHOS Flutter fork as two distinct sources of truth.
- Generate generic Flutter scaffold files with the current Flutter package.
- Overlay only explicit OHOS-owned paths from the OHOS fork: `ohos` and, when required by the package, `rust_builder/ohos` and `rust_builder/pubspec.yaml`.
- Exclude transient trees such as `node_modules` and preserve intentional deletions instead of resurrecting stale files.
- Stage and validate the composed result before replacing the canonical package. Restore the original package if create, dependency resolution, code generation, formatting, or the final swap fails.
- Test both success and failure paths. Do not add production APIs whose only purpose is exposing internals to tests.
