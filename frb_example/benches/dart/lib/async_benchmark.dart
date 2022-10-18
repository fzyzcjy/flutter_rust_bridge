export 'async_benchmark.io.dart' if (dart.library.html) 'async_benchmark.web.dart' show AsyncBenchmark;

import 'package:flutter_rust_bridge_benchmark/constants.dart';
import 'package:flutter_rust_bridge_benchmark/env/stub.dart';

import 'walltime.dart';

class Routine {
  final int iterations;
  final double elapsed;
  const Routine(this.iterations, this.elapsed);
}

class Sample {
  final List<Routine> routines;
  const Sample(this.routines);
  int get iterations => routines.fold(0, (previousValue, element) => previousValue + element.iterations);
  double get elapsed => routines.fold(0, (previousValue, element) => previousValue + element.elapsed);
}

abstract class Bencher {
  final String name;
  final Duration warmUpTime;
  final Duration measurementTime;
  final int sampleSize;

  WallTime start();

  Bencher({required this.name, required this.warmUpTime, required this.measurementTime, required this.sampleSize});

  Future<void> setup() {
    return Future.sync(() {
      print(
          'setup (sample count: $sampleSize, items count: $itemsCount, warm up time: ${warmUpTime.inMilliseconds} ms, measurement time: ${measurementTime.inMilliseconds} ms)');
    });
  }

  Future<void> teardown();

  /// see [criterion warmup](https://bheisler.github.io/criterion.rs/book/analysis.html#warmup)
  Future<Routine> warmup(Future<void> Function() f);

  /// see [criterion warmup](https://bheisler.github.io/criterion.rs/book/analysis.html#measurement)
  Future<Sample> measure(Future<void> Function() f, double warmUpElapsed, int warmUpIter, int sampleSize);

  Future<void> run();

  Future<Sample> bench() async {
    await setup();
    try {
      final routine = await warmup(run);
      return await measure(run, routine.elapsed, routine.iterations, sampleSize);
    } finally {
      await teardown();
    }
  }

  Future<void> report() async {
    assert(sampleSize >= 10);
    assert(warmUpTime >= minimumBenchDuration);
    assert(measurementTime >= minimumBenchDuration);
    print('---');
    final Sample sample = await bench();

    print('report: ${sample.elapsed} microseconds for ${sample.iterations} iterations');
    print('per iteration: ${sample.elapsed / sample.iterations} microseconds');
    print('completed ${sample.routines.length} sample(s) out of $sampleSize');
  }
}

abstract class AsyncBencher extends Bencher {
  AsyncBencher(
      {required super.name, required super.warmUpTime, required super.measurementTime, required super.sampleSize});

  int get warmUpTimeNormalized;
  int get measurementTimeNormalized;

  @override
  Future<Routine> warmup(Future<void> Function() f) async {
    var iterations = 1;
    var elapsed = .0;
    final WallTime time = start();
    while (elapsed < warmUpTimeNormalized) {
      for (var i = 0; i < iterations; i++) {
        await f();
      }
      elapsed = time.timeElapsedMicros;
      iterations *= 2;
    }
    time.stop();
    print(
        'warmed up for $elapsed microseconds for a total of $iterations iterations (approximately ${elapsed / iterations} microseconds per iteration)');
    return Routine(iterations, elapsed);
  }

  @override
  Future<Sample> measure(Future<void> Function() f, double warmUpElapsed, int warmUpIter, int sampleSize) async {
    final totalIters = ((measurementTimeNormalized / warmUpTimeNormalized) * warmUpIter).round();
    final totalRuns = sampleSize * (sampleSize + 1) / 2;
    assert(totalRuns < totalIters);
    final d = warmUpElapsed / warmUpIter;
    final runs = List.generate(sampleSize, (index) => ((index + 1) * d).round(), growable: false);
    var factor = 1;
    var elapsed = .0;
    var samples = <Routine>[];
    var sample = 0;
    while (elapsed < measurementTimeNormalized && factor <= sampleSize) {
      sample = runs[factor - 1];
      final WallTime watch = start();
      for (var i = 0; i < sample; i++) {
        await f();
      }
      final took = watch.timeElapsedMicros;
      watch.reset();
      elapsed += took;
      samples.add(Routine(sample, took));
      factor++;
    }
    return Sample(samples);
  }
}
