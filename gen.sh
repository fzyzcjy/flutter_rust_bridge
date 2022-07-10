set -ex
cargo run --manifest-path=frb_codegen/Cargo.toml -- --rust-input frb_example/pure_dart/rust/src/api.rs --dart-output frb_example/pure_dart/dart/lib/bridge_generated.dart --dart-format-line-length 120
dart format --line-length 120 frb_example/pure_dart/dart/
cargo fmt
dart format --line-length 80 frb_dart
cargo build
cd ~/flutter_rust_bridge/frb_example/pure_dart/dart && dart run lib/main.dart ~/flutter_rust_bridge/target/debug/libflutter_rust_bridge_example.so