import 'dart:convert';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';
import 'package:logging/logging.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'package:flutter_rust_bridge_benchmark/utils.dart';
import 'package:uuid/uuid.dart';

void main(List<String> args) async {
  String dylibPath = args[0];
  final log = Logger('frb_example/benches');
  String? extra = args.length > 1 ? args[1] : null;
  final bool useJSON = extra == '--json';
  if (useJSON) {
    Logger.root.level = Level.INFO;
  } else {
    Logger.root.level = Level.ALL;
  }
  Logger.root.onRecord.listen((record) {
    print('${record.level.name}: ${record.time}: ${record.message}');
  });
  log.fine('flutter_rust_bridge_benchmark_suite example program start (dylibPath=$dylibPath)');
  log.finer('construct api');
  if (extra != '--json') {
    log.warning('CLI extra argument $extra is ignored (expected --json, or none)');
  }
  final api = initializeBenchExternalLibrary(dylibPath, useJSON: useJSON);

  final Uuid uuid = Uuid();
  final thousandUuids = List<UuidValue>.generate(1000, (index) => uuid.v4obj(), growable: false);
  final thousandStrings = List<String>.generate(1000, (index) => getRandomString(uuidSizeInBytes), growable: false);
  final hundredThousandUuids = List<UuidValue>.generate(100000, (index) => uuid.v4obj(), growable: false);
  final hundredThousandStrings =
      List<String>.generate(100000, (index) => getRandomString(uuidSizeInBytes), growable: false);
  await api.handleUuids(ids: thousandUuids, hint: '1,000 uuids');
  await api.handleStrings(strings: thousandStrings, hint: '1,000 strings');
  await api.handleUuidsConvertToStrings(ids: thousandUuids, hint: '1,000 uuids converted to strings');
  await api.handleUuids(ids: hundredThousandUuids, hint: '10,000 uuids');
  await api.handleStrings(strings: hundredThousandStrings, hint: '10,000 strings');
  await api.handleUuidsConvertToStrings(ids: hundredThousandUuids, hint: '10,000 uuids converted to strings');

  if (useJSON) {
    final dartMetrics = await api.dartMetrics() ?? List.empty(growable: false);
    final rustMetrics = await api.rustMetrics();
    assert(rustMetrics.length == dartMetrics.length);
    final int count = rustMetrics.length;
    for (var i = 0; i < count; i++) {
      final dartMetric = dartMetrics[i];
      final rustMetric = rustMetrics[i];
      if (dartMetric.name != rustMetric.name) {
        throw Exception('metric should come in the same order');
      }
      rustMetric.extra = dartMetric.extra;
    }
    final dartMetricsJson = dartMetrics.map((e) => json.encode({
          'name': e.extra != null ? 'dart:${e.name}:${e.extra}' : 'dart:${e.name}',
          'value': e.value,
          'unit': unitToString(e.unit),
        }));
    final rustMetricsJson = rustMetrics.map((e) => json.encode({
          'name': e.extra != null ? 'rust:${e.name}:${e.extra}' : 'rust:${e.name}',
          'value': e.value,
          'unit': unitToString(e.unit),
        }));
    print(List.from(dartMetricsJson)..addAll(rustMetricsJson));
  }
}

String unitToString(Unit unit) => unit == Unit.Microseconds
    ? 'μs'
    : unit == Unit.Milliseconds
        ? 'ms'
        : 'ns';

// vim:expandtab:ts=2:sw=2
