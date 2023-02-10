# To use this file, install Just: `cargo install just`

frb_codegen_bin := "cargo run --manifest-path frb_codegen/Cargo.toml --"
dir_example_pure_dart := "frb_example/pure_dart"
dir_example_pure_dart_multi := "frb_example/pure_dart_multi"
dir_example_with_flutter := "frb_example/with_flutter"
default_line_length := "120"
dylib := if os() == "windows" {
    "flutter_rust_bridge_example.dll"
} else if os() == "macos" {
    "libflutter_rust_bridge_example.dylib"
} else {
    "libflutter_rust_bridge_example.so"
}
path_relative_linux_so := "target/x86_64-unknown-linux-gnu/debug/libflutter_rust_bridge_example.so"
dir_tools := justfile_directory() / "tools"

# ============================ install utilities ============================

install_llvm_linux:
    sudo apt update && sudo apt-get install -y libclang-dev

# ============================ build & test ============================

rust_build_and_test:
    just _rust_build_and_test_single frb_codegen
    just _rust_build_and_test_single frb_rust
    just _rust_build_and_test_single frb_macros
    just _rust_build_and_test_single {{dir_example_pure_dart}}/rust
    just _rust_build_and_test_single {{dir_example_with_flutter}}/rust
    just _rust_build_and_test_single {{dir_example_pure_dart_multi}}/rust
    just _rust_build_and_test_single {{dir_example_pure_dart_multi}}/rust --features c-output
    just _rust_build_and_test_single {{dir_example_pure_dart_multi}}/rust --features c-output,extra-c-output-path

_rust_build_and_test_single directory *args:
    cd {{directory}} && cargo build {{args}}
    cd {{directory}} && cargo test {{args}}

# ============================ linters ============================

rust_linter:
    cargo fmt
    cargo clippy -- -D warnings
    cd frb_rust && cargo clippy --target wasm32-unknown-unknown -- -D warnings

dart_linter mode="default":
    just _dart_linter_single {{mode}} frb_dart dart 80
    just _dart_linter_single {{mode}} {{dir_example_pure_dart}}/dart dart {{default_line_length}}
    just _dart_linter_single {{mode}} {{dir_example_pure_dart_multi}}/dart dart {{default_line_length}}
    just _dart_linter_single {{mode}} {{dir_example_with_flutter}} dart {{default_line_length}}
    just dart_linter_pana

_dart_linter_single mode directory executable line_length:
    cd {{directory}} && {{executable}} pub get
    cd {{directory}} && dart format \
      --line-length {{line_length}} \
      {{ if mode == "fix" { "--fix" } else { "--output=none --set-exit-if-changed" } }} \
      .
    cd {{directory}} && {{executable}} analyze --fatal-infos

dart_linter_pana:
    flutter pub global activate pana
    cd frb_dart && pana --no-warning --line-length 80 --exit-code-threshold 0

precommit:
    TODO rust_linter

    just gen-bridge
    just check
    just lint
    just gen-help
    # sed -i "" -e 's/pub.flutter-io.cn/pub.dartlang.org/g' frb_example/pure_dart/dart/pubspec.lock
    # sed -i "" -e 's/pub.flutter-io.cn/pub.dartlang.org/g' frb_example/pure_dart_multi/dart/pubspec.lock
    # sed -i "" -e 's/pub.flutter-io.cn/pub.dartlang.org/g' frb_example/with_flutter/pubspec.lock

gen-bridge:
    {{frb_codegen_bin}} -r {{dir_example_with_flutter}}/rust/src/api.rs \
                -d {{dir_example_with_flutter}}/lib/bridge_generated.dart \
                --dart-decl-output {{dir_example_with_flutter}}/lib/bridge_definitions.dart \
                -c {{dir_example_with_flutter}}/ios/Runner/bridge_generated.h \
                -e {{dir_example_with_flutter}}/macos/Runner/ \
                --dart-format-line-length {{default_line_length}} --wasm

test: test-support test-pure test-integration
test-pure:
    cd {{dir_example_pure_dart}}/rust && cargo b
    cd {{dir_example_pure_dart}}/dart && \
        dart pub get && \
        dart lib/main.dart ../../../target/debug/{{dylib}}
# TODO: Make ASan tests work for other platforms
test-pure-asan $RUSTFLAGS="-Zsanitizer=address":
    ./tools/dartsdk/fetch.sh
    cd {{dir_example_pure_dart}}/rust && cargo +nightly b --target x86_64-unknown-linux-gnu
    cd {{dir_example_pure_dart}}/dart && \
        {{dir_tools}}/dartsdk/x64/dart pub get && \
        {{dir_tools}}/dartsdk/x64/dart lib/main.dart  ../../../{{path_relative_linux_so}}

test-pure-web *args:
    cd {{dir_example_pure_dart}}/dart && just serve --dart-input lib/main.web.dart --root web/ -c ../rust --port 8081 {{args}}
test-flutter-web *args:
    cd {{dir_example_with_flutter}} && just serve -c rust {{args}}
test-integration:
    cd {{dir_example_with_flutter}} && flutter test integration_test/main.dart
test-support platform="chrome":
    cd frb_dart && dart pub get && \
        dart test test/*.dart && \
        dart test -p {{platform}} test/*.dart

clean:
    cd {{dir_example_pure_dart}}/dart && flutter clean
    cd {{dir_example_pure_dart}}/rust && cargo clean
    cd {{dir_example_pure_dart_multi}}/dart && flutter clean
    cd {{dir_example_pure_dart_multi}}/rust && cargo clean
    cd {{dir_example_with_flutter}} && flutter clean
    cd {{dir_example_with_flutter}}/rust && cargo clean

serve *args:
    cd {{invocation_directory()}} && dart run {{justfile_directory()}}/frb_dart/bin/serve.dart {{args}}

publish-all:
    (cd frb_codegen && cargo publish)
    (cd frb_rust && cargo publish)
    (cd frb_macros && cargo publish)
    (cd frb_dart && flutter pub publish --force --server=https://pub.dartlang.org)

release old_version new_version:
    grep -q 'version = "{{old_version}}"' Cargo.toml
    grep -q '{{new_version}}' CHANGELOG.md

    sed -i '' 's/version = "{{old_version}}"/version = "{{new_version}}"/g' Cargo.toml
    sed -i '' 's/version: {{old_version}}/version: {{new_version}}/g' frb_dart/pubspec.yaml

    just precommit

    cd frb_codegen && ./contrib/scoop.json.sh > ./contrib/flutter_rust_bridge_codegen.json

    git add --all
    git status && git diff --staged | grep ''
    git commit -m "bump from {{old_version}} to {{new_version}}"
    git push

    awk '/## {{new_version}}/{flag=1; next} /## {{old_version}}/{flag=0} flag' CHANGELOG.md | gh release create v{{new_version}} --notes-file "-" --draft --title v{{new_version}}
    echo 'A *DRAFT* release has been created. Please go to the webpage and really release if you find it correct.'
    open https://github.com/fzyzcjy/flutter_rust_bridge/releases

    just publish-all

gen-help:
    cargo run --manifest-path frb_codegen/Cargo.toml -- --help > book/src/help.txt
    dart run frb_dart/bin/serve.dart --help > book/src/help.serve.txt

# vim:expandtab:ts=4:sw=4
