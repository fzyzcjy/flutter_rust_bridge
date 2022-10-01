import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated.io.dart'
    if (dart.library.html) 'bridge_generated.web.dart';
import 'dart:ffi' as ffi;

class FlutterRustBridgeExampleBenchmarkSuiteImplBench
    extends FlutterRustBridgeExampleBenchmarkSuiteImpl {
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench(ExternalLibrary dylib,
          {bool? useJSON}) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench.raw(
          FlutterRustBridgeExampleBenchmarkSuitePlatformBench(
              dylib,
              useJSON ?? false
                  ? FlutterRustBridgeInterceptorJson()
                      as FlutterRustBridgeInterceptor<AsyncStopWatch>
                  : FlutterRustBridgeInterceptorStdOut()));

  /// Only valid on web/WASM platforms.
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench.wasm(
          FutureOr<WasmModule> module,
          {bool? useJSON}) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench(module as ExternalLibrary,
          useJSON: useJSON ?? false);

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

class UniqueAsyncStopWatch extends AsyncStopWatch {
  final UuidValue uuid;
  UniqueAsyncStopWatch(this.uuid);
}

abstract class FlutterRustBridgeInterceptor<T extends AsyncStopWatch> {
  Future<T> beforeExecuteNormal(String debugName);

  Future<void> afterExecuteNormal<S>(String debugName, T stopwatch);

  T beforeExecuteSync<S>(String debugName);

  void afterExecuteSync<S>(String debugName, T stopwatch);

  Future<T> _beforeExecuteNormal(String debugName, T stopwatch) async {
    return Future.sync(() {
      stopwatch.start();
      return stopwatch;
    });
  }

  Future<void> _afterExecuteNormal<S>(String debugName, T stopwatch) async {
    return Future.sync(() {
      stopwatch.stop();
    });
  }

  T _beforeExecuteSync<S>(String debugName, T stopwatch) {
    stopwatch.start();
    return stopwatch;
  }

  void _afterExecuteSync<S>(String debugName, T stopwatch) {
    stopwatch.stop();
  }
}

class FlutterRustBridgeInterceptorStdOut
    extends FlutterRustBridgeInterceptor<AsyncStopWatch> {
  @override
  Future<AsyncStopWatch> beforeExecuteNormal(String debugName) async {
    print('⚡');
    print('(Dart) execute [$debugName] start');
    final AsyncStopWatch stopwatch = AsyncStopWatch();
    await super._beforeExecuteNormal(debugName, stopwatch);
    return stopwatch;
  }

  @override
  Future<void> afterExecuteNormal<S>(
      String debugName, AsyncStopWatch stopwatch) async {
    await super._afterExecuteNormal(debugName, stopwatch);
    print(
        '(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts!}μs');
  }

  @override
  AsyncStopWatch beforeExecuteSync<S>(String debugName) {
    print('(Dart) execute [$debugName] start');
    final AsyncStopWatch stopwatch = AsyncStopWatch();
    super._beforeExecuteSync(debugName, stopwatch);
    return stopwatch;
  }

  @override
  void afterExecuteSync<S>(String debugName, AsyncStopWatch stopwatch) {
    super._afterExecuteSync(debugName, stopwatch);
    print(
        '(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts!}μs');
  }
}

class Metric {
  final int starts;
  int? ends;
  Metric(this.starts);
}

class FlutterRustBridgeInterceptorJson
    extends FlutterRustBridgeInterceptor<UniqueAsyncStopWatch> {
  Map<UuidValue, Metric> metrics = {};
  final Uuid generator = Uuid();
  @override
  Future<UniqueAsyncStopWatch> beforeExecuteNormal(String debugName) async {
    final UuidValue uuid = generator.v4obj();
    UniqueAsyncStopWatch stopwatch = UniqueAsyncStopWatch(uuid);
    metrics.putIfAbsent(uuid, () => Metric(stopwatch.elapsedMicroseconds));
    await super._beforeExecuteNormal(debugName, stopwatch);
    return stopwatch;
  }

  @override
  Future<void> afterExecuteNormal<S>(
      String debugName, UniqueAsyncStopWatch stopwatch) async {
    await super._afterExecuteNormal(debugName, stopwatch);
    metrics.update(stopwatch.uuid, (value) {
      value.ends = stopwatch.elapsedMicroseconds;
      return value;
    });
  }

  @override
  UniqueAsyncStopWatch beforeExecuteSync<S>(String debugName) {
    final UuidValue uuid = generator.v4obj();
    UniqueAsyncStopWatch stopwatch = UniqueAsyncStopWatch(uuid);
    metrics.putIfAbsent(uuid, () => Metric(stopwatch.elapsedMicroseconds));
    super._beforeExecuteSync(debugName, stopwatch);
    return stopwatch;
  }

  @override
  void afterExecuteSync<S>(String debugName, UniqueAsyncStopWatch stopwatch) {
    super._afterExecuteSync(debugName, stopwatch);
    metrics.update(stopwatch.uuid, (value) {
      value.ends = stopwatch.elapsedMicroseconds;
      return value;
    });
  }
}

class FlutterRustBridgeExampleBenchmarkSuitePlatformBench
    extends FlutterRustBridgeExampleBenchmarkSuitePlatform {
  final FlutterRustBridgeInterceptor<AsyncStopWatch> _interceptor;
  FlutterRustBridgeExampleBenchmarkSuitePlatformBench(
      ffi.DynamicLibrary dylib, this._interceptor)
      : super(dylib);
  FlutterRustBridgeInterceptor<AsyncStopWatch> get interceptor => _interceptor;
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
