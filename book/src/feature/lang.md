# Language translations

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

`*` indicates that there are some extra explanations, and will be shown in the sub-sections below.