import 'package:benchmark_harness/benchmark_harness.dart';

abstract class EnhancedBenchmarkBase extends BenchmarkBase {
  const EnhancedBenchmarkBase(super.name);

  // To opt into the reporting the time per run() instead of per 10 run() calls.
  @override
  void exercise() => run();
}
