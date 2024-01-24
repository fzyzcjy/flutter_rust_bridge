# Dart Isolates

:::tip
There is no need to read this page,
unless you need to use it in multiple Dart Isolates.
:::

To use Dart Isolates, just call `RustLib.init()` and `RustLib.dispose()` when your isolate starts and stops,
just like how you deal with many standard Flutter objects.

Working examples (tests that are executed in CI) can be seen [here](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/test/isolate_test.dart).

As a remark, often there is no need to use Dart Isolates when interacting with flutter_rust_bridge.
Since flutter_rust_bridge supports async, even if your Rust function is super slow, it will never block the Dart code.
