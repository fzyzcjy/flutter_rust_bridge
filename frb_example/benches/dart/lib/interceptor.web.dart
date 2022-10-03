import 'dart:async';
import 'dart:html';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';
import 'package:uuid/uuid.dart';

import 'ffi.web.dart';
import 'interceptor.dart';
export 'interceptor.dart';
export 'bridge_generated.dart';

class FlutterRustBridgeExampleBenchmarkSuitePlatformBench
    extends FlutterRustBridgeExampleBenchmarkSuitePlatform {
  final FlutterRustBridgeInterceptor<TimeWatch> _interceptor;
  FlutterRustBridgeExampleBenchmarkSuitePlatformBench(
      FutureOr<WasmModule> dylib, bool useJSON)
      : _interceptor = useJSON
            ? FlutterRustBridgeInterceptorJsonWasm()
                as FlutterRustBridgeInterceptor<TimeWatch>
            : FlutterRustBridgeInterceptorStdOutWasm(),
        super(dylib);
  FlutterRustBridgeInterceptor<TimeWatch> get interceptor => _interceptor;

  @override
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> task) async {
    final String debugName = task.constMeta.debugName;
    final TimeWatch stopwatch =
        await interceptor.beforeExecuteNormal(debugName, task.hint);
    final result = await super.executeNormal(task);
    await interceptor.afterExecuteNormal(debugName, stopwatch);
    return result;
  }

  @override
  S executeSync<S>(FlutterRustBridgeSyncTask task) {
    final String debugName = task.constMeta.debugName;
    final TimeWatch stopwatch =
        interceptor.beforeExecuteSync(debugName, task.hint);
    final result = super.executeSync(task);
    interceptor.afterExecuteSync(debugName, stopwatch);
    return result;
  }

  Future<List<Metric>?> metrics() async {
    if (interceptor is FlutterRustBridgeInterceptorJson) {
      final FlutterRustBridgeInterceptorJson jsonInterceptor =
          interceptor as FlutterRustBridgeInterceptorJson;
      List<Metric> metrics = List.empty(growable: true);
      for (var e in jsonInterceptor.metrics.entries) {
        metrics.add(e.value);
      }
      return metrics;
    }
    return null;
  }
}

class FlutterRustBridgeInterceptorStdOutWasm
    extends FlutterRustBridgeInterceptorStdOut<WindowPerformance> {
  @override
  WindowPerformance create() {
    return WindowPerformance();
  }

  @override
  String get unit => 'ms';
}

class FlutterRustBridgeInterceptorJsonWasm
    extends FlutterRustBridgeInterceptorJson<UniqueWindowPerformance> {
  @override
  UniqueWindowPerformance create() {
    return UniqueWindowPerformance.create();
  }
}

class WindowPerformance implements TimeWatch {
  @override
  int? starts;
  @override
  int? ends;
  @override
  void start() {
    print('before setting starts');
    starts = window.performance.now().round();
    print('after setting starts $starts');
  }

  @override
  void stop() {
    ends = window.performance.now().round();
  }
}

class UniqueWindowPerformance extends WindowPerformance
    implements UniqueTimeWatch {
  late UuidValue _uuid;
  UniqueWindowPerformance.create() {
    Uuid generator = Uuid();
    _uuid = generator.v4obj();
  }

  @override
  UuidValue get uuid => _uuid;
}

class FlutterRustBridgeExampleBenchmarkSuiteWireBench
    extends FlutterRustBridgeExampleBenchmarkSuiteWire {
  FlutterRustBridgeExampleBenchmarkSuiteWireBench(FutureOr<WasmModule> module)
      : super(WasmModule.cast<FlutterRustBridgeExampleBenchmarkSuiteWasmModule>(
            module));
}
