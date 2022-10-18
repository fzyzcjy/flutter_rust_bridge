import 'package:flutter_rust_bridge_benchmark/walltime.dart';
import 'package:flutter_rust_bridge_benchmark/walltime.io.dart';

import 'async_benchmark.dart';

abstract class AsyncBenchmark extends AsyncBencher {
  AsyncBenchmark(
      {required super.name, required super.warmUpTime, required super.measurementTime, required super.sampleSize});

  @override
  int get warmUpTimeNormalized => warmUpTime.inMicroseconds;
  @override
  int get measurementTimeNormalized => measurementTime.inMicroseconds;

  @override
  WallTime start() {
    return AsyncStopWatch()..start();
  }
}
