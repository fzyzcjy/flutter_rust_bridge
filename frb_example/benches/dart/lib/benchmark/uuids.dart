// Import BenchmarkBase class.
import 'package:flutter_rust_bridge_benchmark/env/stub.dart';
import '../async_benchmark.dart';
import 'package:uuid/uuid.dart';
import '../ffi.io.dart' if (dart.library.html) '../ffi.web.dart';

// Create a new benchmark by extending BenchmarkBase
class TemplateBenchmark extends AsyncBenchmark {
  TemplateBenchmark(String dylibPath)
      : super(
            name: 'Vec of UUIDs',
            dylibPath: dylibPath,
            warmUpTime: Duration(milliseconds: warmUpTime),
            measurementTime: Duration(
              milliseconds: measurementTime,
            ),
            sampleSize: sampleCount);

  late FlutterRustBridgeExampleBenchmarkSuiteImpl api;
  late List<UuidValue> uuids;

  static Future<void> main(String dylibPath) async {
    await TemplateBenchmark(dylibPath).report();
  }

  // The benchmark code.
  @override
  Future<List<UuidValue>> run() async {
    return await api.handleUuids(ids: uuids);
  }

  // Not measured setup code executed prior to the benchmark runs.
  @override
  Future<void> setup() async {
    print('flutter_rust_bridge benchmark uuids (dylibPath=$dylibPath)');
    await super.setup();
    return Future.sync(() {
      final uuid = Uuid();
      api = initializeBenchExternalLibrary(dylibPath);
      uuids = List<UuidValue>.generate(itemsCount, (_) => uuid.v4obj());
    });
  }

  // Not measured teardown code executed after the benchmark runs.
  @override
  Future<void> teardown() {
    return Future.value();
  }
}

void main(List<String> args) async {
  final String dylibPath = args[0];
  // Run TemplateBenchmark
  await TemplateBenchmark.main(dylibPath);
}
