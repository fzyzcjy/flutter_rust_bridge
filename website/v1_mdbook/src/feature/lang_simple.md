# Simple correspondence

Here is a brief glance showing what the code generator can generate (non-exhaustive). Some rows have hyper-links pointing to more detailed explanations.

| Rust                                                 | Dart                                                 |
| ---------------------------------------------------- | ---------------------------------------------------- |
| [`Vec<u8>`..`Vec<u64>`](lang_vec.md)                 | `Uint8List`..`Uint64List`                            |
| [`Vec<i8>`..`Vec<i64>`](lang_vec.md)                 | `Int8List`..`Int64List`                              |
| [`Vec<f32>`, `Vec<f64>`](lang_vec.md)                | `Float32List`, `Float64List`                         |
| [`Vec<T>`](lang_vec.md)                              | `List<T>`                                            |
| [`[T; N]`](lang_vec.md)                              | `List<T>`                                            |
| [`struct { .. }`, `struct( .. )`](lang_struct.md)    | `class`                                              |
| [`enum { A, B }`](lang_enum.md)                      | `enum`                                               |
| [`enum { A(..) }`](lang_enum.md)                     | `@freezed sealed class`                              |
| [`use ...`](lang_external.md)                        | act normally                                         |
| [`Option<T>`](lang_option.md)                        | `T?`                                                 |
| [Arbitrary Rust types (opaque)](lang_rust_opaque.md) | `RustOpaque`                                         |
| `DartOpaque`                                         | [Arbitrary Dart types (opaque)](lang_dart_opaque.md) |
| [`Result::Err`, panic](lang_exceptions.md)           | `throw Exception`                                    |
| `Box<T>`                                             | `T`                                                  |
| comments                                             | same                                                 |
| `i8`, `u8`, .., `usize`                              | `int`                                                |
| `f32`, `f64`                                         | `double`                                             |
| `bool`                                               | `bool`                                               |
| `String`                                             | `String`                                             |
| `()`                                                 | `void`                                               |
| `type A = B`                                         | [type alias](lang_type_alias.md)                     |
| [`(T, U, ..)`](lang_tuple.md)                        | [`(T, U, ..)`](https://dart.dev/language/records)    |

Types from `chrono` crate are supported as a feature, see [here](lang_chrono.md).
Types from `uuid` crate are supported as a feature, see [here](lang_uuid.md).

Raw strings are supported for struct field names. For example, you can have `struct S { r#type: i32 }`. In dart, the `r#` prefix will be correctly removed. They are not yet supported for function arguments.
