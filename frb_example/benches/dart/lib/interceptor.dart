/// interceptor (platform-agnostic wrapper)
///
/// record time before and after executing async or sync functions over the bridge.
///
/// can also generate
/// a [continuous-benchmark](https://github.com/marketplace/actions/continuous-benchmark)-compliant json output on demand

import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:logging/logging.dart';
import 'bridge_definitions.dart';
import 'package:uuid/uuid.dart';
import 'interceptor.io.dart' if (dart.library.html) 'interceptor.web.dart';
import 'bridge_generated.io.dart' if (dart.library.html) 'bridge_generated.web.dart';

/// this is the custom platform, which contains the interceptor.
abstract class FlutterRustBridgeExampleBenchmarkSuitePlatformBenchBase<
    STDOUT extends FlutterRustBridgeInterceptorStdOut,
    JSON extends FlutterRustBridgeInterceptorJson> extends FlutterRustBridgeExampleBenchmarkSuitePlatform {
  FlutterRustBridgeExampleBenchmarkSuitePlatformBenchBase(ExternalLibrary dylib) : super(dylib);
  FlutterRustBridgeInterceptor<TimeWatch> get interceptor;

  @override
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> task) async {
    final String debugName = task.constMeta.debugName;
    final TimeWatch stopwatch = await interceptor.beforeExecuteNormal(debugName, task.hint);
    final result = await super.executeNormal(task);
    await interceptor.afterExecuteNormal(debugName, stopwatch);
    return result;
  }

  @override
  S executeSync<S>(FlutterRustBridgeSyncTask task) {
    final String debugName = task.constMeta.debugName;
    final TimeWatch stopwatch = interceptor.beforeExecuteSync(debugName, task.hint);
    final result = super.executeSync(task);
    interceptor.afterExecuteSync(debugName, stopwatch);
    return result;
  }

  List<Metric>? metrics() => interceptor is FlutterRustBridgeInterceptorJson
      ? (interceptor as FlutterRustBridgeInterceptorJson).metrics.values.toList()
      : null;
}

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

/// benchmark metric
abstract class TimeWatch {
  /// typically when the timer starts
  final int starts;

  /// typically when the timer ends
  int? ends;

  /// start timer
  void start();

  /// stop timer
  void stop();

  /// create a timer
  TimeWatch.create(this.starts);
}

/// unique benchmark metric
abstract class UniqueTimeWatch extends TimeWatch {
  /// allows for identifying same metric over time
  ///
  /// this way you can start a timer, save its value(s) somewhere,
  /// and record whenever you stop it anytime later on.
  late UuidValue uuid;
  UniqueTimeWatch.create(int starts) : super.create(starts);
}

abstract class FlutterRustBridgeInterceptor<T extends TimeWatch> {
  T create();
  Future<T> beforeExecuteNormal(String debugName, Object? hint) {
    return Future.sync(() {
      return create()..start();
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
    log.fine('(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts}$unit');
  }

  @override
  T beforeExecuteSync<S>(String debugName, Object? hint) {
    log.fine('(Dart) execute [$debugName] start');
    return super.beforeExecuteSync(debugName, hint);
  }

  @override
  void afterExecuteSync<S>(String debugName, T stopwatch) {
    super.afterExecuteSync(debugName, stopwatch);
    log.fine('(Dart) execute [$debugName] end delta_time=${stopwatch.ends! - stopwatch.starts}$unit');
  }
}

abstract class FlutterRustBridgeInterceptorJson<T extends UniqueTimeWatch> extends FlutterRustBridgeInterceptor<T> {
  Map<String, Metric> metrics = {};
  final Uuid generator = Uuid();
  final log = Logger('FlutterRustBridgeInterceptorJson');
  @override
  Future<T> beforeExecuteNormal(String debugName, Object? hint) async {
    final T stopwatch = await super.beforeExecuteNormal(debugName, hint);
    metrics.putIfAbsent(
        stopwatch.uuid.toString(), () => Metric(name: debugName, extra: hint.toString(), unit: Unit.Microseconds));
    return stopwatch;
  }

  @override
  Future<void> afterExecuteNormal<S>(String debugName, T stopwatch) async {
    await super.afterExecuteNormal(debugName, stopwatch);
    metrics.update(stopwatch.uuid.toString(),
        (metric) => Metric(name: metric.name, unit: metric.unit, extra: metric.extra, value: stopwatch.ends!));
  }

  @override
  T beforeExecuteSync<S>(String debugName, Object? hint) {
    T stopwatch = super.beforeExecuteSync(debugName, hint);
    metrics.putIfAbsent(stopwatch.uuid.toString(),
        () => Metric(value: stopwatch.ends, name: debugName, extra: hint.toString(), unit: Unit.Microseconds));
    return stopwatch;
  }

  @override
  void afterExecuteSync<S>(String debugName, T stopwatch) {
    super.afterExecuteSync(debugName, stopwatch);
    metrics.update(stopwatch.uuid.toString(),
        (metric) => Metric(name: metric.name, unit: metric.unit, extra: metric.extra, value: stopwatch.ends!));
  }
}
