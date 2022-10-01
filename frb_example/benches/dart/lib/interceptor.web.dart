import 'dart:developer';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_benchmark/interceptor.dart';

class AsyncStopWatch extends Stopwatch {
  int? starts;
  int? ends;
  @override
  void start() {
    starts = elapsedMicroseconds;
    super.start();
  }

  @override
  void stop() {
    ends = elapsedMicroseconds;
    super.stop();
  }
}

class FlutterRustBridgeInterceptor {
  Future<dynamic> beforeExecuteNormal(String _) async {
    return Future.sync(() {
      final AsyncStopWatch stopwatch = AsyncStopWatch();
      stopwatch.start();
      return stopwatch;
    });
  }

  Future<void> afterExecuteNormal<S>(String benchName, dynamic tracker) async {
    return Future.sync(() {
      final AsyncStopWatch stopwatch = tracker as AsyncStopWatch;
      stopwatch.stop();
      log('$benchName executed in ${stopwatch.ends! - stopwatch.starts!}');
    });
  }

  dynamic beforeExecuteSync<S>(String _) {
    final AsyncStopWatch stopwatch = AsyncStopWatch();
    stopwatch.start();
    return stopwatch;
  }

  void afterExecuteSync<S>(String benchName, dynamic tracker) {
    final AsyncStopWatch stopwatch = tracker as AsyncStopWatch;
    stopwatch.stop();
    log('$benchName executed in ${stopwatch.ends! - stopwatch.starts!}');
  }
}

mixin FlutterRustBridgeInterceptorMixin
    on FlutterRustBridgeExampleBenchmarkSuiteImplBench {
  @override
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> op) async {
    final AsyncStopWatch? stopwatch =
        await interceptor.beforeExecuteNormal(op.constMeta.debugName);
    final result = await executeNormal(op);
    if (stopwatch != null) {
      await interceptor.afterExecuteNormal(op.constMeta.debugName, stopwatch);
    }
    return result;
  }

  @override
  S executeSync<S>(FlutterRustBridgeSyncTask op) {
    interceptor.beforeExecuteSync(op.constMeta.debugName);
    final result = executeSync(op);
    interceptor.afterExecuteSync(op.constMeta.debugName, null);
    return result;
  }
}
