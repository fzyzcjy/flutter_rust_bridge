// Import BenchmarkBase class.
import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:uuid/uuid.dart';
import '../ffi.io.dart' if (dart.library.html) '../ffi.web.dart';
import 'dart:io' if (dart.library.html) 'dart.html';

// Create a new benchmark by extending BenchmarkBase
class TemplateBenchmark extends AsyncBenchmarkBase {
  TemplateBenchmark() : super('Vec of UUIDs');

  late FlutterRustBridgeExampleBenchmarkSuiteImpl api;
  late List<UuidValue> uuids;

  static void main() {
    TemplateBenchmark().report();
  }

  // The benchmark code.
  @override
  Future<void> run() async {
    // ignore: no_leading_underscores_for_local_identifiers
    final _ = await api.handleUuids(ids: uuids);
  }

  // Not measured setup code executed prior to the benchmark runs.
  @override
  Future<void> setup() {
    return Future.sync(() {
      String dylibPath = Platform.environment["DYLIB_PATH"] ??
          "../../../target/release/libflutter_rust_bridge_example_benchmark_suite.dylib";
      int sampleCount = int.fromEnvironment("SAMPLE_COUNT", defaultValue: 1000);
      print('flutter_rust_bridge benchmark $sampleCount uuids (dylibPath=$dylibPath)');
      print('setup');
      final uuid = Uuid();
      api = initializeBenchExternalLibrary(dylibPath);
      uuids = List<UuidValue>.generate(sampleCount, (_) => uuid.v4obj());
    });
  }

  // Not measured teardown code executed after the benchmark runs.
  @override
  Future<void> teardown() {
    return Future.value();
  }

  // To opt into the reporting the time per run() instead of per 10 run() calls.
  //@override
  //void exercise() => run();
}

void main() {
  // Run TemplateBenchmark
  TemplateBenchmark.main();
}
