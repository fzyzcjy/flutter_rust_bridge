// ignore: unused_import
import 'dart:html';

import 'package:flutter_rust_bridge_benchmark/walltime.dart';
import 'package:flutter_rust_bridge_benchmark/walltime.web.dart';

import 'async_benchmark.dart';

/// use custom windows performance on Dart web
abstract class AsyncBenchmark extends AsyncBencher {
  AsyncBenchmark(
      {required super.name,
      required super.dylibPath,
      required super.warmUpTime,
      required super.measurementTime,
      required super.sampleSize});

  @override
  WallTime start() {
    return WindowPerformance();
  }

  @override
  Future<void> save(Sample sample) async {
    print(sample.toJson().toString());
  }
}
