// ignore: unused_import
import 'dart:io';

import 'package:flutter_rust_bridge_benchmark/env/io.dart';
import 'package:flutter_rust_bridge_benchmark/walltime.dart';
import 'package:flutter_rust_bridge_benchmark/walltime.io.dart';

import 'async_benchmark.dart';

/// use custom Stopwatch on Dart native
abstract class AsyncBenchmark extends AsyncBencher {
  AsyncBenchmark(
      {required super.name,
      required super.dylibPath,
      required super.warmUpTime,
      required super.measurementTime,
      required super.sampleSize});

  @override
  WallTime start() {
    return AsyncStopWatch()..start();
  }

  @override
  Future<void> save(Sample sample) async {
    final file = File('../../../book/benches/$outputFilename.txt')..create(recursive: true);
    file.writeAsString(sample.toJson().toString());
  }
}
