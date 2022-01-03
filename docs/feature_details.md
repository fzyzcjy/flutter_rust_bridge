# Feature details

## What this library is & isn't

This library is nothing but a code generator that helps your Flutter/Dart functions call Rust functions. Therefore, you may refer to external materials to learn Flutter, learn Rust, learn [Flutter FFI](https://flutter.dev/docs/development/platform-integration/c-interop) (Dart FFI) and so on. With material on the Internet, you will know how to create a mobile application using Flutter, and how that app can call Rust functions via Dart FFI (in the C ABI). Then this package comes in, and ease you from the burden to write down tons of boilerplate code ;)

## Command line arguments

Simply add `--help` to see full documentation.

```shell
flutter_rust_bridge_codegen

USAGE:
    flutter_rust_bridge_codegen [FLAGS] [OPTIONS] --dart-output <dart-output> --rust-input <rust-input>

FLAGS:
        --skip-add-mod-to-lib    Skip automatically adding `mod bridge_generated;` to `lib.rs`
    -h, --help                   Prints help information
    -V, --version                Prints version information

OPTIONS:
    -r, --rust-input <rust-input>                              Path of input Rust code
    -d, --dart-output <dart-output>                            Path of output generated Dart code
    -c, --c-output <c-output>                                  Path of output generated C header
        --rust-crate-dir <rust-crate-dir>                      Crate directory for your Rust project
        --rust-output <rust-output>                            Path of output generated Rust code
        --class-name <class-name>                              Generated class name
        --dart-format-line-length <dart-format-line-length>    Line length for dart formatting
        --llvm-path <llvm-path>                                Path to the installed LLVM
```

## Language features this library supports

Here is a list of types that the code generator can generate:

[WIP] You can read the `pure_dart` example code currently, before I put the full list here.

Here are other functionalities:

[WIP] (e.g. [Stream is supported but not documented yet](https://github.com/fzyzcjy/flutter_rust_bridge/issues/179)).

