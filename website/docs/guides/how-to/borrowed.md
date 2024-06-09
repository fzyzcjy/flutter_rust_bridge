# Borrowed types

flutter_rust_bridge does not yet support types with *arbitrary* borrowing and lifetime
like `struct MyStruct<'a> { ... }`,
or functions that return arbitrary reference types,
since this has not been a blocker for users and there are alternative approaches.
However, if you do need to have this feature, feel free to open an issue and we can discuss/implement it!
Below, we discuss several approaches for some scenarios.

## Proxy

The proxy feature can be utilized when the borrowed type is indeed a field of struct or something like that.
Please refer to [this page](../miscellaneous/proxy) for more details.

## Shared ownership

One alternative approach is to use shared ownership.
For example, for the example above, we can rewrite as:

```rust
struct MyStruct {
    field: Arc<Another>,
}
```

As a side remark, if you want to expose that field to Dart side in opaque struct,
you can also wrap as `RustAutoOpaque<Another>`.
