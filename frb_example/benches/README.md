## Benchmarks example for `flutter_rust_bridge`

*`flutter_rust_bridge` or "`FRB`"" for conciseness.*

This repo is a sample on how to implement benchmarks with FRB.

It's a home-baked implementation initially based on [benchmark_harness](https://pub.dev/packages/benchmark_harness) whose logic is altered to follow [criterion](https://github.com/bheisler/criterion.rs)'s [Analysis process](https://bheisler.github.io/criterion.rs/book/analysis.html) recommendations.

Briefly summarized, it adds *warmup* and *linear sampling measurement* phases using a benchmark_harness structure. Other sampling modes are left as an exercise to the reader: for example, when benchmarking large vecs, [flat sampling mode](https://docs.rs/criterion/latest/criterion/enum.SamplingMode.html) might be preferred.

### Get runnin'

#### benchmark for native platforms

Benchmark results will be printed to stdout and saved as a file, by default at `book/benches/output.txt`.

It can be run like:

```sh
just bench-simple
# just bench-simple --define=SILICON=true (e.g. on some latest MacOS Silicon)
```

:bulb: You can also customize a couple of settings:

- `SAMPLE_COUNT` iterations sampled size
  > default to 50
- `WARM_UP_TIME` warm up phase duration, in milliseconds
  > default to 3000
- `MEASUREMENT_TIME` measurement phase duration, in milliseconds
  > default to 5000
- `ITEMS_COUNT` sampled vec size
  > default to 10
- `OUTPUT_FILENAME` filename for benchmarks output
  > default 'output'

#### benchmark for web platform

It can be run like:

```sh
just bench-simple-web
```

The command has the exact same settings as its native counterpart (see above).
