import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.io.dart'
    if (dart.library.html) 'bridge_generated.web.dart';
import 'dart:ffi' as ffi;

class FlutterRustBridgeExampleBenchmarkSuiteImplBench
    extends FlutterRustBridgeExampleBenchmarkSuiteImpl {
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench(
          ExternalLibrary dylib) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench.raw(
          FlutterRustBridgeExampleBenchmarkSuitePlatformBench(
              dylib, FlutterRustBridgeInterceptor()));

  /// Only valid on web/WASM platforms.
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench.wasm(
          FutureOr<WasmModule> module) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench(
          module as ExternalLibrary);
  @override
  FlutterRustBridgeExampleBenchmarkSuiteImplBench.raw(
      FlutterRustBridgeExampleBenchmarkSuitePlatformBench platform)
      : super.raw(platform);
}

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
  Future<AsyncStopWatch> beforeExecuteNormal(String debugName) async {
    print('(Dart) execute [$debugName] start');
    return Future.sync(() {
      final AsyncStopWatch stopwatch = AsyncStopWatch();
      stopwatch.start();
      return stopwatch;
    });
  }

  Future<void> afterExecuteNormal<S>(
      String debugName, AsyncStopWatch stopwatch) async {
    return Future.sync(() {
      stopwatch.stop();
      print(
          '(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts!}μs');
    });
  }

  AsyncStopWatch beforeExecuteSync<S>(String debugName) {
    print('(Dart) execute [$debugName] start');
    final AsyncStopWatch stopwatch = AsyncStopWatch();
    stopwatch.start();
    return stopwatch;
  }

  void afterExecuteSync<S>(String debugName, AsyncStopWatch stopwatch) {
    stopwatch.stop();
    print(
        '(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts!}μs');
  }
}

class FlutterRustBridgeExampleBenchmarkSuitePlatformBench
    extends FlutterRustBridgeExampleBenchmarkSuitePlatform {
  final FlutterRustBridgeInterceptor _interceptor;
  FlutterRustBridgeExampleBenchmarkSuitePlatformBench(
      ffi.DynamicLibrary dylib, this._interceptor)
      : super(dylib);
  FlutterRustBridgeInterceptor get interceptor => _interceptor;
  @override
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> task) async {
    final String debugName = task.constMeta.debugName;
    final AsyncStopWatch stopwatch =
        await interceptor.beforeExecuteNormal(debugName);
    final result = await super.executeNormal(task);
    await interceptor.afterExecuteNormal(debugName, stopwatch);
    return result;
  }

  @override
  S executeSync<S>(FlutterRustBridgeSyncTask task) {
    final String debugName = task.constMeta.debugName;
    final AsyncStopWatch stopwatch = interceptor.beforeExecuteSync(debugName);
    final result = super.executeSync(task);
    interceptor.afterExecuteSync(debugName, stopwatch);
    return result;
  }
}
