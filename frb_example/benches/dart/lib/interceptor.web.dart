import 'dart:developer';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

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

mixin FlutterRustBridgeInterceptorMixin<T extends FlutterRustBridgeWireBase>
    on FlutterRustBridgeBase<T> {
  @override
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> op) async {
    final AsyncStopWatch? stopwatch =
        await interceptor.beforeExecuteNormal(op.constMeta.debugName);
    final result = await super.executeNormal(op);
    if (stopwatch != null) {
      await interceptor.afterExecuteNormal(op.constMeta.debugName, stopwatch);
    }
    return result;
  }

  @override
  S executeSync<S>(FlutterRustBridgeSyncTask op) {
    final AsyncStopWatch? stopwatch =
        interceptor.beforeExecuteSync(op.constMeta.debugName);
    final result = super.executeSync(op);
    if (stopwatch != null) {
      interceptor.afterExecuteSync(op.constMeta.debugName, stopwatch);
    }
    return result;
  }

  FlutterRustBridgeInterceptor get interceptor;
}
