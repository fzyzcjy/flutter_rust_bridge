import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';
import 'package:logging/logging.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated.io.dart' if (dart.library.html) 'bridge_generated.web.dart';
import 'dart:ffi' as ffi;

class FlutterRustBridgeExampleBenchmarkSuiteImplBench extends FlutterRustBridgeExampleBenchmarkSuiteImpl {
  final FlutterRustBridgeExampleBenchmarkSuitePlatformBench platform;
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench(ExternalLibrary dylib, {bool? useJSON}) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench.raw(FlutterRustBridgeExampleBenchmarkSuitePlatformBench(
          dylib,
          useJSON ?? false
              ? FlutterRustBridgeInterceptorJson() as FlutterRustBridgeInterceptor<AsyncStopWatch>
              : FlutterRustBridgeInterceptorStdOut()));

  /// Only valid on web/WASM platforms.
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench.wasm(FutureOr<WasmModule> module, {bool? useJSON}) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench(module as ExternalLibrary, useJSON: useJSON ?? false);

  @override
  FlutterRustBridgeExampleBenchmarkSuiteImplBench.raw(this.platform) : super.raw(platform);

  Future<List<Metric>?> dartMetrics() async {
    return platform.metrics();
  }
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
  Future<T> beforeExecuteNormal(String debugName, Object? hint);

  Future<void> afterExecuteNormal<S>(String debugName, T stopwatch);

  T beforeExecuteSync<S>(String debugName, Object? hint);

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

class FlutterRustBridgeInterceptorStdOut extends FlutterRustBridgeInterceptor<AsyncStopWatch> {
  final log = Logger('FlutterRustBridgeInterceptorStdOut');
  @override
  Future<AsyncStopWatch> beforeExecuteNormal(String debugName, Object? hint) async {
    log.fine('(Dart) hint [$debugName] => ${hint.toString()}');
    log.fine('(Dart) execute [$debugName] start');
    final AsyncStopWatch stopwatch = AsyncStopWatch();
    await super._beforeExecuteNormal(debugName, stopwatch);
    return stopwatch;
  }

  @override
  Future<void> afterExecuteNormal<S>(String debugName, AsyncStopWatch stopwatch) async {
    await super._afterExecuteNormal(debugName, stopwatch);
    log.fine('(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts!}μs');
  }

  @override
  AsyncStopWatch beforeExecuteSync<S>(String debugName, Object? hint) {
    log.fine('(Dart) execute [$debugName] start');
    final AsyncStopWatch stopwatch = AsyncStopWatch();
    super._beforeExecuteSync(debugName, stopwatch);
    return stopwatch;
  }

  @override
  void afterExecuteSync<S>(String debugName, AsyncStopWatch stopwatch) {
    super._afterExecuteSync(debugName, stopwatch);
    log.fine('(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts!}μs');
  }
}

class FlutterRustBridgeInterceptorJson extends FlutterRustBridgeInterceptor<UniqueAsyncStopWatch> {
  Map<String, Metric> metrics = {};
  final Uuid generator = Uuid();
  final log = Logger('FlutterRustBridgeInterceptorJson');
  @override
  Future<UniqueAsyncStopWatch> beforeExecuteNormal(String debugName, Object? hint) async {
    final UuidValue uuid = generator.v4obj();
    UniqueAsyncStopWatch stopwatch = UniqueAsyncStopWatch(uuid);
    metrics.putIfAbsent(
        uuid.toString(), () => Metric(name: debugName, extra: hint.toString(), unit: Unit.Microseconds));
    await super._beforeExecuteNormal(debugName, stopwatch);
    return stopwatch;
  }

  @override
  Future<void> afterExecuteNormal<S>(String debugName, UniqueAsyncStopWatch stopwatch) async {
    await super._afterExecuteNormal(debugName, stopwatch);
    metrics.update(
        stopwatch.uuid.toString(),
        (metric) =>
            Metric(name: metric.name, unit: metric.unit, extra: metric.extra, value: stopwatch.elapsedMicroseconds));
  }

  @override
  UniqueAsyncStopWatch beforeExecuteSync<S>(String debugName, Object? hint) {
    final UuidValue uuid = generator.v4obj();
    UniqueAsyncStopWatch stopwatch = UniqueAsyncStopWatch(uuid);
    metrics.putIfAbsent(
        uuid.toString(),
        () => Metric(
            value: stopwatch.elapsedMicroseconds, name: debugName, extra: hint.toString(), unit: Unit.Microseconds));
    super._beforeExecuteSync(debugName, stopwatch);
    return stopwatch;
  }

  @override
  void afterExecuteSync<S>(String debugName, UniqueAsyncStopWatch stopwatch) {
    super._afterExecuteSync(debugName, stopwatch);
    metrics.update(
        stopwatch.uuid.toString(),
        (metric) =>
            Metric(name: metric.name, unit: metric.unit, extra: metric.extra, value: stopwatch.elapsedMicroseconds));
  }
}

class FlutterRustBridgeExampleBenchmarkSuitePlatformBench extends FlutterRustBridgeExampleBenchmarkSuitePlatform {
  final FlutterRustBridgeInterceptor<AsyncStopWatch> _interceptor;
  final log = Logger('FlutterRustBridgeExampleBenchmarkSuitePlatformBench');
  FlutterRustBridgeExampleBenchmarkSuitePlatformBench(ffi.DynamicLibrary dylib, this._interceptor) : super(dylib);
  FlutterRustBridgeInterceptor<AsyncStopWatch> get interceptor => _interceptor;
  @override
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> task) async {
    final String debugName = task.constMeta.debugName;
    final AsyncStopWatch stopwatch = await interceptor.beforeExecuteNormal(debugName, task.hint);
    final result = await super.executeNormal(task);
    await interceptor.afterExecuteNormal(debugName, stopwatch);
    return result;
  }

  @override
  S executeSync<S>(FlutterRustBridgeSyncTask task) {
    final String debugName = task.constMeta.debugName;
    final AsyncStopWatch stopwatch = interceptor.beforeExecuteSync(debugName, task.hint);
    final result = super.executeSync(task);
    interceptor.afterExecuteSync(debugName, stopwatch);
    return result;
  }

  Future<List<Metric>?> metrics() async {
    if (interceptor is FlutterRustBridgeInterceptorJson) {
      final FlutterRustBridgeInterceptorJson jsonInterceptor = interceptor as FlutterRustBridgeInterceptorJson;
      List<Metric> metrics = List.empty(growable: true);
      for (var e in jsonInterceptor.metrics.entries) {
        metrics.add(e.value);
      }
      return metrics;
    }
    return null;
  }
}

extension UnitToJsonExtension on Unit {
  String convertToJson() {
    switch (this) {
      case Unit.Microseconds:
        return 'μs';
      case Unit.Nanoseconds:
        return 'ns';
    }
  }
}
