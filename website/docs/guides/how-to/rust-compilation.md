# Rust compilation

Sometimes, some customization about Rust compilation may be needed,
such as [using nightly Rust](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1862) (instead of stable Rust).

To customize this, please refer to the documentations of the tool that is used in your project to compile Rust.
See [here](../../manual/integrate/overview) for a list of common tools.
For example, the default `flutter_rust_bridge_codegen create` command uses the Cargokit integration backend to compile Rust.
Thus, we can refer
to [the doc](https://github.com/irondash/cargokit/blob/main/docs/architecture.md#configuring-cargokit) and write down:

```yaml
# cargokit.yaml
cargo:
  release:
    toolchain: nightly
```

to fulfill the need of using nightly Rust.

If the project was generated with `--integration-backend native-assets`,
Rust compilation is driven by the generated Dart build hook and `flutter_rust_bridge_hooks`.
In that case, configure Rust through `rust-toolchain.toml`, Cargo features, and the options exposed by the hook package.
