# jazz-chord demo user

Minimal reproduction package for flutter_rust_bridge issue #3164.

This package scans the local `../jazz-chord` crate with third-party automatic mode:

```bash
cargo run --manifest-path frb_codegen/Cargo.toml -- generate --config-file temp/jazz-chord-demo-user/flutter_rust_bridge.yaml
cargo check --manifest-path temp/jazz-chord-demo-user/rust/Cargo.toml
```

Current state:

- `generate` succeeds after the FRB missing-owner panic guard.
- `cargo check` succeeds for the currently reduced `../jazz-chord` fixture.
- The current fixture intentionally keeps only a minimal bridgeable subset. The next step is to add back real `jazz-chord` code incrementally and either fix FRB or add targeted overrides whenever the scan fails again.
