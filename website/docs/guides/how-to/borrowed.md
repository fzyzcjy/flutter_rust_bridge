# Borrowed types

flutter_rust_bridge does not yet support types with borrowing and lifetime
like `struct MyStruct<'a> { field: &'a Another }`,
since this has not been a blocker for users and there are alternative approaches.
However, if you do need to have this feature, feel free to open an issue and we can discuss/implement it!

One alternative approach is to use shared ownership.
For example, for the example above, we can rewrite as:

```rust
struct MyStruct {
    field: Arc<Another>,
}
```

As a side remark, if you want to expose that field to Dart side in opaque struct,
you can also wrap as `RustAutoOpaque<Another>`.
