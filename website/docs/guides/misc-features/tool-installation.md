# Tool installation

`flutter_rust_bridge_codegen` automatically installs missing tools when needed
(`cargo-expand` for code generation, and `wasm-pack` and `wasm-bindgen-cli` for web builds).

If you manage your toolchain yourself (for example, with `mise` or in Docker),
pass `--skip-tool-install` to report missing tools instead of installing them:

```shell
flutter_rust_bridge_codegen generate --skip-tool-install
```

The flag is also available for web builds:

```shell
flutter_rust_bridge_codegen build-web --skip-tool-install
```

Install the required tools yourself and ensure they are available on `PATH` before running these commands.
