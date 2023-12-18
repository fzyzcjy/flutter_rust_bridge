# Initialization

Suppose you want to initialize the Rust side during Flutter/Dart startup,
then just simply call arbitrary Rust function as you like.
There is nothing special about initialization compared to normal execution.

For example:

```dart
Future<void> main() async {
  await RustLib.init();
  await myRustInitLogic(); // or `sync` if you like
  // ...
}
```

```rust
fn my_rust_init_logic() {
    // initialize whatever things here
}
```