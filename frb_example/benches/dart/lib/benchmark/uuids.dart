// Import BenchmarkBase class.
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import '../async_benchmark.dart';
import 'package:uuid/uuid.dart';
import '../ffi.io.dart' if (dart.library.html) '../ffi.web.dart';
import '../env/stub.dart';

// Create a new benchmark by extending BenchmarkBase
class TemplateBenchmark extends AsyncBenchmark {
  TemplateBenchmark()
      : super(
            name: 'Vec of UUIDs',
            warmUpTime: Duration(milliseconds: warmUpTime),
            measurementTime: Duration(
              milliseconds: measurementTime,
            ),
            sampleSize: sampleCount);

  late FlutterRustBridgeExampleBenchmarkSuiteImpl api;
  late List<UuidValue> uuids;

  static void main() {
    TemplateBenchmark().report();
  }

  // The benchmark code.
  @override
  Future<List<UuidValue>> run() async {
    return await api.handleUuids(ids: uuids);
  }

  // Not measured setup code executed prior to the benchmark runs.
  @override
  Future<void> setup() async {
    String path = dylibPath ?? "../../../target/release/libflutter_rust_bridge_example_benchmark_suite.dylib";
    print('flutter_rust_bridge benchmark uuids (dylibPath=$path)');
    await super.setup();
    return Future.sync(() {
      final uuid = Uuid();
      api = initializeBenchExternalLibrary(path);
      uuids = List<UuidValue>.generate(itemsCount, (_) => uuid.v4obj());
    });
  }

  // Not measured teardown code executed after the benchmark runs.
  @override
  Future<void> teardown() {
    return Future.value();
  }
}

void main() {
  // Run TemplateBenchmark
  TemplateBenchmark.main();
}
