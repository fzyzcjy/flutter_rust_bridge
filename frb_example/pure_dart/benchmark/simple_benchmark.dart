import 'dart:math';

import 'benchmark_utils.dart';

void main() {
  const ComputePrimeBenchmark(90000049).report();
  const ComputePrimeBenchmark(9000000001).report();
  const ComputePrimeBenchmark(900000000013).report();
}

// For a list of primes: http://compoasso.free.fr/primelistweb/page/prime/liste_online_en.php
class ComputePrimeBenchmark extends EnhancedBenchmarkBase {
  final int number;

  const ComputePrimeBenchmark(this.number)
      : super('ComputePrime_Number$number');

  @override
  void run() {
    final ans = isPrime(number);
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
