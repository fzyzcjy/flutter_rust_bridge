# Properties

The `pub` fields of an opaque struct will be automatically translated,
such that it can be used as if it is a normal field.

## Example

Suppose we have the following opaque type:

```rust
pub struct MyOpaqueType {
    pub name: String,
    db: Database,
}
```

Then, the public field, `name`, will be recognized. The getters and setters will be generated:

```dart
// Auto-generated class
class MyOpaqueType {
    String get name => ...;
    set name(String value) => ...;
    ...
}
```

Then, we can use it as if it is a normal field:

```dart
var object = MyOpaqueType();
object.name += 'a';
print('Hi ${object.name}');
```

## Caveats

:::tip
There is no need to memorize anything here (or anything in doc) -
the code generator will provide warnings when detecting non-best-practices.
:::

Because borrowed types are not (yet) supported, the current implementation clones the field when reading it.
This is no problem when, for example, the field is an integer, a String, or a `RustAutoOpaque<T>`.
However, if it is 
TODO
