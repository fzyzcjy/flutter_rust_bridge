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

## Common Errors

| Error | Solution |
|-------|----------|
| `store_dart_post_cobject` undefined | Run `cargo build` again (generated rs not included) |
| iOS TestFlight symbol error | Xcode: `Strip Linked Product` → `No` |
| `ld: error: unable to find library -lgcc` | Downgrade Android NDK to v22 |
| `__cxa_pure_virtual` on Android | Add `build.rs`: `println!("cargo:rustc-link-lib=c++_shared")` |
| `frb_expand` cfg warning | Add to `Cargo.toml` lints: `check-cfg = ['cfg(frb_expand)']` |
| `dlopen failed: library not found` | Cargokit version mismatch with Flutter - upgrade/downgrade to match |
| Linker undefined symbols (iOS/macOS) | Add `-lc++` to podspec `OTHER_LDFLAGS` |

## Full Troubleshooting

If above doesn't help, see:

* `website/docs/manual/troubleshooting.md`
* `website/docs/docs/guides/contributing/tip.md`

## Related Skills

- `frb-code-generation` - Which generation commands to run
- `frb-develop-feature` - Development workflow
