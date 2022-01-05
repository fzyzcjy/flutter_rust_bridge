# Feature details

## What this library is & isn't

This library is nothing but a code generator that helps your Flutter/Dart
functions call Rust functions. Therefore, you may refer to external materials to
learn Flutter, learn Rust, learn
[Flutter FFI](https://flutter.dev/docs/development/platform-integration/c-interop)
(Dart FFI) and so on. With material on the Internet, you will know how to create
a mobile application using Flutter, and how that app can call Rust functions via
Dart FFI (in the C ABI). Then this package comes in, and ease you from the
burden to write down tons of boilerplate code ;)

## Language features this library supports

Here is a brief glance showing what the code generator can generate (non-exhaustive):

| Rust type         | Dart type        |
| ----------------- | ---------------- |
| `i8, u8, ..`      | `int`            |
| `f32, f64`        | `double`         |
| `bool`            | `bool`           |
| `String`          | `String`         |
| `()`              | `void`           |
| `Vec<i8, u8, ..>` | `..List`         |
| `Vec<T>`          | `List<T>`        |
| `struct`          | `class`          |
| `enum { A, B }`   | `enum`           |
| `enum { A(..) }`  | `@freezed class` |
| `Option<T>`       | `T?`             |
| `Box<T>`          | `T`              |

Here are other functionalities:

- Tuple structs `struct Foo(A, B)` are translated as
  `class Foo { A field0; B field1; }`
- Doc-comments on Rust items are copied over (if there are missing comments
  please open an issue!)
- `Result::Err` and panics become thrown exceptions
- `fn(StreamSink<T>, ..) -> Result<()>` returns a Dart `Stream<T>`
- `ZeroCopyBuffer<Vec<i8, u8, ..>>` sends the buffer to Dart without making
  copies
- `SyncReturn<Vec<u8>>` sends the byte buffer to Dart synchronously

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
