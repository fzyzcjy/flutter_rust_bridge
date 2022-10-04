import 'dart:async';
import 'dart:html';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

import 'ffi.web.dart';
import 'interceptor.dart';
export 'interceptor.dart';
export 'bridge_generated.dart';

class FlutterRustBridgeExampleBenchmarkSuitePlatformBench
    extends FlutterRustBridgeExampleBenchmarkSuitePlatformBenchBase<FlutterRustBridgeInterceptorStdOutWasm,
        FlutterRustBridgeInterceptorJsonWasm> {
  final FlutterRustBridgeInterceptor _interceptor;
  FlutterRustBridgeExampleBenchmarkSuitePlatformBench(FutureOr<WasmModule> dylib, bool useJSON)
      : _interceptor = useJSON
            ? FlutterRustBridgeInterceptorJsonWasm() as FlutterRustBridgeInterceptor<TimeWatch>
            : FlutterRustBridgeInterceptorStdOutWasm(),
        super(dylib as ExternalLibrary);
  @override
  FlutterRustBridgeInterceptor<TimeWatch> get interceptor => _interceptor;
}

class FlutterRustBridgeInterceptorStdOutWasm extends FlutterRustBridgeInterceptorStdOut<WindowPerformance> {
  @override
  WindowPerformance create() {
    return WindowPerformance();
  }

  @override
  String get unit => 'ms';
}

class FlutterRustBridgeInterceptorJsonWasm extends FlutterRustBridgeInterceptorJson<UniqueWindowPerformance> {
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
    starts = window.performance.now().round();
  }

  @override
  void stop() {
    ends = window.performance.now().round();
  }
}

class UniqueWindowPerformance extends WindowPerformance implements UniqueTimeWatch {
  late UuidValue _uuid;
  UniqueWindowPerformance.create() {
    Uuid generator = Uuid();
    _uuid = generator.v4obj();
  }

  @override
  UuidValue get uuid => _uuid;
}

class FlutterRustBridgeExampleBenchmarkSuiteWireBench extends FlutterRustBridgeExampleBenchmarkSuiteWire {
  FlutterRustBridgeExampleBenchmarkSuiteWireBench(FutureOr<WasmModule> module)
      : super(WasmModule.cast<FlutterRustBridgeExampleBenchmarkSuiteWasmModule>(module));
}
