import 'dart:async';
import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_definitions.dart';
import 'ffi.io.dart';
import 'interceptor.dart';
export 'interceptor.dart';
export 'bridge_generated.dart';

class FlutterRustBridgeExampleBenchmarkSuitePlatformBench extends FlutterRustBridgeExampleBenchmarkSuitePlatform {
  final FlutterRustBridgeInterceptor<TimeWatch> _interceptor;
  FlutterRustBridgeExampleBenchmarkSuitePlatformBench(ffi.DynamicLibrary dylib, bool useJSON)
      : _interceptor = useJSON
            ? FlutterRustBridgeInterceptorJsonIO() as FlutterRustBridgeInterceptor<TimeWatch>
            : FlutterRustBridgeInterceptorStdOutIO(),
        super(dylib);
  FlutterRustBridgeInterceptor<TimeWatch> get interceptor => _interceptor;

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

class FlutterRustBridgeInterceptorStdOutIO extends FlutterRustBridgeInterceptorStdOut<AsyncStopWatch> {
  @override
  AsyncStopWatch create() {
    return AsyncStopWatch();
  }

  @override
  String get unit => 'μs';
}

class FlutterRustBridgeInterceptorJsonIO extends FlutterRustBridgeInterceptorJson<UniqueAsyncStopWatch> {
  @override
  UniqueAsyncStopWatch create() {
    return UniqueAsyncStopWatch.create();
  }
}

class AsyncStopWatch extends Stopwatch implements TimeWatch {
  @override
  int? starts;
  @override
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

class UniqueAsyncStopWatch extends AsyncStopWatch implements UniqueTimeWatch {
  late UuidValue _uuid;
  UniqueAsyncStopWatch.create() {
    Uuid generator = Uuid();
    _uuid = generator.v4obj();
  }

  @override
  UuidValue get uuid => _uuid;
}

class FlutterRustBridgeExampleBenchmarkSuiteWireBench extends FlutterRustBridgeExampleBenchmarkSuiteWire {
  FlutterRustBridgeExampleBenchmarkSuiteWireBench(ffi.DynamicLibrary dynamicLibrary) : super(dynamicLibrary);
}
