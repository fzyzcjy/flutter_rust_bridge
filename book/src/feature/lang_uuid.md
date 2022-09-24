# uuid

Codegen optionally support [uuid crate](https://docs.rs/uuid) with feature `uuid`.

| 🦀 Rust     | 🎯 Dart                                                         |
| ----------- | -----------                                                    |
| `Uuid`      | `UuidValue` see package [uuid](https://pub.dev/packages/uuid)  |

⚠️ Please note that you need to add package [uuid](https://pub.dev/packages/uuid/install) to your Dart/Flutter dependencies in `pubspec.yaml` as well.

💡 `Vec<Uuid>` implementation detail : all the uuids get concatenated as a single array of bytes for performance optimization.
