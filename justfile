# To use this file, install Just: `cargo install just`

pana := if os() == "windows" {
    "pana.bat"
} else {
    "pana"
}

sed := if os() == "macos" {
    "sed -i ''"
} else {
    "sed -i"
}

library_file_ext := if os() == "macos" { "dylib" } else { "so" }

dir_example_pure_dart := "frb_example/pure_dart"
dir_example_pure_dart_multi := "frb_example/pure_dart_multi"
dir_example_with_flutter := "frb_example/with_flutter"

# ============================ installation ============================

install_ffigen_dependency:
    # needed by `ffigen`, see https://github.com/dart-lang/ffigen#installing-llvm
    {{ if os() == "linux" { "sudo apt update && sudo apt-get install -y libclang-dev" } else { "" } }}

install_ffigen:
    dart pub global activate ffigen

install_valgrind:
    sudo apt install -y valgrind

install_prerequisite_for_integration_test_linux:
    sudo apt update && sudo apt-get -y install clang cmake ninja-build pkg-config libgtk-3-dev liblzma-dev libglu1-mesa

install_corrosion_linux:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd ..
    git clone https://github.com/corrosion-rs/corrosion.git
    cmake -Scorrosion -Bbuild -DCMAKE_BUILD_TYPE=Release
    cmake --build build --config Release
    sudo cmake --install build --config Release

dart_pub_get mode="default":
    cd frb_dart && dart pub get
    cd {{dir_example_pure_dart}}/dart && dart pub get
    cd {{dir_example_pure_dart_multi}}/dart && dart pub get
    cd {{dir_example_with_flutter}} && {{ if mode == "dart_only" { "pwd" } else { "flutter pub get" } }}

# ============================ build & test ============================

rust_build_and_test:
    just install_expand
    just _rust_build_and_test_single frb_codegen --features uuid,chrono
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

