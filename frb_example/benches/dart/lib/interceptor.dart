import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:logging/logging.dart';
import 'bridge_definitions.dart';
import 'package:uuid/uuid.dart';
import 'interceptor.io.dart' if (dart.library.html) 'interceptor.web.dart';

class FlutterRustBridgeExampleBenchmarkSuiteImplBench extends FlutterRustBridgeExampleBenchmarkSuiteImpl {
  final FlutterRustBridgeExampleBenchmarkSuitePlatformBench platform;
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench(ExternalLibrary dylib, {bool? useJSON}) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench.raw(
          FlutterRustBridgeExampleBenchmarkSuitePlatformBench(dylib, useJSON ?? false));

  /// Only valid on web/WASM platforms.
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench.wasm(FutureOr<WasmModule> module, {bool? useJSON}) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench(module as ExternalLibrary, useJSON: useJSON ?? false);

  @override
  FlutterRustBridgeExampleBenchmarkSuiteImplBench.raw(this.platform) : super.raw((platform));

  Future<List<Metric>?> dartMetrics() async {
    return platform.metrics();
  }
}

abstract class TimeWatch {
  int? starts;
  int? ends;
  void start();
  void stop();
  TimeWatch.create();
}

abstract class UniqueTimeWatch extends TimeWatch {
  UniqueTimeWatch.create() : super.create();
  UuidValue get uuid;
}

abstract class FlutterRustBridgeInterceptor<T extends TimeWatch> {
  T create();
  Future<T> beforeExecuteNormal(String debugName, Object? hint) {
    return Future.sync(() {
      final T stopwatch = create();
      stopwatch.start();
      return stopwatch;
    });
  }

  Future<void> afterExecuteNormal<S>(String debugName, T stopwatch) {
    return Future.sync(() {
      stopwatch.stop();
    });
  }

  T beforeExecuteSync<S>(String debugName, Object? hint) {
    final T stopwatch = create();
    stopwatch.start();
    return stopwatch;
  }

  void afterExecuteSync<S>(String debugName, T stopwatch) {
    stopwatch.stop();
  }
}

abstract class FlutterRustBridgeInterceptorStdOut<T extends TimeWatch> extends FlutterRustBridgeInterceptor<T> {
  final log = Logger('FlutterRustBridgeInterceptorStdOut');
  String get unit;
  @override
  Future<T> beforeExecuteNormal(String debugName, Object? hint) async {
    log.fine('(Dart) hint [$debugName] => ${hint.toString()}');
    log.fine('(Dart) execute [$debugName] start');
    return await super.beforeExecuteNormal(debugName, hint);
  }

  @override
  Future<void> afterExecuteNormal<S>(String debugName, T stopwatch) async {
    await super.afterExecuteNormal(debugName, stopwatch);
    log.fine('(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts!}$unit');
  }

  @override
  T beforeExecuteSync<S>(String debugName, Object? hint) {
    log.fine('(Dart) execute [$debugName] start');
    return super.beforeExecuteSync(debugName, hint);
  }

  @override
  void afterExecuteSync<S>(String debugName, T stopwatch) {
    super.afterExecuteSync(debugName, stopwatch);
    log.fine('(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts!}$unit');
  }
}

abstract class FlutterRustBridgeInterceptorJson<T extends UniqueTimeWatch> extends FlutterRustBridgeInterceptor<T> {
  Map<String, Metric> metrics = {};
  final Uuid generator = Uuid();
  final log = Logger('FlutterRustBridgeInterceptorJson');
  @override
  Future<T> beforeExecuteNormal(String debugName, Object? hint) async {
    T stopwatch = create();
    metrics.putIfAbsent(
        stopwatch.uuid.toString(), () => Metric(name: debugName, extra: hint.toString(), unit: Unit.Microseconds));
    await super.beforeExecuteNormal(debugName, stopwatch);
    return stopwatch;
  }

  @override
  Future<void> afterExecuteNormal<S>(String debugName, T stopwatch) async {
    await super.afterExecuteNormal(debugName, stopwatch);
    metrics.update(stopwatch.uuid.toString(),
        (metric) => Metric(name: metric.name, unit: metric.unit, extra: metric.extra, value: stopwatch.ends));
  }

  @override
  T beforeExecuteSync<S>(String debugName, Object? hint) {
    final UuidValue uuid = generator.v4obj();
    T stopwatch = create();
    metrics.putIfAbsent(uuid.toString(),
        () => Metric(value: stopwatch.ends, name: debugName, extra: hint.toString(), unit: Unit.Microseconds));
    super.beforeExecuteSync(debugName, stopwatch);
    return stopwatch;
  }

  @override
  void afterExecuteSync<S>(String debugName, T stopwatch) {
    super.afterExecuteSync(debugName, stopwatch);
    metrics.update(stopwatch.uuid.toString(),
        (metric) => Metric(name: metric.name, unit: metric.unit, extra: metric.extra, value: stopwatch.ends));
  }
}
