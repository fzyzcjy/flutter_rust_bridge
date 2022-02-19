# Zero copy

`ZeroCopyBuffer<Vec<u8>>` (and its friends like `ZeroCopyBuffer<Vec<i8>>`) sends the data from Rust to Dart without making copies. Thus, you save the time of copying data, which can be large if your data is big (such as a high-resolution image).

## Example

```rust,noplayground
pub fn draw_tree(tree: Vec<TreeNode>) -> ZeroCopyBuffer<Vec<u8>> { ... }
```

Becomes:

```Dart
Future<Uint8List> drawTree({required List<TreeNode> tree});
```

The generated Dart code looks exactly the same as the case without `ZeroCopyBuffer`. However, the internal implementation changes and there is no memory copy at all!

Remark: If you are curious about `Future`, have a look at [this](async_dart.md).