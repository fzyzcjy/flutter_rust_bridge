# Simple correspondence

Here is a brief glance showing what the code generator can generate (non-exhaustive). Some rows have hyper-links
pointing to more detailed explanations.

| Rust                                                       | Dart                                                      |
|------------------------------------------------------------|-----------------------------------------------------------|
| [`Vec<u8>`..`Vec<u64>`](detailed/vec)                      | `Uint8List`..`Uint64List`                                 |
| [`Vec<i8>`..`Vec<i64>`](detailed/vec)                      | `Int8List`..`Int64List`                                   |
| [`Vec<f32>`, `Vec<f64>`](detailed/vec)                     | `Float32List`, `Float64List`                              |
| [`Vec<T>`](detailed/vec)                                   | `List<T>`                                                 |
| [`HashMap<K, V>`](detailed/map_set)                        | `Map<K, V>`                                               |
| [`HashSet<T>`](detailed/map_set)                           | `Set<T>`                                                  |
| [`[T; N]`](detailed/vec)                                   | `List<T>`                                                 |
| [`struct { .. }`, `struct( .. )`](detailed/struct)         | `class`                                                   |
| [`enum { A, B }`](detailed/enum)                           | `enum`                                                    |
| [`enum { A(..) }`](detailed/enum)                          | `@freezed sealed class`                                   |
| [`use ...`](external/same-crate)                           | act normally                                              |
| [`Option<T>`](detailed/option)                             | `T?`                                                      |
| [Auto arbitrary Rust types](../arbitrary/rust-auto-opaque) | act normally                                              |
| [Arbitrary Rust types (opaque)](../arbitrary/rust-opaque)  | `RustOpaque`                                              |
| `DartOpaque`                                               | [Arbitrary Dart types (opaque)](../arbitrary/dart-opaque) |
| `DartDynamic`                                              | [dynamic](../arbitrary/dart-dynamic)                      |
| [`Result::Err`, panic](return)                             | `throw Exception`                                         |
| `Box<T>`                                                   | `T`                                                       |
| comments                                                   | same                                                      |
| `i8`, `u8`, .., `usize`, `i128`, `u128`                    | [`int`, `BigInt`](detailed/primitive)                     |
| `f32`, `f64`                                               | `double`                                                  |
| `bool`                                                     | `bool`                                                    |
| `char`, `String`                                           | `String`                                                  |
| `()`                                                       | `void`                                                    |
| `type A = B`                                               | [type alias](detailed/alias)                              |
| [`(T, U, ..)`](detailed/tuple)                             | [`(T, U, ..)`](https://dart.dev/language/records)         |

Types from `chrono` crate are supported as a feature, see [here](detailed/chrono).
Types from `uuid` crate are supported as a feature, see [here](detailed/uuid).

Raw strings are supported for struct field names. For example, you can have `struct S { r#type: i32 }`. In dart,
the `r#` prefix will be correctly removed. They are not yet supported for function arguments.
