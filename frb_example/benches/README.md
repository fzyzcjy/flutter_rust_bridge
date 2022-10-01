## Benchmarks example for `flutter_rust_bridge`

This repo is a sample on how to implement benchmarks with flutter rust bridge.
You can find a first example, using [benchmark_harness](https://pub.dev/packages/benchmark_harness), in `dart/lib/benchmark`.
It can be run like:

```sh
just bench
# just bench --define=SILICON=true (on latest MacOS Silicon)
```

In the same folder, you'll also find how to implement your custom interceptor in `interceptor.dart`. As you'll notice, a slight change is made to both `ffi.io.dart` and `ffi.web.dart` to initialize it instead.
It can be run like:

```sh
just test-benches
# just test-benches --define=SILICON=true (on latest MacOS Silicon)
```
