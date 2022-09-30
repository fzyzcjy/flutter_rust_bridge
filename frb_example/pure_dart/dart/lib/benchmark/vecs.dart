// Import BenchmarkBase class.
import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:uuid/uuid.dart';
import '../ffi.io.dart' if (dart.library.html) '../ffi.web.dart';

// Create a new benchmark by extending BenchmarkBase
class TemplateBenchmark extends AsyncBenchmarkBase {
  TemplateBenchmark() : super('Vec of UUIDs');

  late FlutterRustBridgeExampleSingleBlockTestImpl api;
  late List<UuidValue> uuids;

  static void main() {
    TemplateBenchmark().report();
  }

  // The benchmark code.
  @override
  Future<void> run() async {
    final _ = await api.benchHandleUuids(ids: uuids);
  }

  // Not measured setup code executed prior to the benchmark runs.
  @override
  Future<void> setup() {
    return Future.sync(() {
      String dylibPath = String.fromEnvironment("DYLIB_PATH",
          defaultValue:
              "../../../target/release/libflutter_rust_bridge_example.dylib");
      int sampleCount =
          int.fromEnvironment("SAMPLE_COUNT", defaultValue: 10000);
      print(
          'flutter_rust_bridge benchmark $sampleCount uuids (dylibPath=$dylibPath)');
      print('setup');
      final uuid = Uuid();
      api = initializeExternalLibrary(dylibPath);
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
