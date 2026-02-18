---
name: frb-debugging
description: Use when generated code looks wrong, code generation fails, or you need to understand FRB internals
---

# FRB Debugging

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

## Related Skills

- `frb-code-generation` - Which generation commands to run
- `frb-develop-feature` - Development workflow
