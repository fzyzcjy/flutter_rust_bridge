# Vec and array

## `Vec<u8>`, `Vec<i8>`, ...

In Dart, when you want to express a long byte array such as a big image or some binary blob, people normally use `Uint8List` instead of `List<int>` since the former is much performant. `flutter_rust_bridge` takes this into consideration for you. When you have `Vec<u8>` (or `Vec<i8>`, or `Vec<i32>`, etc), it will be translated into `Uint8List` or its friends.

[This section](lang_simple.md) provides more details about `Vec<u8>` and its friends.

## `Vec<T>`

When you have normal `Vec<T>` for `T` types other than `u8`, `i8` etc, it will be converted to normal `List<T>`.

Remark: `Vec<Box<T>>` is not supported yet though fixable ([#1072](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1072)), but according to [clippy lints](https://rust-lang.github.io/rust-clippy/master/index.html#vec_box), it is usually better to use `Vec<T>` directly.

## `[T; N]`

Since Dart does not have special treatment for static-sized arrays, it is converted to `List<T>` as well.

## Example

```rust,noplayground
pub fn draw_tree(tree: Vec<TreeNode>) -> Vec<u8> { ... }
```

Becomes:

```Dart
Future<Uint8List> drawTree({required List<TreeNode> tree});
```

Remark: If you are curious about `Future`, have a look at [this](async_dart.md).
