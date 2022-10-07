## Benchmarks example for `flutter_rust_bridge`

*`flutter_rust_bridge` or "`FRB`"" for conciseness.*

This repo is a sample on how to implement benchmarks with FRB.

### Get runnin'

#### simple benchmark

First example uses [benchmark_harness](https://pub.dev/packages/benchmark_harness), in `dart/lib/benchmark`.

It can be run like:

```sh
just bench-simple
# just bench-simple --define=SILICON=true (on latest MacOS Silicon)
```

:bulb: You can also customize the size of the vecs benchmarked with env var e.g. `SAMPLE_COUNT=100`. (default: `1000`)

Example resides in [frb_example/benches/dart/lib/benchmark](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/benches/dart/lib/benchmark).

#### custom benchmark

In the same folder, you'll also find a slightly more convoluted example on how to implement a custom platform-agnostic interceptor.

It can be run like:

```sh
just bench-custom
# just bench-custom --define=SILICON=true (on latest MacOS Silicon)
```

:bulb: You can alternatively output as json with env var e.g. `JSON=true`. (default: `false`)

Example resides both at [frb_example/benches/dart](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/benches/dart)
and [frb_example/benches/rust](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/benches/rust).

### How does it work ?

Glad you asked!

All the nitty gritty details can be found in the [book, under the section Benchmarks](http://cjycode.com/flutter_rust_bridge/feature/benchmarks.html).
