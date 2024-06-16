# Override opaqueness

By default, flutter_rust_bridge tries to infer whether a type is opaque or not automatically.
To override the default behavior, please use `#[frb(opaque)]` and `#[frb(non_opaque)]`.

## Example

If a type is indeed encodable, it will by default be translated to the corresponding Dart types.
However, if you want to force it to be opaque, you can use the `#[frb(opaque)]` attribute.

This is useful, for example, when the data is heavy and is mainly used in Rust,
and you do not want to transfer it between Dart and Rust over and over again.

For example:

```rust
struct A {
    name: String,
}

#[frb(opaque)]
struct B {
    name: String,
}
```

Will generate different Dart code:

```dart
// A pretty standard Dart class with fields inside it
class A { String name; ... }

// A Dart class without data fields, you should pass it to Rust to manipulate it
class B extends RustAutoOpaque { ... }
```
