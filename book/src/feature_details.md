# Feature details

## What this library is & isn't

This library is nothing but a code generator that helps your Flutter/Dart functions call Rust functions. It only generates some boilerplate code that you will manually write down otherwise. Moreover, we have provided detailed tutorials for you to play with examples, set up brand new apps, and integrate with existing apps.

Of course, you may still need to have some basic familiarity with Flutter/Dart, Rust, and its [ffi](https://flutter.dev/docs/development/platform-integration/c-interop).

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
- Can separate generated definitions from implementations (see #298)
- Imported symbols can be used normally, such as `use crate::data::{MyEnum, MyStruct};`

## Customize handler's behavior

By default, the `DefaultHandler` is used. You can implement your own `Handler` doing whatever you want. In order to do this, create a variable named `FLUTTER_RUST_BRIDGE_HANDLER` in the Rust input file (probably using `lazy_static`). You may not need to create a brand new struct implementing `Handler`, but instead, use the `SimpleHandler` and customize its generic arguments such as its `Executor`.

## Setup/init FFI call

If you want that feature, have a look at `FlutterRustBridgeSetupMixin` in the Dart side.

## Async in Rust

If you want to use async/await or return a Future type from your Rust functions, please refer to [this documentation](article/async_in_rust.md) for a detailed guide.

