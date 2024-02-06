# Regular upgrades

Just upgrade your `flutter_rust_bridge_codegen` and run the `generate` command.
Everything (the Dart and Rust flutter_rust_bridge dependency) will be automatically upgraded.

## Example

Suppose you installed the codegen by the standard `cargo install`, then just run:

```shell
cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.0' && flutter_rust_bridge_codegen generate
```
