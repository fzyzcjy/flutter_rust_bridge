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

1. Enable in `flutter_rust_bridge.yaml`:
   ```yaml
   dump_all: true
   ```
2. Run code generation
3. Check `rust/target/frb_dump/` for:
   - Effective configuration
   - IR (intermediate representation)
   - Generated spec and code

### Debug Logs

```bash
RUST_LOG=debug flutter_rust_bridge_codegen ...
```

### CI Failures

- **Flaky tests**: Re-run CI first
- **Git diff errors**: Copy CI diff output and `git apply`
- **Local reproduction**: Use corresponding `./frb_internal` command from CI
- **Auto-fix**: Many `./frb_internal` commands have `--fix` option

## Full Troubleshooting

If above doesn't help, see:

* `website/docs/docs/guides/contributing/tip.md`
* `website/docs/manual/troubleshooting.md`

## Related Skills

- `frb-code-generation` - Which generation commands to run
- `frb-develop-feature` - Development workflow
