---
name: frb-debugging
description: Use when generated code looks wrong, code generation fails, or you need to understand FRB internals
---

# FRB Debugging

## Overview

Debugging techniques for flutter_rust_bridge code generation and runtime issues.

## When to Use

- Generated code looks wrong
- Code generation fails
- Need to understand FRB internals
- Runtime errors in generated code

## Debug Code Generation

When generated code looks wrong:

1. Enable dumping in `flutter_rust_bridge.yaml`:
   ```yaml
   dump_all: true
   ```

2. Run code generation

3. Inspect `rust/target/frb_dump/` for intermediate representations

## TODO

- [ ] Add more debugging techniques
- [ ] Add common error patterns and solutions
- [ ] Add runtime debugging tips

## Related Skills

- `frb-code-generation` - Which generation commands to run
- `frb-develop-feature` - Development workflow
