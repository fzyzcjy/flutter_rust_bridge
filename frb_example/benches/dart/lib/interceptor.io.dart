/// interceptor (on native platforms)
///
/// underlying timer is based on [Stopwatch](https://api.dart.dev/dev/2.8.0-dev.7.0/dart-core/Stopwatch-class.html)
///
/// mostly a lot of boilerplate so that dart agrees to compile.

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
  final int starts = 0;
  @override
  int? ends;

  @override
  void stop() {
    super.stop();
    ends = elapsedMicroseconds;
  }
}

class UniqueAsyncStopWatch extends AsyncStopWatch implements UniqueTimeWatch {
  @override
  UuidValue uuid;
  UniqueAsyncStopWatch.create()
      : uuid = Uuid().v4obj(),
        super();
}

class FlutterRustBridgeExampleBenchmarkSuiteWireBench extends FlutterRustBridgeExampleBenchmarkSuiteWire {
  FlutterRustBridgeExampleBenchmarkSuiteWireBench(ffi.DynamicLibrary dynamicLibrary) : super(dynamicLibrary);
}
