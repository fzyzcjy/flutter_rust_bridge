# Wrapping up

With our new definition of `Platform` in place, we can rewrite the previous code to make use
of it! Here is an example of what you can do with freezed enums.

In `lib/main.dart`:

```diff
- final text = const {
-   Platform.Android: 'Android',
-   Platform.Ios: 'iOS',
-   Platform.MacApple: 'MacOS with Apple Silicon',
-   Platform.MacIntel: 'MacOS',
-   Platform.Windows: 'Windows',
-   Platform.Unix: 'Unix',
-   Platform.Wasm: 'the Web',
- }[platform] ??
- 'Unknown OS';
+ final text = platform.when(
+   android: () => 'Android',
+   ios: () => 'iOS',
+   macOs: (arch) => 'MacOS on $arch',
+   windows: () => 'Windows',
+   unix: () => 'Unix',
+   wasm: () => 'the Web',
+ );
```

In `native/src/api.rs`:

```diff
     } else if cfg!(target_os = "ios") {
         Platform::Ios
     } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
-        Platform::MacApple
+        Platform::MacOs("Apple Silicon".into())
     } else if cfg!(target_os = "macos") {
-        Platform::MacIntel
+        Platform::MacOs("Intel".into())
     } else if cfg!(target_family = "wasm") {
         Platform::Wasm
     } else if cfg!(unix) {
```

When you `flutter run`, you should get something like this:
![macos-intel](macos_intel.png)

## Tip: Using switch expressions

Introduced in Dart 3, switch expressions provide the equivalent of Rust's `match` expressions, complete with exhaustive checks.
Instead of using `when()` in the above example, you could also use this syntax:

```dart
final text = switch (platform) {
  Platform_Android() => 'Android',
  Platform_Ios() => 'iOS',
  Platform_MacOs(:final arch) => 'MacOS on $arch',
  Platform_Windows() => 'Windows',
  Platform_Unix() => 'Unix',
  Platform_Wasm() => 'the Web',
  // we have covered all cases, so this compiles.
};
```
