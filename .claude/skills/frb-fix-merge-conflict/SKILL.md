---
name: frb-fix-merge-conflict
description: Use when resolving merge or rebase conflicts in flutter_rust_bridge, especially conflicts involving generated files after master/main changed independently from a PR branch.
---

# FRB Fix Merge Conflict

Use this skill before resolving merge or rebase conflicts in `flutter_rust_bridge`.

## Core Rule

Resolve hand-written files semantically. Do not hand-resolve generated files as the final result.

Generated conflict output should be treated as a stale artifact from two independently valid histories. After the hand-written sources are merged, regenerate the generated outputs and commit that regenerated result.

## Before Starting

1. Read the FRB environment setup instructions for the current checkout before running generation or tests.
2. Read `frb-code-generation` to choose the minimal regeneration command.
3. If the conflict is discovered through CI or PR checks, read `frb-fix-ci` before deeper investigation.
4. Make sure the worktree starts clean except for the merge/rebase conflict you are intentionally resolving.

## Classify Conflicts

List unmerged files:

```bash
git diff --name-only --diff-filter=U
```

Classify each file into one of these buckets:

| Bucket | Examples | Resolution |
|--------|----------|------------|
| Hand-written source | Rust APIs, codegen logic, Dart tests, docs, configs | Resolve manually with normal semantic merge |
| Generated output | `frb_generated.*`, `*.freezed.dart`, `*.g.dart`, generated API wrappers, generated test entrypoints | Pick one side only as a temporary placeholder, then regenerate |
| Ambiguous generated-like output | lockfiles, copied template outputs, integrate scaffolds | Confirm source of truth before choosing; use `frb-fix-ci` dependency graph if unsure |

Do not use the generated-file shortcut for hand-written files merely because they are noisy.

## Generated File Conflict Workflow

When a generated file conflicts because both base and PR independently added tests or APIs:

1. Resolve all hand-written source conflicts first.
2. For each generated conflicted file, choose a conflict-free placeholder from either side.
   The chosen side is not the final answer; it only clears conflict markers so the generator can run.
3. Stage those placeholder generated files.
4. Run the appropriate generation command from `frb-code-generation`.
   For broad PR-vs-master generated drift, prefer:

```bash
./frb_internal precommit-generate
```

   For `frb_example/pure_dart` and `frb_example/pure_dart_pde` conflicts, the internal generator and package codegen cover different outputs. If the merge touches generated test entrypoints, copied PDE files, or the pure-Dart chain, run:

```bash
./frb_internal generate-internal-frb-example-pure-dart
```

   If the merge also brings in hand-written Rust or Dart API changes, run package codegen after the internal generator:

```bash
./frb_internal generate-run-frb-codegen-command-generate --package frb_example/pure_dart
./frb_internal generate-run-frb-codegen-command-generate --package frb_example/pure_dart_pde
```

   Do not stop after the internal generator when CI errors mention missing methods on generated Dart APIs such as `RustLibApi`; that usually means the package `frb_generated.*` files are stale.

5. Inspect the regenerated diff. The final generated files should reflect both sides' hand-written sources.
6. Commit the merge/rebase resolution.

### Placeholder Commands

For a normal `git merge master` into a PR branch, this is usually enough for generated paths:

```bash
git checkout --ours -- <generated-path>
git add <generated-path>
```

For a `git rebase`, remember Git reverses the intuitive meaning of ours/theirs: `--ours` is the branch being rebased onto, and `--theirs` is the replayed commit. Since generated files are only placeholders, either side can be acceptable if the file still exists and regeneration will overwrite it. When deletion/addition is involved, prefer the side that leaves the path in the shape expected by the generator, then verify after regeneration.

## Validation

After regeneration:

```bash
git diff --check
git diff --name-only --diff-filter=U
git status --short
```

Then run the relevant local Docker-backed checks from the project skills. For broad generated-file conflict repairs, start with the same generation command CI would run, then use CI to validate the full matrix.

If CI later reports more generated diff, add the experience back to this skill so the next repair has a sharper rule.

## Commit Shape

Keep the conflict-resolution commit focused:

- Include semantic hand-written conflict resolutions.
- Include regenerated outputs from the chosen generation command.
- Do not include unrelated cleanup.
- Mention the generation command in the commit message body when useful.
