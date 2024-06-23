# Trait implementations

The `impl A for B` is supported, and is translated to a `class B implements A`.
In addition, the methods inside this `impl` block will also be converted automatically.

## Unignore a function

Some trait implementations are ignored by default to avoid generating meaningless things to Dart side such as `clone` and `deref`.
However, if a function is ignored while you want it, you can put arbitrary attributes on it to tell flutter_rust_bridge you want it.
For example, `#[frb] fn f() { .. }` suffices. Attributes with real contents like `#[frb(sync)] fn f() { .. }` also works.

## Example

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