dart_test_web_unit:
    cd frb_dart && dart pub get
    cd frb_dart && dart test test/*.dart
    cd frb_dart && dart test -p chrome test/*.dart

dart_test_web_integration features="":
    just dart_pub_get dart_only
    cd {{dir_example_pure_dart}}/dart && dart run \
      ../../../frb_dart/bin/serve.dart \
      -c ../rust --dart-input lib/main.web.dart --root web/ --run-tests \
      --features={{features}}

dart_test_vm_service:
    cd frb_example/pure_dart/rust && cargo build --verbose
    dart run --enable-vm-service --define=ENABLE_FRB_FFI_TEST_TOOL=true \
      frb_example/pure_dart/dart/lib/main_with_vm_service.dart \
      target/debug/libflutter_rust_bridge_example_pure_dart.so

dart_test_valgrind name:
    just _dart_test_raw {{name}} "PYTHONUNBUFFERED=1 ./valgrind_util.py"

dart_test_simple name:
    just _dart_test_raw {{name}} ""

_dart_test_raw name script_prefix:
    cd frb_example/{{name}}/rust && cargo build --verbose
    # need to be AOT, since prod environment is AOT, and JIT+valgrind will have strange problems
    cd frb_example/{{name}}/dart && dart compile exe bin/{{name}}.dart -o main.exe
    cd frb_example/{{name}}/dart && \
        {{script_prefix}} ./main.exe \
        "../../../target/debug/libflutter_rust_bridge_example_{{name}}.{{library_file_ext}}" --chain-stack-traces

flutter_example_with_flutter_integration_test:
    flutter config --enable-{{ os() }}-desktop
    cd {{dir_example_with_flutter}} && flutter pub get
    cd {{dir_example_with_flutter}} && flutter test -d {{ os() }} integration_test/main.dart --verbose

# ============================ code generators ============================

generate_all: generate_ffigen generate_bridge generate_book_help

generate_book_help:
    cargo run --manifest-path frb_codegen/Cargo.toml -- --help > book/src/help.txt
    dart run frb_dart/bin/serve.dart --help > book/src/help.serve.txt

generate_ffigen:
    cd frb_dart && dart run ffigen

cargo_run_codegen := if env_var_or_default("FRB_TEST_USE_RELEASE_VERSION", "false") == "true" {
    "flutter_rust_bridge_codegen"
} else {
    "cargo run --manifest-path frb_codegen/Cargo.toml --package flutter_rust_bridge_codegen --bin flutter_rust_bridge_codegen --features 'chrono,uuid' -- "
}

generate_bridge:
    just _generate_bridge_pure_dart
    just _generate_bridge_pure_dart_multi
    just _generate_bridge_with_flutter

_generate_bridge_pure_dart:
    {{cargo_run_codegen}} frb_example/pure_dart/rust/.flutter_rust_bridge.yml

_generate_bridge_pure_dart_multi:
    {{cargo_run_codegen}} \
        --rust-input frb_example/pure_dart_multi/rust/src/api_1.rs frb_example/pure_dart_multi/rust/src/api_2.rs \
        --dart-output frb_example/pure_dart_multi/dart/lib/bridge_generated_api_1.dart frb_example/pure_dart_multi/dart/lib/bridge_generated_api_2.dart \
        --dart-format-line-length 120 \
        --dart-enums-style \
        --rust-output frb_example/pure_dart_multi/rust/src/generated_api_1.rs frb_example/pure_dart_multi/rust/src/generated_api_2.rs \
        --class-name ApiClass1 ApiClass2 \
        --wasm

_generate_bridge_with_flutter:
    {{cargo_run_codegen}} \
        --rust-input frb_example/with_flutter/rust/src/api.rs \
        --dart-output frb_example/with_flutter/lib/bridge_generated.dart \
        --c-output frb_example/with_flutter/ios/Runner/bridge_generated.h \
        --dart-decl-output frb_example/with_flutter/lib/bridge_definitions.dart \
        --dart-format-line-length 120 \
        --dart-enums-style \
        --no-use-bridge-in-method \
        --wasm

# ============================ linters ============================

rust_linter:
    just _rust_linter_main
    just _rust_linter_wasm

_rust_linter_main:
    cargo fmt
    # cargo clippy -- -D warnings # temp disable

_rust_linter_wasm:
    rustup target add wasm32-unknown-unknown
    cd frb_rust && cargo clippy --target wasm32-unknown-unknown -- -D warnings

default_line_length := "120"

dart_linter mode="default":
    just dart_pub_get

    just _dart_linter_single {{mode}} frb_dart dart 80
    just _dart_linter_single {{mode}} {{dir_example_pure_dart}}/dart dart {{default_line_length}}
    just _dart_linter_single {{mode}} {{dir_example_pure_dart_multi}}/dart dart {{default_line_length}}
    just _dart_linter_single {{mode}} {{dir_example_with_flutter}} dart {{default_line_length}}

    just dart_linter_pana

_dart_linter_single mode directory executable line_length:
    cd {{directory}} && dart format \
      --line-length {{line_length}} \
      {{ if mode == "fix" { "--fix" } else { "--output=none --set-exit-if-changed" } }} \
      .
    # cd {{directory}} && {{executable}} analyze --fatal-infos

dart_linter_pana:
    flutter pub global activate pana
    cd frb_dart && {{pana}} --no-warning --line-length 80 --exit-code-threshold 0

dart_check_included_source:
    #!/usr/bin/env bash
    set -euxo pipefail

    git clone --depth 1 --filter=blob:none --sparse --branch stable https://github.com/dart-lang/sdk.git
    cd sdk
    git sparse-checkout set runtime/include
    cd ..
    cp -rf ./sdk/runtime/include/* ./frb_rust/src/dart_api/
    rm -r sdk
    git diff --exit-code

# ============================ (some of) CI ============================

ci_dart_valgrind:
    just install_ffigen_dependency
    just install_valgrind
    just dart_pub_get dart_only
    just dart_test_valgrind pure_dart

ci_dart_simple:
    just install_ffigen_dependency
    just dart_pub_get dart_only
    just dart_test_simple pure_dart
    just dart_test_simple pure_dart_multi

ci_codegen:
    just install_ffigen_dependency
    just dart_pub_get
    just dart_check_included_source
    just install_expand
    just generate_all
    just check_no_git_diff

# ============================ misc ============================

clean:
    cd frb_dart && flutter clean
    cd {{dir_example_pure_dart}}/dart && flutter clean
    cd {{dir_example_pure_dart_multi}}/dart && flutter clean
    cd {{dir_example_with_flutter}} && flutter clean
    cargo clean

check_no_git_diff:
    # Check nothing has changed (Use `just precommit` if error occurred)
    git diff --exit-code

normalize_pubspec_lock:
    just _normalize_pubspec_lock_one frb_example/pure_dart/dart/pubspec.lock
    just _normalize_pubspec_lock_one frb_example/pure_dart_multi/dart/pubspec.lock
    just _normalize_pubspec_lock_one frb_example/with_flutter/pubspec.lock

_normalize_pubspec_lock_one path:
    {{sed}} -e 's/pub.flutter-io.cn/pub.dev/g' {{path}}

serve *args:
    cd {{invocation_directory()}} && dart run {{justfile_directory()}}/frb_dart/bin/serve.dart {{args}}

use_flutter_rust_bridge_release:
    cp ./frb_example/pure_dart/dart/pubspec.yaml.release ./frb_example/pure_dart/dart/pubspec.yaml
    cp ./frb_example/pure_dart/rust/Cargo.toml.release ./frb_example/pure_dart/rust/Cargo.toml
    cp ./frb_example/with_flutter/pubspec.yaml.release ./frb_example/with_flutter/pubspec.yaml
    cp ./frb_example/with_flutter/rust/Cargo.toml.release ./frb_example/with_flutter/rust/Cargo.toml

configure_ndk:
    #!/usr/bin/env bash
    set -euxo pipefail

    if [ "$(uname)" == "Darwin" ]; then
        # Do something under Mac OS X platform
        ANDROID_HOME=$HOME/Library/Android/sdk
        SDKMANAGER=$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager
        echo y | $SDKMANAGER "ndk;21.4.7075529"
        ln -sfn $ANDROID_HOME/ndk/21.4.7075529 $ANDROID_HOME/ndk-bundle
    elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
        # Do something under GNU/Linux platform
        ANDROID_ROOT=/usr/local/lib/android
        ANDROID_SDK_ROOT=${ANDROID_ROOT}/sdk
        SDKMANAGER=${ANDROID_SDK_ROOT}/cmdline-tools/latest/bin/sdkmanager
        echo "y" | $SDKMANAGER "ndk;21.4.7075529"
        ANDROID_NDK_ROOT=${ANDROID_SDK_ROOT}/ndk-bundle
        ln -sfn $ANDROID_SDK_ROOT/ndk/21.4.7075529 $ANDROID_NDK_ROOT
    fi
# ============================ precommit ============================

precommit: dart_pub_get generate_all rust_linter dart_linter normalize_pubspec_lock

# ============================ releasing new versions ============================

release old_version new_version:
    just _release_sanity_check_version {{old_version}} {{new_version}}
    just _release_update_version {{old_version}} {{new_version}}
    just precommit
    just _release_update_scoop
    just _release_update_git {{old_version}} {{new_version}}
    just _release_update_github {{old_version}} {{new_version}}
    just _release_publish_all

_release_sanity_check_version old_version new_version:
    grep -q 'version = "{{old_version}}"' Cargo.toml
    grep -q '{{new_version}}' CHANGELOG.md

_release_update_version old_version new_version:
    {{sed}} 's/version = "{{old_version}}"/version = "{{new_version}}"/g' Cargo.toml
    {{sed}} 's/version: {{old_version}}/version: {{new_version}}/g' frb_dart/pubspec.yaml

_release_update_scoop:
    cd frb_codegen && ./contrib/scoop.json.sh > ./contrib/flutter_rust_bridge_codegen.json

_release_update_git old_version new_version:
    git add --all
    git status && git diff --staged | grep ''
    git commit -m "bump from {{old_version}} to {{new_version}}"
    git push

_release_update_github old_version new_version:
    awk '/## {{new_version}}/{flag=1; next} /## {{old_version}}/{flag=0} flag' CHANGELOG.md | gh release create v{{new_version}} --notes-file "-" --draft --title v{{new_version}}
    echo 'A *DRAFT* release has been created. Please go to the webpage and really release if you find it correct.'
    open https://github.com/fzyzcjy/flutter_rust_bridge/releases

_release_publish_all:
    (cd frb_codegen && cargo publish)
    (cd frb_rust && cargo publish)
    (cd frb_macros && cargo publish)
    (cd frb_dart && flutter pub publish --force --server=https://pub.dartlang.org)

install_ndk:
    just _install_crate cargo-ndk

install_lipo:
    just _install_crate cargo-lipo

install_expand:
    just _install_crate cargo-expand

_install_crate name="cargo-lipo":
    #!/usr/bin/env bash
    set -euxo pipefail

    PACKAGE_NAME={{name}}
    echo $PACKAGE_NAME
    VERSION=$(cargo search $PACKAGE_NAME | grep "$PACKAGE_NAME" | cut -d '"' -f 2)
    echo $VERSION

    if ! [ -x "$(command -v $PACKAGE_NAME)" ]; then
      echo "$PACKAGE_NAME not found. Installing version $VERSION ..."
      cargo install $PACKAGE_NAME --version $VERSION
    elif ! $PACKAGE_NAME --version | grep -q $VERSION; then
      echo "Updating $PACKAGE_NAME to version $VERSION ..."
      cargo install $PACKAGE_NAME --version $VERSION --force
    else
      echo "Already installed the correct version of $PACKAGE_NAME."
    fi

_noop:
    echo "this is noop"

# ============================ to be migrated ============================

# TODO - @Desdaemon
#dylib := if os() == "windows" {
#    "flutter_rust_bridge_example.dll"
#} else if os() == "macos" {
#    "libflutter_rust_bridge_example.dylib"
#} else {
#    "libflutter_rust_bridge_example.so"
#}
#path_relative_linux_so := "target/x86_64-unknown-linux-gnu/debug/libflutter_rust_bridge_example.so"
#dir_tools := justfile_directory() / "tools"
#test: test-support test-pure test-integration
#test-pure:
#    cd {{dir_example_pure_dart}}/rust && cargo b
#    cd {{dir_example_pure_dart}}/dart && \
#        dart pub get && \
#        dart lib/main.dart ../../../target/debug/{{dylib}}
## TODO: Make ASan tests work for other platforms
#test-pure-asan $RUSTFLAGS="-Zsanitizer=address":
#    ./tools/dartsdk/fetch.sh
#    cd {{dir_example_pure_dart}}/rust && cargo +nightly b --target x86_64-unknown-linux-gnu
#    cd {{dir_example_pure_dart}}/dart && \
#        {{dir_tools}}/dartsdk/x64/dart pub get && \
#        {{dir_tools}}/dartsdk/x64/dart lib/main.dart  ../../../{{path_relative_linux_so}}
#test-pure-web *args:
#    cd {{dir_example_pure_dart}}/dart && just serve --dart-input lib/main.web.dart --root web/ -c ../rust --port 8081 {{args}}
#test-flutter-web *args:
#    cd {{dir_example_with_flutter}} && just serve -c rust {{args}}
#test-integration:
#    cd {{dir_example_with_flutter}} && flutter test integration_test/main.dart
#test-support platform="chrome":
#    cd frb_dart && dart pub get && \
#        dart test test/*.dart && \
#        dart test -p {{platform}} test/*.dart

