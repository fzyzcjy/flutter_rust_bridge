import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'benchmark_utils.dart';

void main(List<String> args) {
  final emitter = JsonEmitter();

  ComputePrimeBenchmark(90000049, emitter: emitter).report();
  ComputePrimeBenchmark(9000000001, emitter: emitter).report();
  ComputePrimeBenchmark(900000000013, emitter: emitter).report();

  final pathOutput = args[0];
  print('Write reports to $pathOutput');
  File(pathOutput).writeAsStringSync(jsonEncode(emitter.items));
}

// For a list of primes: http://compoasso.free.fr/primelistweb/page/prime/liste_online_en.php
class ComputePrimeBenchmark extends EnhancedBenchmarkBase {
  final int number;

  const ComputePrimeBenchmark(this.number, {super.emitter})
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
