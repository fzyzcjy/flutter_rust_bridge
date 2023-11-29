import 'dart:math';

import 'benchmark_utils.dart';

void main() {
  const ComputePrimeBenchmark().report();
}

class ComputePrimeBenchmark extends EnhancedBenchmarkBase {
  const ComputePrimeBenchmark() : super('ComputePrime');

  @override
  void run() {
    // http://compoasso.free.fr/primelistweb/page/prime/liste_online_en.php
    final ans = isPrime(900000000013);
    if (!ans) throw Exception('unexpected');
  }

  @override
  void setup() {}

  @override
  void teardown() {}

  bool isPrime(int n) {
    final sqrtN = sqrt(n);
    for (var i = 2; i <= sqrtN; ++i) {
      if (n % i == 0) return false;
    }
    return true;
  }
}
