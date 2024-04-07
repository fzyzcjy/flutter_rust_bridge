# Equals and Hash

If you want to provide arbitrary custom Rust functions to compute the `equals` and `hashCode` of a generated Dart class,
simply add the `#[frb(derive(Eq, Hash))]` on the struct.

As a remark, without this, they will by default be the Dart default one for normal structs/enums,
or the `freezed`-generated one (usually field-by-field comparison) if you use the `freezed` mode on the structs/enums.

## Example

```rust
#[frb(derive(Eq, Hash))]
pub struct MyStruct {
    ...
}
```

This annotation will generate `equals`/`hashCode` functions at Dart side which calls the equivalent Rust functions.

As for how to implement `equals`/`hashCode` on Rust side, just follow standard Rust practice.
For example, the usual way is to add `#[derive(PartialEq, Eq, Hash)]` and the Rust compiler will do it for you.
Alternatively, you are free to manually write down implementations by things like `impl Hash for MyStruct { ... }`.
