# Using `build_runner`

Inspect your `lib/bridge_generated.dart` and you will see that the definition of `Platform` has changed:

```dart
@freezed
class Platform extends _$Platform {
    const factory Platform.unknown() = Unknown;
    const factory Platform.android() = Android;
    const factory Platforn.ios() = Ios;
    const factory Platform.windows() = Windows;
    const factory Platform.unix() = Unix;
    const factory Platform.macOs(String field0) = MacOs;
    const factory Platform.wasm() = Wasm;
}
```

It is no longer a plain enum, but a full-blown enum class with variants! As it is right now, this code
cannot compile yet since it is missing some components, namely the `freezed` library.