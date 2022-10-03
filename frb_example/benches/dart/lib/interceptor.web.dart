import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';

import 'ffi.web.dart';
export 'bridge_generated.dart';
import 'interceptor.dart';

class FlutterRustBridgeExampleBenchmarkSuitePlatformBench extends FlutterRustBridgeExampleBenchmarkSuitePlatform {
  final FlutterRustBridgeInterceptor<AsyncStopWatch> _interceptor;
  FlutterRustBridgeExampleBenchmarkSuitePlatformBench(FutureOr<WasmModule> dylib, bool useJSON)
      : _interceptor = useJSON
            ? FlutterRustBridgeInterceptorJson()
            : FlutterRustBridgeInterceptorStdOut() as FlutterRustBridgeInterceptor<AsyncStopWatch>,
        super(dylib);
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

class FlutterRustBridgeExampleBenchmarkSuiteWireBench extends FlutterRustBridgeExampleBenchmarkSuiteWire {
  FlutterRustBridgeExampleBenchmarkSuiteWireBench(FutureOr<WasmModule> module)
      : super(WasmModule.cast<FlutterRustBridgeExampleBenchmarkSuiteWasmModule>(module));
}
