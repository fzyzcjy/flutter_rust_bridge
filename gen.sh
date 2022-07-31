set -ex
cargo run --manifest-path=frb_codegen/Cargo.toml -- --rust-input frb_example/pure_dart/rust/src/api.rs --dart-output frb_example/pure_dart/dart/lib/bridge_generated.dart --dart-format-line-length 120
cargo run --manifest-path=frb_codegen/Cargo.toml -- --rust-input frb_example/with_flutter/rust/src/api.rs --dart-output frb_example/with_flutter/lib/bridge_generated.dart --dart-format-line-length 120
cargo run --manifest-path=frb_codegen/Cargo.toml -- --rust-input frb_example/pure_dart_multi/rust/src/api_1.rs frb_example/pure_dart_multi/rust/src/api_2.rs --dart-output frb_example/pure_dart_multi/dart/lib/bridge_generated_api_1.dart frb_example/pure_dart_multi/dart/lib/bridge_generated_api_2.dart --class-name ApiClass1 ApiClass2 --rust-output frb_example/pure_dart_multi/rust/src/generated_api_1.rs frb_example/pure_dart_multi/rust/src/generated_api_2.rs --dart-format-line-length 120
cargo fmt
cargo build
dart format --line-length 120 frb_example/pure_dart/dart/
dart format --line-length 120 frb_example/pure_dart_multi/dart/
dart format --line-length 80 frb_dart
cd ~/flutter_rust_bridge/frb_example/pure_dart/dart && dart run lib/main.dart ~/flutter_rust_bridge/target/debug/libflutter_rust_bridge_example.so
#cargo run --manifest-path=frb_codegen/Cargo.toml -- --rust-input something/src/api.rs --dart-output something/something/lib/bridge_generated.dart --dart-format-line-length 120
