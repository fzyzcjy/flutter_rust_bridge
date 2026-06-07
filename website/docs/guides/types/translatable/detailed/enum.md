# Enums

Rust's `enum` are known to be very expressive and powerful - it allows each enum variant to have different associated data. Dart does not have such things in built-in enums, but no worries - we will automatically translate it into the equivalent using the `freezed` Dart library by default. The syntax for `freezed` may look a bit strange at the first glance, but please look at [its doc](https://pub.dev/packages/freezed) and see its powerfulness.

Some features are documented in [structs](struct.md). For example, how to make it support json serialization, how to unignore a type, etc.

## Example

```rust
pub enum KitchenSink {
    Empty,
    Primitives {
        /// Dart field comment
        int32: i32,
        float64: f64,
        boolean: bool,
    },
    Nested(Box<KitchenSink>),
    Optional(
        /// Comment on anonymous field
        Option<i32>,
        Option<i32>,
    ),
    Buffer(ZeroCopyBuffer<Vec<u8>>),
    Enums(Weekdays),
}
```

Becomes:

```Dart
@freezed
class KitchenSink with _$KitchenSink {
  /// Comment on variant
  const factory KitchenSink.empty() = Empty;
  const factory KitchenSink.primitives({
    /// Dart field comment
    required int int32,
    required double float64,
    required bool boolean,
  }) = Primitives;
  const factory KitchenSink.nested(
    KitchenSink field0,
  ) = Nested;
  const factory KitchenSink.optional([
    /// Comment on anonymous field
    int? field0,
    int? field1,
  ]) = Optional;
  const factory KitchenSink.buffer(
    Uint8List field0,
  ) = Buffer;
  const factory KitchenSink.enums(
    Weekdays field0,
  ) = Enums;
}
```

And they are powered with [all functionalities](https://pub.dev/packages/freezed) of `freezed`.

Remark: If you are curious about `Future`, have a look at [this](../../../concurrency/async-dart).

## Pattern matching in Dart

Introduced in Dart 3, sealed classes can be used to [pattern match](https://dart.dev/language/patterns) values,
enabling exhaustive variant checks and refuable patterns among other capabilities. Refer to [the documentation](https://dart.dev/language/patterns#switch-statements-and-expressions)
for more details.

This feature supersedes Freezed's `map` and `when` families of methods.
You can opt out of generating sealed classes by passing `--no-dart3` when running codegen.

```rust
pub enum Maybe {
    None,
    Some { value: i32 },
}
```

```dart
Maybe maybe;
final value = switch (maybe) {
  Maybe_None() => 'got nothing',
  Maybe_Some(:final value) => 'got value: $value',
};
// single case à la if-let
if (maybe case Maybe_Some(:final value)) {
  ..
}
```

## Native Dart classes without Freezed

If you only need Dart 3 sealed classes and pattern matching, and do not need Freezed-generated helpers such as `copyWith`, `map`, or `when`, disable Freezed for complex enum generation:

```yaml
dart_enums_freezed: false
```

Or pass the CLI flag:

```bash
flutter_rust_bridge_codegen generate --no-dart-enums-freezed
```

With this option, complex Rust enums are generated as a native Dart base class plus concrete variant subclasses. The constructors and variant class names remain compatible with the Freezed-backed shape used by FRB, so generated codec code and Dart pattern matching continue to use names such as `Maybe.some(...)` and `Maybe_Some`.
