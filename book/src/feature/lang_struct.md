# `struct`s

You can even use recursive fields. For example: `pub struct TreeNode { pub value: String, pub children: Vec<MyTreeNode>, pub parent: Box<MyTreeNode> }`.

Tuple structs `struct Foo(A, B)` are translated as `class Foo { A field0; B field1; }`, since Dart does not have anonymous fields.

## Example

```rust,noplayground
pub struct MyTreeNode {
    pub value: Vec<u8>,
    pub children: Vec<MyTreeNode>,
}
```

Becomes:

```Dart
class MyTreeNode {
  final Uint8List value;
  final List<MyTreeNode> children;
  MyTreeNode({required this.value, required this.children});
}
```

Remark: If you are curious about `Future`, have a look at [this](async_dart.md).

