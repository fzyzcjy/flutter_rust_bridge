import 'dart:ffi' as ffi;

import 'package:uuid/uuid.dart';
import 'ffi.io.dart';
import 'interceptor.dart';
export 'interceptor.dart';
export 'bridge_generated.dart';

class FlutterRustBridgeExampleBenchmarkSuitePlatformBench
    extends FlutterRustBridgeExampleBenchmarkSuitePlatformBenchBase<FlutterRustBridgeInterceptorStdOutIO,
        FlutterRustBridgeInterceptorJsonIO> {
  final FlutterRustBridgeInterceptor _interceptor;
  FlutterRustBridgeExampleBenchmarkSuitePlatformBench(ffi.DynamicLibrary dylib, bool useJSON)
      : _interceptor = useJSON
            ? FlutterRustBridgeInterceptorJsonIO() as FlutterRustBridgeInterceptor<TimeWatch>
            : FlutterRustBridgeInterceptorStdOutIO(),
        super(dylib);
  @override
  FlutterRustBridgeInterceptor<TimeWatch> get interceptor => _interceptor;
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
    starts = 0;
    super.start();
  }

  @override
  void stop() {
    super.stop();
    ends = elapsedMicroseconds;
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
