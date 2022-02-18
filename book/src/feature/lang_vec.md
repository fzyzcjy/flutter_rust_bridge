# `Vec<u8>` and `Vec<T>`

In Dart, when you want to express a long byte array such as a big image or some binary blob, people normally use `Uint8List` instead of `List<int>` since the former is much performant. `flutter_rust_bridge` takes this into consideration for you. When you have `Vec<u8>` (or `Vec<i8>`, or `Vec<i32>`, etc), it will be translated it into `Uint8List` or its friends; but when you have normal `Vec<T>` for other `T` types, it will be normal `List<T>`.

## Example

```rust,noplayground
pub fn draw_tree(tree: Vec<TreeNode>) -> Vec<u8> { ... }
```

Becomes:

```Dart
Future<Uint8List> drawTree({required List<TreeNode> tree});
```

Remark: If you are curious about `Future`, have a look at [this](async_dart.md).

