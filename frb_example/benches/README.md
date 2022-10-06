## Benchmarks example for `flutter_rust_bridge`

*`flutter_rust_bridge` or "`FRB`"" for conciseness.*

This repo is a sample on how to implement benchmarks with FRB.

### Get runnin'

#### simple benchmark

First example uses [benchmark_harness](https://pub.dev/packages/benchmark_harness), in `dart/lib/benchmark`.

It can be run like:

```sh
just bench
# just bench --define=SILICON=true (on latest MacOS Silicon)
```

#### custom benchmark

In the same folder, you'll also find a slightly more convoluted example on how to implement a custom platform-agnostic interceptor.

It can be run like:

```sh
just test-benches
# just test-benches --define=SILICON=true (on latest MacOS Silicon)
```

### How does it work ?

Glad you asked!

All the nitty gritty details can be found in the [book, under the section Benchmarks](http://cjycode.com/flutter_rust_bridge/feature/benchmarks.html).
