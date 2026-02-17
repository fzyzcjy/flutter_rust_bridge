# Zero copy

`ZeroCopyBuffer<Vec<u8>>` (and its friends like `ZeroCopyBuffer<Vec<i8>>`) sends the data from Rust to Dart without making copies[^1]. Thus, you save the time of copying data, which can be large if your data is big (such as a high-resolution image).

## Make it the default

If you don't want to wrap `Vec<u8>` and its friends with `ZeroCopyBuffer` over and over again to avoid copying memory, you can alternatively provide a cargo feature called `zero-copy`. With this feature enabled, `Vec<u8>` and its friends will be zero-copied by default.

```toml
# Cargo.toml
[dependencies]
flutter_rust_bridge = { version = "...", features = ["zero-copy"] }
```

## Example

```rust,noplayground
pub fn draw_tree(tree: Vec<TreeNode>) -> ZeroCopyBuffer<Vec<u8>> { ... }
```

Becomes:

```Dart
Future<Uint8List> drawTree({required List<TreeNode> tree});
```

The generated Dart code looks exactly the same as the case without `ZeroCopyBuffer`. However, the internal implementation changes and now there is no memory copy at all!

If you are curious about what those `Vec<u8>` and its friends actually are, take a look at [this](lang_simple.md).

[^1]: Not currently supported on Web, and will fallback to copying the buffer.
