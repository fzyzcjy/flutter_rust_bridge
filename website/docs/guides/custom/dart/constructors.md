# Constructor parameters

As we know (and auto injected when using `flutter_rust_bridge_codegen create/integrate` command),
we need to initialize the system via `await RustLib.init();` (or your own lib name).
This constructor allows some customization, i.e.

```dart
await RustLib.init(someCustomizableParameter: someValue);
```

For full and latest configurations, please refer to the method signature directly.
The Dart classes are designed to be modularized, so it should not be too hard to plug in and replace some modules
to satisfy your needs.

Some typical scenarios are:

* [Testing and mocking](../../how-to/test)
* [Inspection](../../how-to/inspect)
* [Load Rust library](../../how-to/load-library)
