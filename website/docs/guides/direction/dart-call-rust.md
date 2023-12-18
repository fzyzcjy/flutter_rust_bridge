# Dart calls Rust

The Dart code can seamlessly call the Rust code.
This is shown throughout the whole documentation, so here we only briefly demonstrates it,
and this page exists mainly to make the documentation symmetrical (the other half is "Rust calls Dart").

As a super simple example, suppose you have a Rust function (again, see the whole doc for more features):

```rust
fn my_rust_function(a: String) -> String { a.repeat(2) }
```

Then you can call it in Dart:

```dart
print(myRustFunction('Hello'));
```

