---
name: frb-develop-feature
description: Use when adding tests or developing new features in flutter_rust_bridge, covers test patterns and testing bed setup
---

# FRB Develop Feature

## Overview

Guide for developing features and adding tests in flutter_rust_bridge.

## Use dart_minimal as Testing Bed

`frb_example/pure_dart` is large and slow to compile. Use `frb_example/dart_minimal` as a quick testing bed:

- Ad-hoc add functions to dart_minimal when examining outputs and behavior
- Faster iteration cycle during development

## How to Add a Test

:::info
This package has scripts to automatically create more tests based on the test you write, so you write one test and get (usually) six tests ;)
:::

### Steps

1. In `./frb_example/pure_dart`, add your test in:
   - `rust/src/api/whatever.rs`
   - `test/api/whatever_test.dart`

2. Mimic existing tests and add `_twin_normal`/`TwinNormal`/etc as appropriate (this allows more "twin" tests to be generated)

3. Run generation:
   ```bash
   # Full (slow)
   ./frb_internal precommit --mode slow

   # Or faster alternative
   ./frb_internal generate-internal-frb-example-pure-dart && ./frb_internal generate-run-frb-codegen-command-generate --package frb_example/pure_dart
   ```

## Related Skills

- `frb-code-generation` - What generation commands to run
- `frb-test` - How to run tests locally
