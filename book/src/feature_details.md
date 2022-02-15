# Feature details

## Prelogue: What this library is and is not

This library is nothing but a code generator that helps your Flutter/Dart functions call Rust functions. It only generates some boilerplate code that you will manually write down otherwise. Moreover, we have provided detailed tutorials for you to play with examples, set up brand new apps, and integrate with existing apps.

Of course, you may still need to have some basic familiarity with Flutter/Dart, Rust, and its [ffi](https://flutter.dev/docs/development/platform-integration/c-interop).

## Prelogue: Full examples

If you want to look at a lot of examples - I have to warn you, really too many - have a look at [pure_dart's api.rs](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/src/api.rs). It contains all tests for this library.

## Language translations

Here is a brief glance showing what the code generator can generate (non-exhaustive):

| Rust type                         | Dart type                   |
| --------------------------------- | --------------------------- |
| `Vec<u8>`, `Vec<i8>`.. *          | `Uint8List`, `Int8List`, .. |
| `Vec<T>` *                        | `List<T>`                   |
| `struct { .. }`, `struct( .. )` * | `class`                     |
| `enum { A, B }` *                 | `enum`                      |
| `enum { A(..) }` *                | `@freezed class`            |
| `use ...` *                       | act normally                |
| `Option<T>`                       | `T?`                        |
| `Box<T>`                          | `T`                         |
| comments                          | same                        |
| `Result::Err`                     | `throw Exception`           |
| panic                             | `throw Exception`           |
| `i8, u8, ..`                      | `int`                       |
| `f32, f64`                        | `double`                    |
| `bool`                            | `bool`                      |
| `String`                          | `String`                    |
| `()`                              | `void`                      |

`*` indicates that there are some extra explanations below.

### `Vec<u8>` and `Vec<T>`

In Dart, when you want to express a long byte array such as a big image or some binary blob, people normally use `Uint8List` instead of `List<int>` since the former is much performant. `flutter_rust_bridge` takes this into consideration for you. When you have `Vec<u8>` (or `Vec<i8>`, or `Vec<i32>`, etc), it will be translated it into `Uint8List` or its friends; but when you have normal `Vec<T>` for other `T` types, it will be normal `List<T>`.

### `struct`s

You can even use recursive fields. For example: `pub struct TreeNode { pub value: String, pub children: Vec<MyTreeNode>, pub parent: Box<MyTreeNode> }`.

Tuple structs `struct Foo(A, B)` are translated as `class Foo { A field0; B field1; }`, since Dart does not have anonymous fields.

### `enum`s

Rust's `enum` are known to be very expressive and powerful - it allows each enum variant to have different associated data. Dart does not have such things in built-in enums, but no worries - we will automatically translate it into the equivalent using the `freezed` Dart library.

### `use`

Imported symbols can be used normally. For example, with `use crate::data::{MyEnum, MyStruct};`, you can use `MyEnum` or `MyStruct` in your code normally.

## `ZeroCopyBuffer`s

`ZeroCopyBuffer<Vec<u8>>` (and its friends like `ZeroCopyBuffer<Vec<i8>>`) sends the data from Rust to Dart without making copies. Thus, you save the time of copying data, which can be large if your data is big (such as a high-resolution image).

## `Stream`s (call once, return multiple times; like `Iterator`s)

Flutter's [Stream](https://dart.dev/tutorials/language/streams) is a powerful abstraction. When using it as the return value of Rust function, we can allow the scenario that we call function once, and then return multiple times.

For example, your Rust function may run computationally heavy algorithms, and for every hundreds of milliseconds, it finds out a new piece of the full solution. In this case, it can immediately give that piece to Flutter, then Flutter can render it to UI immediately. Therefore, users do not need to wait for the full algorithm to finish before he can see some partial results on the user interface.

As for the details, a Rust function with signature like `fn f(sink: StreamSink<T>, ..) -> Result<()>` is translated to a Dart function  `Stream<T> f(..)`.

## Asynchronous in Dart

This library generates functions that are *asynchronous* in Dart by default. So you will see `fn f(..) -> String` becomes `Future<String> f(..)` with that interesting `Future`.

Why? Flutter UI is single-threaded. If you use the intuitive synchronous approach, just like what you will (have to) do with plain-old Flutter bindings, your UI will be *stuck* as long as your Rust code is executing. If your Rust code run for 100ms for a heavy computation, your UI will fully freeze for 100ms and the users will not be happy.

On the other hand, with the generated asynchronous bindings in Dart, you can simply call functions directly in main isolate (thread) of Dart/Flutter, and Rust code will not block the Flutter UI.

Remark: A common mistake is to call Rust code in *another* Dart isolate (i.e. "thread") instead of the main isolate. That is completely not needed, and will only make your life harder. As is described above, even if your Rust code computes for 100ms, the async call will only take, say, 0.1ms, and will not block your UI.

## Synchronous in Dart

If you really need to generate synchronous functions in Dart, you can ues the `SyncReturn<Vec<u8>>` as the return type.

We suggest only do this for very quick Rust functions, or the Dart UI will be blocked.

## Concurrency

Multiple Rust functions can be running at the same time, and they will be running concurrently. This is because by default we use a thread pool to execute the Rust functions. However, you can fully customize this behavior (and even throw away the thread pool).

## Customize handler's behavior

By default, the `DefaultHandler` is used. You can implement your own `Handler` doing whatever you want. In order to do this, create a variable named `FLUTTER_RUST_BRIDGE_HANDLER` in the Rust input file (probably using `lazy_static`). You may not need to create a brand new struct implementing `Handler`, but instead, use the `SimpleHandler` and customize its generic arguments such as its `Executor`.

For example, you may want to add logging logic. Or, you may want to report the captured errors to your APM service instead of printing it.

## Setup or initialization

If you want that feature, have a look at `FlutterRustBridgeSetupMixin` in the Dart side. (More documentaions to be added; you can create an issue if you have questions now)

## Async in Rust

To use async/await or return a Future type from your Rust functions, please refer to [this documentation](article/async_in_rust.md). If you have interest in more integrated generator, please create and issue.

## Miscellaneous

* You can separate generated definitions from implementations (see #298)
