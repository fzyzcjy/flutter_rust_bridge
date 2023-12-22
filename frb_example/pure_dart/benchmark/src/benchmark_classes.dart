// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'package:benchmark_harness/benchmark_harness.dart';

import 'benchmark_utils.dart';
import 'generated.dart' as generated;
import 'manual_benchmarks.dart';

List<MaybeAsyncBenchmarkBase> createBenchmarks(
    {required ScoreEmitter emitter}) {
  return [
    // compute prime
    ComputePrimeBenchmark(90000049, emitter: emitter),
    ComputePrimeBenchmark(9000000001, emitter: emitter),
    ComputePrimeBenchmark(900000000013, emitter: emitter),

    ...generated.createBenchmarks(emitter: emitter),
  ];
}
