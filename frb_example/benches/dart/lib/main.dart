import 'dart:convert';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';
import './seeder.dart';
import 'package:logging/logging.dart';
import 'package:uuid/uuid.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';

void main(List<String> args) async {
  String dylibPath = args[0];
  // on web, env useJSON is unset, but server will send a --json cli arg instead
  // this is because WASM cannot be fed any env var, but happily takes cli args.
  final allowJSON = useJSON || args.contains('--json');
  final log = Logger('frb_example/benches');
  if (allowJSON) {
    Logger.root.level = Level.INFO;
  } else {
    Logger.root.level = Level.ALL;
  }
  Logger.root.onRecord.listen((record) {
    print('${record.level.name}: ${record.time}: ${record.message}');
  });
  log.fine('flutter_rust_bridge_benchmark_suite example program start (dylibPath=$dylibPath)');
  log.finer('construct api');
  if (isSilicon) {
    log.warning("⚠️  [env] SILICON: uses open method for dynamic library");
  }
  if (useJSON) {
    log.info("🗃️  [env] JSON: switch to json output");
  }
  if (isWeb && !allowJSON) {
    log.warning("⚠️  on Web, Rust logs to stdout are redirected to browser's console (or simply choose JSON)");
  }
  final api = initializeBenchExternalLibrary(dylibPath, useJSON: allowJSON);

  // here freely do as much setup as you need,
  // since only calls to bridge-wired functions will record any metric
  final List<UuidValue> thousandUuids = seed(UuidSeeder(), just_1000);
  final List<String> thousandStrings = seed(StringSeeder(), just_1000);
  final List<UuidValue> hundredThousandUuids = seed(UuidSeeder(), just_100_000);
  final List<String> hundredThousandStrings = seed(StringSeeder(), just_100_000);

  // real game starts here
  // --- (benchmarks:start) (avoid any initialization or computation)
  // let's get rid of sync first
  // lest stdout's output be intermingled
  // (otherwise feel free to open a PR for opentelemetry)
  api.handleSyncBool(input: false, hint: 'bool');
  api.handleSyncI16(input: 42, hint: 'sync i16');
  api.handleSyncF32(input: 42, hint: 'sync f32');
  api.handleSyncF64(input: 42, hint: 'sync f64');
  api.handleSyncI32(input: 42, hint: 'sync i32');
  api.handleSyncI64(input: 42, hint: 'sync i64');
  api.handleSyncString(input: "I'm a sync string", hint: 'sync string');

  await api.handleBool(input: false, hint: 'bool');
  await api.handleI16(input: 42, hint: 'i16');
  await api.handleF32(input: 42, hint: 'f32');
  await api.handleF64(input: 42, hint: 'f64');
  await api.handleI32(input: 42, hint: 'i32');
  await api.handleI64(input: 42, hint: 'i64');
  await api.handleString(input: "Not a sync string", hint: 'string');

  await api.handleUuids(ids: thousandUuids, hint: 'reverse 1,000 uuids');
  await api.handleStrings(strings: thousandStrings, hint: 'reverse 1,000 strings');
  await api.handleUuidsConvertToStrings(ids: thousandUuids, hint: '1,000 uuids converted to strings');
  await api.handleUuids(ids: hundredThousandUuids, hint: 'reverse 10,000 uuids');
  await api.handleStrings(strings: hundredThousandStrings, hint: 'reverse 10,000 strings');
  await api.handleUuidsConvertToStrings(ids: hundredThousandUuids, hint: '10,000 uuids converted to strings');

  // feel free to benchmark (many) more method(s), as you fancy !

  // --- (benchmarks:end)

  // benchmarks just finished, let's collect metrics
  if (allowJSON) {
    final dartMetrics = await api.dartMetrics() ?? List.empty(growable: false);
    final rustMetrics = await api.rustMetrics();
    if (rustMetrics.length != dartMetrics.length) {
      throw Exception('metrics on both sides should be of same length');
    }
    final int count = dartMetrics.length;
    for (var i = 0; i < count; i++) {
      final dartMetric = dartMetrics[i];
      final rustMetric = rustMetrics[i];
      if (dartMetric.name != rustMetric.name) {
        throw Exception('metric should come in the same order');
      }
      // manually tie Rust call to its parent Dart call (until 'hint' can be sent to Rust)
      rustMetric.extra = dartMetric.extra;
    }
    final dartMetricsJson = dartMetrics.map((e) => jsonEncodeMetric(e, 'dart'));
    final rustMetricsJson = rustMetrics.map((e) => jsonEncodeMetric(e, 'rust'));
    print(List.from(dartMetricsJson)..addAll(rustMetricsJson));
  }
}

String unitToString(Unit unit) => unit == Unit.Microseconds
    ? 'μs'
    : unit == Unit.Milliseconds
        ? 'ms'
        : 'ns';

/// metric encoded as json for [continuous-benchmark](https://github.com/marketplace/actions/continuous-benchmark)
///
/// naming convention allows for outputs to be harmoniously grouped by default
///
/// whenever a wired function is called:
/// - outer dart function call over FFI execution time is recorded
/// - inner rust function call from FFI execution time is recorded
/// outputs are then naturally grouped based on their name.
String jsonEncodeMetric(Metric metric, String language) => json.encode({
      'name': metric.extra != null ? '${metric.name}:${metric.extra}:$language' : '${metric.name}:$language',
      'value': metric.value,
      'unit': unitToString(metric.unit),
    });

// vim:expandtab:ts=2:sw=2
