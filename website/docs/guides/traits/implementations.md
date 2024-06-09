# Trait implementations

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
