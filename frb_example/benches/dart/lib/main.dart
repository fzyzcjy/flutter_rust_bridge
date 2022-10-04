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
  final api = initializeBenchExternalLibrary(dylibPath, useJSON: useJSON);

  final Uuid uuid = Uuid();
  final thousandUuids = List<UuidValue>.generate(1000, (index) => uuid.v4obj(), growable: false);
  final thousandStrings = List<String>.generate(1000, (index) => getRandomString(uuidSizeInBytes), growable: false);
  final hundredThousandUuids = List<UuidValue>.generate(100000, (index) => uuid.v4obj(), growable: false);
  final hundredThousandStrings =
      List<String>.generate(100000, (index) => getRandomString(uuidSizeInBytes), growable: false);
  await api.handleUuids(ids: thousandUuids, hint: 'reverse 1,000 uuids');
  await api.handleStrings(strings: thousandStrings, hint: 'reverse 1,000 strings');
  await api.handleUuidsConvertToStrings(ids: thousandUuids, hint: '1,000 uuids converted to strings');
  await api.handleUuids(ids: hundredThousandUuids, hint: 'reverse 10,000 uuids');
  await api.handleStrings(strings: hundredThousandStrings, hint: 'reverse 10,000 strings');
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

String jsonEncodeMetric(Metric metric, String language) => json.encode({
      'name': metric.extra != null ? '$language:${metric.name}:${metric.extra}' : '$language:${metric.name}',
      'value': metric.value,
      'unit': unitToString(metric.unit),
    });

// vim:expandtab:ts=2:sw=2
