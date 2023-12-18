# Misc operations in contributing

> Remark: Some docs here seem to be outdated. Refer to ci.yaml, main doc, justfile, etc to see an up-to-date version. This appendix will be overhauled.

## Releasing a new version

Usually this is done by the owner (@fzyzcjy), so you do not need to do the following. If you need to release a new version, the following steps are needed. Bump several versions, change the version number in changelog, and use `cargo check` to automatically update the examples' dependency versions:

```
just release
```

## Sample commands to run code generator

Just copied from [CI codegen.yml](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/codegen.yml).

```
(cd frb_codegen && cargo run --package flutter_rust_bridge_codegen --bin flutter_rust_bridge_codegen -- --rust-input ../frb_example/pure_dart/rust/src/api.rs --dart-output ../frb_example/pure_dart/dart/lib/bridge_generated.dart --dart-format-line-length 120 && cargo run --package flutter_rust_bridge_codegen --bin flutter_rust_bridge_codegen -- --rust-input ../frb_example/with_flutter/rust/src/api.rs --dart-output ../frb_example/with_flutter/lib/bridge_generated.dart --c-output ../frb_example/with_flutter/ios/Runner/bridge_generated.h --dart-format-line-length 120)
```

## Format and lint everything

```
(cd frb_codegen && cargo fmt --all); (cd frb_rust && cargo fmt --all); (cd frb_macros && cargo fmt --all); (cd frb_example/pure_dart/rust && cargo fmt --all); (cd frb_example/with_flutter/rust && cargo fmt --all);
(cd frb_codegen && cargo clippy); (cd frb_rust && cargo clippy); (cd frb_macros && cargo clippy); (cd frb_example/pure_dart/rust && cargo clippy); (cd frb_example/with_flutter/rust && cargo clippy);                                                                                                                                          
(cd frb_dart && dart format . --line-length 80); (cd frb_example/pure_dart/dart && dart format . --line-length 120); (cd frb_example/with_flutter && dart format . --line-length 120);
(cd frb_dart && dart analyze --fatal-infos); (cd frb_example/pure_dart/dart && dart analyze --fatal-infos); (cd frb_example/with_flutter && dart analyze --fatal-infos);
```

## Upgrade dependency in your dependent project

```
flutter pub upgrade flutter_rust_bridge
cargo update -p flutter_rust_bridge
```

