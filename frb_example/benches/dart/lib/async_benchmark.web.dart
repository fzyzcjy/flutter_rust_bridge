// ignore: unused_import
import 'dart:html';

import 'package:flutter_rust_bridge_benchmark/walltime.dart';
import 'package:flutter_rust_bridge_benchmark/walltime.web.dart';

import 'async_benchmark.dart';

abstract class AsyncBenchmark extends AsyncBencher {
  AsyncBenchmark(
      {required super.name, required super.warmUpTime, required super.measurementTime, required super.sampleSize});

  @override
  int get warmUpTimeNormalized => warmUpTime.inMilliseconds;
  @override
  int get measurementTimeNormalized => measurementTime.inMilliseconds;

  @override
  WallTime start() {
    return WindowPerformance();
  }

  @override
  Future<void> save(Sample sample) async {
    print(sample.toJson().toString());
  }
}
