## Contributing guide

Firstly, welcome, and thanks for your contributions!

If you want to contribute, feel free to create a Pull Request. If you need some ideas of what to contribute, have a look at the Issues section of this repository.

### Checklist

* **Add tests**: Edit `frb_example/pure_dart` example to add some functions as if you are a user using the new feature. For example, you may change `rust/src/api.rs` and `dart/lib/main.dart` together.
* **Run code generator**: Run the code generator (`frb_codegen`) for *both* `frb_example/pure_dart` and `frb_example/with_flutter`, using the commands mentioned below. You need to commit the changes to git (if any), otherwise CI will fail.
* **Pass CI**: The code is covered by CI, and please ensure the CI passes, which often catches bugs. 
* **Update CHANGELOG.md**: Please update [the CHANGELOG.md](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/CHANGELOG.md) under root directory (ignore other changelog files - they will be updated automatically) by adding a line explaining what you have done.
* **Ping** (if needed): If the PR is submitted but I do not reply for a few days, maybe I just did not see it, so please ping me.

### Releasing a new version

Usually this is done by the owner (@fzyzcjy), so you do not need to do the following. If you need to release a new version, the following steps are needed. Bump several versions, change the version number in changelog, and use `cargo check` to automatically update the examples' dependency versions:

```
just release
```

### Appendix: Sample commands to run code generator

Just copied from [CI codegen.yml](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/codegen.yml).

```
(cd frb_codegen && cargo run --package flutter_rust_bridge_codegen --bin flutter_rust_bridge_codegen -- --rust-input ../frb_example/pure_dart/rust/src/api.rs --dart-output ../frb_example/pure_dart/dart/lib/bridge_generated.dart --dart-format-line-length 120 && cargo run --package flutter_rust_bridge_codegen --bin flutter_rust_bridge_codegen -- --rust-input ../frb_example/with_flutter/rust/src/api.rs --dart-output ../frb_example/with_flutter/lib/bridge_generated.dart --c-output ../frb_example/with_flutter/ios/Runner/bridge_generated.h --dart-format-line-length 120)
```

### Appenfix: Format and lint everything

```
(cd frb_codegen && cargo fmt --all); (cd frb_rust && cargo fmt --all); (cd frb_macros && cargo fmt --all); (cd frb_example/pure_dart/rust && cargo fmt --all); (cd frb_example/with_flutter/rust && cargo fmt --all);
(cd frb_codegen && cargo clippy); (cd frb_rust && cargo clippy); (cd frb_macros && cargo clippy); (cd frb_example/pure_dart/rust && cargo clippy); (cd frb_example/with_flutter/rust && cargo clippy);                                                                                                                                          
(cd frb_dart && dart format . --line-length 120); (cd frb_example/pure_dart/dart && dart format . --line-length 120); (cd frb_example/with_flutter && dart format . --line-length 120);
(cd frb_dart && dart analyze --fatal-infos); (cd frb_example/pure_dart/dart && dart analyze --fatal-infos); (cd frb_example/with_flutter && dart analyze --fatal-infos);
```

### Appenfix: Upgrade dependency in your dependent project

```
flutter pub upgrade flutter_rust_bridge
cargo update -p flutter_rust_bridge
```
