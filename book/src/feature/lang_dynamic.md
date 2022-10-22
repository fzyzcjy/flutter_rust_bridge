# Dynamic Types

Darts `dynamic` type is a special type that can hold any type of value. It is
not per se supported by `flutter_rust_bridge`, but most use cases can likely be
solved by using an `enum` instead that has a variant for each type you want to
support.

## Example

Let's say you have a struct that can hold either a `u32` or a `String` and some
other fields:

```rust,noplayground
struct MyStruct {
    a: Optional<u32>,
    b: Optional<String>,
}

struct DataStruct {
    msg:  String,
    data: MyStruct,
}
```

You can define an `enum` in Rust to represent this:

```rust,noplayground
enum MyEnum {
    U32(u32),
    String(String),
}
```

And then you can define a struct that holds this `enum`:

```rust,noplayground
struct MyStruct {
    msg:  String,
    data: MyEnum,
}
```


