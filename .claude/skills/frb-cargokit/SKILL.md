---
name: frb-cargokit
description: Explain how cargokit is sourced, copied, and analyzed inside flutter_rust_bridge. Use when working on `integration_template/**/cargokit`, `frb_example/**/cargokit`, integrate/generate output drift, or any diff that mentions cargokit and you need to identify the true source of truth before judging the change.
---

# FRB Cargokit

## Overview

Use this skill to avoid mixing together three different things: the external cargokit repository, the template submodule pointers in FRB, and copied cargokit files inside generated examples.

The main job is to determine which layer a change belongs to before analyzing whether the change is correct.

## Mental Model

There are two important cargokit forms inside FRB:

1. External source
   FRB depends on an external cargokit repository through git submodules under `frb_codegen/assets/integration_template/**/cargokit`.

2. Copied outputs
   FRB commands such as integrate/generate often copy concrete cargokit files into `frb_example/**/cargokit` or `frb_example/**/rust_builder/cargokit`.

This means a large cargokit diff in examples often does not mean FRB designed a large new cargokit behavior. It may only mean a template or submodule change propagated into copied outputs.

## Source Of Truth

Treat these paths as the source of truth when deciding cargokit behavior:

- `frb_codegen/assets/integration_template/app/rust_builder/cargokit`
- `frb_codegen/assets/integration_template/plugin/cargokit`

Treat these paths as downstream copies unless the evidence shows otherwise:

- `frb_example/**/cargokit/**`
- `frb_example/**/rust_builder/cargokit/**`

Do not start by judging copied example files in isolation. First determine which source submodule pointer or template content produced them.

## Diff Analysis Order

When a user asks "what happened to cargokit?" follow this order:

1. Identify the layer.
   Decide whether the diff is a submodule pointer update, a template file change, or copied output churn.

2. If the change is a submodule pointer update, inspect the two external cargokit commits first.
   Compare the old and new submodule SHAs inside the submodule repository before saying anything about the FRB-side diff.

3. Only then inspect copied example files.
   Ask whether they merely reflect the upstream change or whether FRB also introduced independent local edits.

4. Separate semantic changes from churn.
   Distinguish real logic changes from format-only refreshes, lockfile updates, scaffold refreshes, and regenerated outputs.

## What To Say Explicitly

When answering, state which of these is true:

- "This is an external cargokit change exposed by a submodule pointer update."
- "This is downstream copy churn caused by integrate/generate."
- "This repository also has local copied cargokit edits in examples."

If discussing correctness, tie the judgment to the correct layer:

- For submodule updates, judge the diff between the two external SHAs.
- For copied outputs, judge whether the copy is expected and synchronized.
- Do not infer upstream intent from a regenerated example diff alone.

## Common Mistakes

Avoid these mistakes:

- Treating `frb_example/**/cargokit/**` as the primary implementation source.
- Assuming a large example diff implies a large semantic cargokit change.
- Analyzing copied files before checking the submodule pointer diff.
- Mixing "FRB updated the pointer" with "FRB locally rewrote cargokit."

## Scope Boundary

This skill explains ownership, propagation, and diff interpretation.

Use `frb-code-generation` when you need to know which generation command to run.
Use `frb-debugging` when generation or cargokit-related behavior is failing and needs deeper investigation.
