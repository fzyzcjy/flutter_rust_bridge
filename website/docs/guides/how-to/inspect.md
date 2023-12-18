# Inspection / Hooks / Aspect-oriented programming

Sometimes, we may want to add some logic when things like Dart-call-Rust starts and ends.
For example, we may want to print a log line when it starts or ends,
or we may want to do a simple timing to find the slow function.
We can do it easily in both Rust and Dart side.

## Rust

You can write your own `Executor`, which wraps the default executor and do some extra work.

TODO: explain how to use your own (create an issue if you want to know and this doc is still not updated).

## Dart

Similarly, you can utilize the `BaseHandler` class by providing your own subclass.
Then, provide your subclass during initialization:

```dart
await RustLib.init(handler: YourCustomHandler());
```

For a working example (and is tested continuously in CI), please refer to
[this file](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/test/custom_handler_test.dart).
