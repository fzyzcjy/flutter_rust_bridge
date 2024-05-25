# Constructors

Firstly, when a Rust struct has synchronous `new` method,
it will automatically become the default constructor in Dart.
For example:

```rust
pub struct MyStruct {
    ...
}

impl MyStruct {
    #[frb(sync)]
    pub fn new() -> Self { ... }
}
```

will become:

```dart
var a = MyStruct(); // calls the `fn new()` on Rust side
```

The reason why the `sync` is needed is that,
things are async by default to ensure Flutter thread (isolate) is never blocked.

Secondly, you can construct your object freely using whatever method name and synchronousness,
and they will be simply static methods and nothing special happens.
For example:

```rust
impl MyStruct {
    pub fn new_with_name(name: String) -> Self { ... }

    #[frb(sync)]
    pub fn new_from_pieces(a: String, b: i32, c: Vec<u8>) -> Self { ... }

    pub async fn whatever_you_like(x: (String, String)) -> Self { ... }
}
```

can be used as:

```dart
var a = await MyStruct.newWithName(...);
var b = MyStruct.newFromPieces(...);
var c = await MyStruct.whateverYouLike(...);
```
