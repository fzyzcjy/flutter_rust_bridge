# jazz-chord demo user

Minimal reproduction package for flutter_rust_bridge issue #3164.

This package scans the local `../jazz-chord` crate with third-party automatic mode:

```bash
cargo run --manifest-path frb_codegen/Cargo.toml -- generate --config-file temp/jazz-chord-demo-user/flutter_rust_bridge.yaml
cargo check --manifest-path temp/jazz-chord-demo-user/rust/Cargo.toml
```

Current state:

- `generate` succeeds after the FRB missing-owner panic guard and the `#[frb(opaque)]` overrides for `Change`, `Extension`, and `Triad`.
- `cargo check` still fails because whole-crate automatic scanning exposes additional unsuitable methods and private/external types. The next reduction step should ignore or wrap that surface until the generated Rust compiles.
