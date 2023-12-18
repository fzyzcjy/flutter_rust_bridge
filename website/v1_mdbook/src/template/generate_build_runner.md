# Using `build_runner`

Inspect your `lib/bridge_generated.dart` and you will see that the definition of `Platform` has changed:

```dart
@freezed
sealed class Platform with _$Platform {
    const factory Platform.unknown() = Platform_Unknown;
    const factory Platform.android() = Platform_Android;
    const factory Platforn.ios() = Platform_Ios;
    const factory Platform.windows() = Platform_Windows;
    const factory Platform.unix() = Platform_Unix;
    const factory Platform.macOs(
        String field0,
    ) = Platform_MacOs;
    const factory Platform.wasm() = Platform_Wasm;
}
```

It is no longer a plain enum, but a full-blown enum class with variants! As it is right now, this code
cannot compile yet since it is missing some components, namely the [`freezed`] library. [`freezed`] is a
codegen library similar to those we've encountered thus far, but generates more Dart code instead.
All such libraries perform their code generation upon invoking `build_runner`, i.e. when `flutter pub run build_runner build` is executed.

Regardless, to make this code compile again, we need to make a few changes:
- Run the following commands to add the latest version of [`freezed`]:

```shell
flutter pub add -d build_runner
flutter pub add -d freezed
flutter pub add freezed_annotation
```

- Update `justfile` to run `build_runner` after Rust codegen:

```diff
 gen:
     ..
     # Uncomment this line to invoke build_runner as well
-    # flutter pub run build_runner build
+    flutter pub run build_runner build
```

Now calling `just` will generate both the Rust bindings *and* the Dart library code.

[`freezed`]: https://pub.dev/packages/freezed