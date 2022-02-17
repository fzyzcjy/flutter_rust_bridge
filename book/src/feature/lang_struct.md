# `struct`s

You can even use recursive fields. For example: `pub struct TreeNode { pub value: String, pub children: Vec<MyTreeNode>, pub parent: Box<MyTreeNode> }`.

Tuple structs `struct Foo(A, B)` are translated as `class Foo { A field0; B field1; }`, since Dart does not have anonymous fields.