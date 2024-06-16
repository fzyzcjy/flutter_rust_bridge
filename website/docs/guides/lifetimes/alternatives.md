# Alternatives

## Direct approach

`flutter_rust_bridge` does support types with borrowing and lifetime like `struct MyStruct<'a> { ... }`,
or functions that return types with lifetimes such as references.
Please refer to [this page](../types/arbitrary/rust-auto-opaque/lifetime) for details.

For completeness, below we discuss alternative approaches, which are useful in some scenarios.
(They are originally documented because at that time the lifetime had not been supported.)

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

## Clone

Yet another way is to simply clone related things when returned, instead of returning a reference.
This is especially reasonable when the resource being cloned is not heavy.
