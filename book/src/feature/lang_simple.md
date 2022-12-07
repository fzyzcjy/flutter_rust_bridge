# Simple correspondence

Here is a brief glance showing what the code generator can generate (non-exhaustive). Some rows have hyper-links pointing to more detailed explanations.

| Rust                                              | Dart                        |
| ------------------------------------------------- | --------------------------- |
| [`Vec<u8>`, `Vec<i8>`..](lang_vec.md)             | `Uint8List`, `Int8List`, .. |
| [`Vec<T>`](lang_vec.md)                           | `List<T>`                   |
| [`[T; N]`](lang_vec.md)                           | `List<T>`                   |
| [`struct { .. }`, `struct( .. )`](lang_struct.md) | `class`                     |
| [`enum { A, B }`](lang_enum.md)                   | `enum`                      |
| [`enum { A(..) }`](lang_enum.md)                  | `@freezed class`            |
| [`impl Trait`](lang_type_impl_trait.md)           | `@freezed class`            |
| [`use ...`](lang_external.md)                     | act normally                |
| [`Option<T>`](lang_option.md)                     | `T?`                        |
| [Arbitrary types (opaque)](lang_opaque.md)        | `Opaque`                    |
| `Box<T>`                                          | `T`                         |
| comments                                          | same                        |
| `Result::Err`, panic                              | `throw Exception`           |
| `i8`, `u8`, .., `usize`                           | `int`                       |
| `f32`, `f64`                                      | `double`                    |
| `bool`                                            | `bool`                      |
| `String`                                          | `String`                    |
| `()`                                              | `void`                      |

Types from `chrono` crate are supported as a feature, see [here](lang_chrono.md).
Types from `uuid` crate are supported as a feature, see [here](lang_uuid.md).
