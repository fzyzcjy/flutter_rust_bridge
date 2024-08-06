# Use unreleased version

This page documents how to use the unreleased version of flutter_rust_bridge
(e.g. the version on master branch, or your custom local fork of it).

Suppose you have cloned the `flutter_rust_bridge` GitHub repository
and put it in `/path/to/your/flutter_rust_bridge`. Then:

* Change `flutter_rust_bridge.yaml` and set `local: true`.
* Change `Cargo.toml`'s `flutter_rust_bridge` dependency.
* Change `pubspec.yaml` flutter_rust_bridge dependency.
* Run `cargo run --manifest-path /path/to/your/flutter_rust_bridge/frb_codegen/Cargo.toml -- generate` instead
  of `flutter_rust_bridge_codegen generate`

