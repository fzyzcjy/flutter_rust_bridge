# `Option`s

Dart has special syntaxs for nullable variables - the `?` symbol, and we translate `Option` into `?` automatically. You may refer to [the official doc](https://dart.dev/null-safety) for more information. 

In addition, `flutter_rust_bridge` also understands the `required` keyword in Dart: If an argument is not-null, it is marked as `required` since you have to provide a value. On the other hand, if it is nullable, no `required` is needed since by Dart's convention a null is there in absence of manually providing a value.

## Example

```rust,noplayground
pub struct Element {
    pub tag: Option<String>,
    pub text: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub children: Option<Vec<Element>>,
}

pub fn parse(mode: String, document: Option<String>) -> Option<Element> { ... }
```

Becomes:

```Dart
Future<Element?> handleOptionalStruct({required String mode, String? document});

class Element {
  final String? tag;
  final String? text;
  final List<Attribute>? attributes;
  final List<Element>? children;
  Element({this.tag, this.text, this.attributes, this.children});
}
```

Remark: If you are curious about `Future`, have a look at [this](async_dart.md).

