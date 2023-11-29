import 'benchmark_utils.dart';

void main() {
  const HelloBenchmark().report();
}

class HelloBenchmark extends EnhancedBenchmarkBase {
  const HelloBenchmark() : super('Hello');

  @override
  void run() {}

  @override
  void setup() {}

  @override
  void teardown() {}
}
