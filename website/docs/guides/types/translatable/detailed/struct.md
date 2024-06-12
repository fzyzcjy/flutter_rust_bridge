# Structs

Normal Rust structs are supported. You can even use recursive fields, such as `pub struct TreeNode { pub value: String, pub children: Vec<MyTreeNode>, pub parent: Box<MyTreeNode> }`.

For versions older than v1.66.0 (no need for latest version), if a struct field has type being a struct or an enum, please add a `Box` on it, or it will lead to compile-time error. For example, `struct A {b: B}` should be `struct A {b: Box<B>}` instead.

## Tuple structs

Tuple structs `struct Foo(A, B)` are translated as `class Foo { A field0; B field1; }`, since Dart does not have anonymous fields.

## Non-final fields

By adding `#[frb(non_final)]` to a field of struct, the corresponding field in Dart will be non-final. By default, we make all generated fields final because of Rust's philosophy - immutable by default.

Unless a field has been annotated with `#[frb(non_final)]`, generated classes will also be const-constructible.

## Dart metadata annotations

You can add dart metadata annotations using `dart_metadata` parameter in `frb` macro.

* For annotations that are prelude by dart (e.g. `@deprecated`), just put annotation as a Rust literal.
* If importing is needed, then add importing part behind the annotation string. Currently two forms of importing supported:
  * `import 'somepackage'`
  * `import 'somepackage' as somename`, where `somename` will be the prefix of the annotation
* Multiple annotations are separated by comma `,`.

See below for an example.

## `freezed` Dart classes

If you want the generated Dart class to be [`freezed`](https://pub.dev/packages/freezed) (which is like data-classes in other languages like Kotlin), simply put `#[frb(dart_metadata=("freezed"))]` and it will generate everything needed for you.

## Rename fields

The `#[frb(name = "...")]` can be utilized to change the Dart name of a struct field.
For example:

```rust
#[frb]
pub struct MyStruct {
  #[frb(name = "dartFieldName")]
  pub rust_field_name: Vec<u8>,
}
```

It will give a Dart class with field `dartFieldName`.

## Example

### Example 1: Recursive fields

```rust
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

Remark: If you are curious about `Future`, have a look at [this](../../../concurrency/async-dart).

### Example 2: Metadata

```rust
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserId {
    pub value: u32,
}
```

Becomes:

```dart
import 'package:meta/meta.dart' as meta;

@freezed
@meta.immutable
class UserId with _$UserId {
  const factory UserId({
    required int value,
  }) = _UserId;
}
```
