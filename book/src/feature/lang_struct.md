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

# Dart metadata annotations

You can add dart metadata annotations using `dart_metadata` parameter in `frb` macro.

* For annotations that are prelude by dart (eg. @deprecated ...), just put annotation in rust str.
* If importing is needed, then add importing part behind the annotation str.
  * Currently two forms of importing supported:
    * `import 'somepackage'`
    * `import 'somepackage' as somename`, and `somename` will be the prefix of the annotation
* Multiple annotations are seperated by comma ","


## Metadata Example
```rust,noplayground
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserId {
    pub value: u32,
}
```

Becomes:
```Dart
import 'package:meta/meta.dart' as meta;

@freezed
@meta.immutable
class UserId with _$UserId {
  const factory UserId({
    required int value,
  }) = _UserId;
}
```
