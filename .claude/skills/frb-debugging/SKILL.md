---
name: frb-debugging
description: Use when generated code looks wrong, code generation fails, or you need to understand FRB internals
---

# FRB Debugging

> **Note:** Check your user-level `remote-testing` rules before running commands. Codegen and debugging may require remote execution.

## When to Use

- Generated code looks wrong
- Code generation fails
- Need to understand FRB internals
- Runtime errors in generated code

## Quick Diagnostics

### Dump Intermediate Representations

The in-tree examples by default enable `dump_all: true` in  `flutter_rust_bridge.yaml` for debugability.

Therefore, you can check `rust/target/frb_dump/` for:
- Effective configuration
- IR (intermediate representation)
- Generated spec and code
- ...

### Debug Logs

```bash
RUST_LOG=debug flutter_rust_bridge_codegen ...
```

## Full Troubleshooting

If above doesn't help, see:

* `website/docs/docs/guides/contributing/tip.md`
* `website/docs/manual/troubleshooting.md`

Before going deep into FRB internals, first check whether you are actually seeing failure propagation from unstable generated outputs or integrate templates.

In particular:

- If `Generate` / `Integrate` failures appear together with Flutter build or native test failures, use `frb-fix-ci` first
- If the same package keeps getting regenerated or reverted without stabilizing CI, use `frb-fix-ci` first
- If `pure_dart_pde` is failing, confirm whether `pure_dart` is still changing before debugging the downstream copy

## Related Skills

- `frb-code-generation` - Which generation commands to run
- `frb-fix-ci` - Diagnose CI failure propagation before deep debugging
- `frb-develop-feature` - Development workflow
