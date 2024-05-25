# Properties

The `pub` fields of an opaque struct will be automatically translated to corresponding properties.

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
