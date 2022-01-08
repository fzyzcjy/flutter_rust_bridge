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

Here is a brief glance showing what the code generator can generate
(non-exhaustive):

| Rust type         | Dart type                |
| ----------------- | ------------------------ |
| `i8, u8, ..`      | `int`                    |
| `f32, f64`        | `double`                 |
| `bool`            | `bool`                   |
| `String`          | `String`                 |
| `()`              | `void`                   |
| `Vec<i8, u8, ..>` | `..List`                 |
| `Vec<T>`          | `List<T>`                |
| `struct`          | `class`                  |
| `enum { A, B }`   | `enum`                   |
| `enum { A(..) }`  | `@freezed class`         |
| `Option<T>`       | `T?`                     |
| `Box<T>`          | `T`                      |
| `Opaque<T>`       | Generated opaque pointer |

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

### `Opaque` pointers

Opaque pointers allow Dart to _own_ pointers to some data in memory. Rust
functions can either return opaque pointers to transfer ownership of the data,
or accept them as parameters to extract the pointer's contents.

`Opaque<T>` is valid for any T, with the only limitation being that T implements
the `DartSafe` marker trait, which itself requires that T implements:

- `Send` and `Sync`: Dart and Rust functions run in separate threads, therefore
  types returned from Rust must also be safe to send to and share with Dart.
- `UnwindSafe`: This library uses `catch_unwind` to convert Rust panics to
  proper Dart exceptions, so the return type must also be safe to unwind.
- `RefUnwindSafe`: Similar to above, but for shared references. This trait
  disqualifies all interior mutability types, such as Cell, RefCell, etc., and
  types that depend on UnsafeCell.

`DartSafe` is automatically implemented for types that implement all of these
traits.

This requirement has one consequence: trait objects e.g. `Box<dyn Trait>` must
also implement `DartSafe` to be compatible with Opaque. Here is a common way to
define opaque pointers to trait objects:

```rust
// first define a trait that exhibits the desired traits
pub trait MyTrait: DartSafe + Clone + .. {}
// auto implement the trait for qualifying types
impl<T: DartSafe + Clone + ..> MyTrait for T {}
// use the trait
pub struct MyWrapper(pub Opaque<Box<dyn MyTrait>>);
```

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
