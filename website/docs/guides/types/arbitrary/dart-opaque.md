# Automatic arbitrary Dart type

This feature is just like the mirror of the [automatic arbitrary Rust type](rust-auto-opaque).
Therefore, just use the mental model and knowledge learnt in the previous section,
and swap all "Rust" with "Dart", and we only briefly explain it here.

In addition to directly use it,
this feature is also useful when in combination with [Rust calls Dart](../../direction/rust-call-dart).

## Example

Here, we pass a Dart closure `() => '42'` into the Rust world, and get it back to Dart, and use it:

Rust:

```rust
pub fn put_dart_opaque(a: DartOpaque) { ... }
pub fn get_dart_opaque() -> DartOpaque { ... }
```

And use it in Dart:

```dart
await putDartOpaque(() => '42');
var answer = (await getDartOpaque()) as String Function(); // it is the `() => '42'` closure
print(answer()); // print '42'
```

## Implementation details

As for how it is implemented as well as the design towards safety,
please refer to [this doc](../../contributing/submodules/dart-opaque)
