# Synchronous Dart

In order to generate synchronous functions in Dart,
add `#[frb(sync)]` attributes to the Rust function.
As usual, this feature is compatible with other features,
e.g. you can use arbitrary argument and return types.

It is useful when you want to make the generated Dart API be synchronous,
especially helpful when you cannot `await` something,
for example inside Flutter's `build`.

If your function is quick and is called a ton of times,
using synchronous mode will reduce the calling overhead (though already small).
If, on the other hand, your function is slow,
it is recommended to use the asynchronous mode,
because synchronous mode will block the Dart UI.

## Example

```rust
fn normal() {}

#[frb(sync)]
fn dart_counterpart_is_synchronous() {}
```

Dart:

```dart
await normal(); // Need await
dartCounterpartIsSynchronous(); // No need await
```
