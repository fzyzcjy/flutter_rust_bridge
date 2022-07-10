# To use this file, install Just: cargo install just

frb_bin := "frb_codegen/target/debug/flutter_rust_bridge_codegen"
frb_pure := "frb_example/pure_dart"
frb_pure_multi := "frb_example/pure_dart_multi"
frb_flutter := "frb_example/with_flutter"
line_length := "120"
dylib := if os() == "windows" {
    "flutter_rust_bridge_example.dll"
} else if os() == "macos" {
    "libflutter_rust_bridge_example.dylib"
} else {
    "libflutter_rust_bridge_example.so"
}

default: gen-bridge

alias b := build
build:
    cd frb_codegen && cargo build

alias g := gen-bridge
gen-bridge: build
    {{frb_bin}} -r {{frb_pure}}/rust/src/api.rs \
                -d {{frb_pure}}/dart/lib/bridge_generated.dart \
                --dart-format-line-length {{line_length}}
    {{frb_bin}} -r {{frb_flutter}}/rust/src/api.rs \
                -d {{frb_flutter}}/lib/bridge_generated.dart \
                -c {{frb_flutter}}/ios/Runner/bridge_generated.h \
                -c {{frb_flutter}}/macos/Runner/bridge_generated.h \
                --dart-format-line-length {{line_length}}

alias l := lint
lint:
    dart format --fix .
    dart format --fix -l {{line_length}} {{frb_pure}}
    dart format --fix -l {{line_length}} {{frb_pure_multi}}
    dart format --fix -l {{line_length}} {{frb_flutter}}

alias t := test
test: test-pure test-integration
test-pure:
    cd {{frb_pure}}/rust && cargo b
    cd {{frb_pure}}/dart && \
        dart pub get && \
        dart lib/main.dart ../rust/target/debug/{{dylib}}
test-integration:
    cd {{frb_flutter}} && flutter test integration_test/main.dart

alias c := clean
clean:
    cd {{frb_pure}}/dart && flutter clean
    cd {{frb_pure}}/rust && cargo clean
    cd {{frb_pure_multi}}/dart && flutter clean
    cd {{frb_pure_multi}}/rust && cargo clean
    cd {{frb_flutter}} && flutter clean
    cd {{frb_flutter}}/rust && cargo clean

check:
    cd {{frb_pure}}/dart && dart pub get && dart analyze
    cd {{frb_pure}}/rust && cargo clippy
    cd {{frb_pure_multi}}/dart && dart pub get && dart analyze
    cd {{frb_pure_multi}}/rust && cargo clippy
    cd {{frb_flutter}} && flutter pub get && flutter analyze
    cd {{frb_flutter}}/rust && cargo clippy

refresh_all:
    (cd frb_rust && cargo clippy -- -D warnings)
    (cd frb_macros && cargo clippy -- -D warnings)
    (cd frb_example/pure_dart/rust && cargo clippy -- -D warnings)
    (cd frb_example/pure_dart_multi/rust && cargo clippy -- -D warnings)
    (cd frb_example/with_flutter/rust && cargo clippy -- -D warnings)
    (cd frb_example/pure_dart/dart && dart pub get)
    (cd frb_example/pure_dart_multi/dart && dart pub get)
    (cd frb_example/with_flutter && flutter pub get)

    just lint

    sed -i "" -e 's/pub.flutter-io.cn/pub.dartlang.org/g' frb_example/pure_dart/dart/pubspec.lock
    sed -i "" -e 's/pub.flutter-io.cn/pub.dartlang.org/g' frb_example/pure_dart_multi/dart/pubspec.lock
    sed -i "" -e 's/pub.flutter-io.cn/pub.dartlang.org/g' frb_example/with_flutter/pubspec.lock

publish_all:
    (cd frb_codegen && cargo publish)
    (cd frb_rust && cargo publish)
    (cd frb_macros && cargo publish)
    (cd frb_dart && flutter pub publish --force --server=https://pub.dartlang.org)

release old_version new_version:
    grep -q 'version = "{{old_version}}"' frb_codegen/Cargo.toml
    grep -q '{{new_version}}' CHANGELOG.md

    sed -i '' 's/version = "{{old_version}}"/version = "{{new_version}}"/g' frb_codegen/Cargo.toml
    sed -i '' 's/version = "{{old_version}}"/version = "{{new_version}}"/g' frb_rust/Cargo.toml
    sed -i '' 's/version = "{{old_version}}"/version = "{{new_version}}"/g' frb_macros/Cargo.toml
    sed -i '' 's/version: {{old_version}}/version: {{new_version}}/g' frb_dart/pubspec.yaml

    just refresh_all

    cd frb_codegen && ./contrib/scoop.json.sh > ./contrib/flutter_rust_bridge_codegen.json

    git add --all
    git status && git diff --staged | grep ''
    git commit -m "bump from {{old_version}} to {{new_version}}"
    git push

    awk '/## {{new_version}}/{flag=1; next} /## {{old_version}}/{flag=0} flag' CHANGELOG.md | gh release create v{{new_version}} --notes-file "-" --draft --title v{{new_version}}
    echo 'A *DRAFT* release has been created. Please go to the webpage and really release if you find it correct.'
    open https://github.com/fzyzcjy/flutter_rust_bridge/releases

    just publish_all

# vim:expandtab:ts=4:sw=4
