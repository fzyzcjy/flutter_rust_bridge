# Customize Rust library loading

By default, during initialization,
the flutter_rust_bridge Dart/Flutter code checks places that should contain
the compiled Rust library in various scenarios (Flutter real app, Flutter test, Dart test, ...).

However, there is nothing to stop you from customizing this logic.
For example, maybe you are not using the default Flutter+Rust integration template,
but is compiling and loading the Rust libraries on your own.

To do so, simply add an argument during initialization:

```dart
await RustLib.init(externalLibrary: loadMyRustLibraryUsingWhateverMethod());
```

It may be useful to refer to the default loading logic in the function named `loadExternalLibrary`.
