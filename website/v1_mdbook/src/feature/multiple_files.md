# Multiple files

When having a large project, it is often insufficient to put everything in a single `api.rs`, but instead we may want to separate it into `api_of_one_module.rs`, `api_of_another_module.rs`, etc. That is why we have this feature.

Basically, just specify all input Rust files and all output locations and we are done. Here is an example:

```shell
flutter_rust_bridge_codegen \
  --rust-input "$REPO_DIR/native/src/api_1.rs" "$REPO_DIR/native/src/api_2.rs" \
  --dart-output "$REPO_DIR/lib/bridge_generated_api_1.dart" "$REPO_DIR/lib/bridge_generated_api_2.dart" \
  --class-name ApiClass1 ApiClass2 \
  --rust-output generated_api_1 generated_api_2
```

For more details, have a look at [this article](../article/generate_multiple_files.md).

