# Trait objects / `dyn T` / Arbitrary implementers

The type `&dyn MyTrait`, where `MyTrait` is a trait, is supported.
It will be translated to `MyTrait` on Dart side.

This is especially helpful when we want to have an argument that accepts anything that implements the trait.

## Example

```rust
pub trait MyTrait {}
impl MyTrait for MyStructOne {}
impl MyTrait for MyStructTwo {}

pub fn f(a: &dyn MyTrait) {}
```

Then, the generated Dart signature is like:

```dart
void f(MyTrait a) => ...;
```

And we can use it like:

```dart
MyStructOne one = ...;
MyStructOne two = ...;

f(one); // allowed
f(two); // also allowed
```
