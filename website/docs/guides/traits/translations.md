# Translations

## Trait definition

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

## Trait implementation

For example, suppose we write down:

```rust
impl MyTrait for MyStruct {
    fn f(&self, a: String) -> i32 { ... }
}
```

Then, the generated `MyStruct` class will be like:

```dart
class MyStruct implements MyTrait {
    ...
    int f(String a) => ...;
}
```

Therefore, the trait can act as an "interface" or an "abstract class" in the Dart world.
