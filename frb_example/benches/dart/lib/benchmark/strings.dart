// Import BenchmarkBase class.
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../async_benchmark.dart';
import 'package:flutter_rust_bridge_benchmark/env/stub.dart';
import 'package:flutter_rust_bridge_benchmark/utils.dart';
import '../ffi.io.dart' if (dart.library.html) '../ffi.web.dart';

// Create a new benchmark by extending BenchmarkBase
class TemplateBenchmark extends AsyncBenchmark {
  TemplateBenchmark(String dylibPath)
      : super(
            name: 'Vec of Strings',
            dylibPath: dylibPath,
            warmUpTime: Duration(milliseconds: warmUpTime),
            measurementTime: Duration(
              milliseconds: measurementTime,
            ),
            sampleSize: sampleCount);

  late FlutterRustBridgeExampleBenchmarkSuiteImpl api;
  late List<String> strings;

  static Future<void> main(String dylibPath) async {
    await TemplateBenchmark(dylibPath).report();
  }

  // The benchmark code.
  @override
  Future<List<String>> run() async {
    return await api.handleStrings(strings: strings);
  }

  // Not measured setup code executed prior to the benchmark runs.
  @override
  Future<void> setup() async {
    print('flutter_rust_bridge benchmark strings (dylibPath=$dylibPath)');
    await super.setup();
    return Future.sync(() {
      api = initializeBenchExternalLibrary(dylibPath);
      strings = List<String>.generate(itemsCount, (_) => getRandomString(uuidSizeInBytes));
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
