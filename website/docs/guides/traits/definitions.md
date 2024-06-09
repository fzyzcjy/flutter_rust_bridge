# Trait definitions

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
