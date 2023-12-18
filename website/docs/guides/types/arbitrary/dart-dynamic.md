# Dart dynamic type

Dart's `dynamic` is a special type that can hold any type of value. Although it is possible
to return a `dynamic` to Dart in the form of [`DartDynamic`], it is preferable to
return an `enum` instead that has a variant for each type you want to support.

## Example

Let's say you have a struct that can hold either a `u32` or a `String` and some
other fields (in a significantly worse design):

```rust
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

```rust
enum MyEnum {
    U32(u32),
    String(String),
}
```

And then you can define a struct that holds this `enum`:

```rust
struct MyStruct {
    msg:  String,
    data: MyEnum,
}
```

## Returning `dynamic`s

Aside from [`DartOpaque`], you may also return a `dynamic` type to Dart by specifing the return type as [`DartDynamic`].
[`DartDynamic`] is the umbrella type for all [C-representable](https://docs.rs/flutter_rust_bridge/latest/flutter_rust_bridge/ffi/io/ffi/enum.DartCObjectType.html)
Dart values, which can be obtained from Rust types that implement
[`IntoDart`](https://docs.rs/flutter_rust_bridge/latest/flutter_rust_bridge/ffi/trait.IntoDart.html).

```rust
pub fn return_dynamic() -> DartDynamic {
    vec![
        ().into_dart(),
        0i32.into_dart(),
        format!("Hello there!").into_dart()
    ].into_dart()
}
```

```dart
final dynamic values = await api.returnDynamic();
assert(values is List<dynamic>);
assert(values[0] == null);
assert(values[1] == 0);
assert(values[2] == "Hello there!");
```

[`DartDynamic`] is not supported as parameters, and structs that transitively include them may not be used in parameter positions either.
If you only care about accepting or returning an opaque Dart object without interacting with it, consider [`DartOpaque`].

This type is meant to be used only as an esacpe hatch, if your data cannot be expressed as either a fixed [struct](../translatable/detailed/struct) or [enum](../translatable/detailed/enum).

[`DartOpaque`]: ./dart-opaque
[`DartDynamic`]: https://docs.rs/flutter_rust_bridge/latest/flutter_rust_bridge/ffi/type.DartDynamic.html
