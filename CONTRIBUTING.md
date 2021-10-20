## Contributing guide

Firstly, welcome, and thanks for your contributions!

If you want to contribute, feel free to create a Pull Request. If you need some ideas of what to contribute, have a look at the Issues section of this repository.

### Checklist

* **Add tests**: Edit `frb_example/pure_dart` example to add some functions as if you are a user using the new feature. For example, you may change `rust/src/api.rs` and `dart/lib/main.dart` together.
* **Run code generator**: Run the code generator (`frb_codegen`) for *both* `frb_example/pure_dart` and `frb_example/with_flutter`. You need to commit the changes to git (if any), otherwise CI will fail.
* **Pass CI**: The code is covered by CI, and please ensure the CI passes, which often catches bugs. 
* **Ping** (if needed): If the PR is submitted but I do not reply for a few days, maybe I just did not see it, so please ping me.

### Releasing a new version

Usually this is done by the owner (@fzyzcjy), so you do not need to do the following. If you need to release a new version, the following steps are needed. Bump several versions, write down a changelog, and use `cargo check` to automatically update the examples' dependency versions:

```
vim frb_codegen/Cargo.toml && vim frb_rust/Cargo.toml && vim frb_dart/pubspec.yaml && vim CHANGELOG.md && (cd frb_example/pure_dart/rust && cargo check) && (cd frb_example/with_flutter/rust && cargo check) && (cd frb_example/pure_dart/dart && dart pub get) && (cd frb_example/with_flutter && dart pub get)
```

