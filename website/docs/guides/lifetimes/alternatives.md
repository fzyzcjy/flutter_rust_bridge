# Alternatives

In addition to directly using the feature, sometimes the following alternative approaches are also helpful.

## Proxy

The proxy feature can be utilized when the borrowed type is indeed a field of struct or something like that.
Please refer to [this page](../misc-features/proxy) for more details.

## Shared ownership

One alternative approach is to use shared ownership.
For example, we can write something like:

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
