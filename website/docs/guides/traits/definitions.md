# Trait definitions

The trait definitions will be automatically converted to Dart abstract classes.

The non-instance method (i.e. static method) will not be generated,
since Dart, unlike Rust, does not support such methods being implemented by subclasses.

## Example

For example, suppose we write down:

```rust
pub trait MyTrait {
    fn f(&self, a: String) -> i32;
}
```

It will become an abstract base class like:

```dart
abstract class MyTrait {
  int f(String a);
}
```
