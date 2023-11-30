// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'dart:typed_data';

import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:frb_example_pure_dart/src/rust/api/benchmark_api.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.io.dart';

import 'benchmark_utils.dart';

void main(List<String> args) {
  final [pathOutput, partialName] = args;
  final emitter = JsonEmitter(namer: (x) => 'PureDart_${x}_$partialName');

  ComputePrimeBenchmark(90000049, emitter: emitter).report();
  ComputePrimeBenchmark(9000000001, emitter: emitter).report();
  ComputePrimeBenchmark(900000000013, emitter: emitter).report();

  VoidAsyncBenchmark(emitter: emitter).report();
  VoidSyncBenchmark(emitter: emitter).report();
  VoidSyncRawBenchmark(emitter: emitter).report();

  for (final size in [0, 10000, 1000000]) {
    InputBytesAsyncBenchmark(size, emitter: emitter).report();
    InputBytesSyncBenchmark(size, emitter: emitter).report();
    InputBytesSyncRawBenchmark(size, emitter: emitter).report();
  }

  final output = jsonEncode(emitter.items);
  print('Write reports to $pathOutput with output=$output');
  File(pathOutput).writeAsStringSync(output);
}

late final RustLibWire _wire = (RustLib.instance.api as RustLibApiImpl).wire;

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

  bool isPrime(int n) {
    final sqrtN = sqrt(n);
    for (var i = 2; i <= sqrtN; ++i) {
      if (n % i == 0) return false;
    }
    return true;
  }
}

class VoidAsyncBenchmark extends AsyncBenchmarkBase {
  const VoidAsyncBenchmark({super.emitter}) : super('VoidAsync');

  @override
  Future<void> run() async => benchmarkVoidTwinNormal();
}

class VoidSyncBenchmark extends EnhancedBenchmarkBase {
  const VoidSyncBenchmark({super.emitter}) : super('VoidSync');

  @override
  void run() => benchmarkVoidTwinSync();
}

class VoidSyncRawBenchmark extends EnhancedBenchmarkBase {
  VoidSyncRawBenchmark({super.emitter}) : super('VoidSyncRaw');

  @override
  void run() => _wire.benchmark_raw_void_sync();
}

class InputBytesAsyncBenchmark extends AsyncBenchmarkBase {
  final Uint8List bytes;

  InputBytesAsyncBenchmark(int size, {super.emitter})
      : bytes = Uint8List(size),
        super('InputBytesAsync_Size$size');

  @override
  Future<void> run() async => benchmarkInputBytesTwinNormal(bytes: bytes);
}

class InputBytesSyncBenchmark extends EnhancedBenchmarkBase {
  final Uint8List bytes;

  InputBytesSyncBenchmark(int size, {super.emitter})
      : bytes = Uint8List(size),
        super('InputBytesSync_Size$size');

  @override
  void run() => benchmarkInputBytesTwinSync(bytes: bytes);
}

class InputBytesSyncRawBenchmark extends EnhancedBenchmarkBase {
  final Uint8List bytes;

  InputBytesSyncRawBenchmark(int size, {super.emitter})
      : bytes = Uint8List(size),
        super('InputBytesSyncRaw_Size$size');

  @override
  void run() => TODO;
}
