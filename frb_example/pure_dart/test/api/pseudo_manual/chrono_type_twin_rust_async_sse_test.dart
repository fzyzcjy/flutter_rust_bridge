// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `chrono_type_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/chrono_type_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('DateTime<Utc>', () async {
    final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
    final resp = await datetimeUtcTwinRustAsyncSse(d: date);
    expect(resp.year, date.year);
    expect(resp.month, date.month);
    expect(resp.day, date.day);
    expect(resp.hour, date.hour);
    expect(resp.minute, date.minute);
    expect(resp.second, date.second);
    expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
    expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
  });

  test('DateTime<Local>', () async {
    final date = DateTime(2022, 09, 10, 20, 48, 53, 123, 456);
    final resp = await datetimeLocalTwinRustAsyncSse(d: date);
    expect(resp.year, date.year);
    expect(resp.month, date.month);
    expect(resp.day, date.day);
    expect(resp.hour, date.hour);
    expect(resp.minute, date.minute);
    expect(resp.second, date.second);
    expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
    expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
  });

  test('NaiveDateTime', () async {
    final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
    final resp = await naivedatetimeTwinRustAsyncSse(d: date);
    expect(resp.year, date.year);
    expect(resp.month, date.month);
    expect(resp.day, date.day);
    expect(resp.hour, date.hour);
    expect(resp.minute, date.minute);
    expect(resp.second, date.second);
    expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
    expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
  });
  test('Empty DateTime', () async {
    final resp = await optionalEmptyDatetimeUtcTwinRustAsyncSse(d: null);
    expect(resp, null);
  });

  test('Duration', () async {
    final d = Duration(hours: 4);
    final resp = await durationTwinRustAsyncSse(d: d);
    expect(resp.inHours, d.inHours);
  });

  test('List<Duration>', () async {
    final expected = [
      Duration(days: 1),
      Duration(days: 10),
      Duration(days: 100),
      Duration(milliseconds: 333),
      if (!kIsWeb) Duration(microseconds: 333)
    ];
    final now = DateTime.now();
    final durations = await handleTimestampsTwinRustAsyncSse(
      timestamps: expected.map(now.subtract).toList(),
      epoch: now,
    );
    expect(durations, expected);
  });

  test('List<DateTime>', () async {
    final expected = [
      Duration(days: 3),
      Duration(hours: 2),
      Duration(seconds: 1),
      Duration(milliseconds: 500),
      if (!kIsWeb) Duration(microseconds: 400)
    ];
    final now = DateTime.now();
    final result =
        await handleDurationsTwinRustAsyncSse(durations: expected, since: now);
    expect(result, expected.map(now.subtract));
  });

  test('Combined Chrono types', () async {
    final test = await testChronoTwinRustAsyncSse();
    expect(test.dt!.millisecondsSinceEpoch, 1631297333000);
    expect(test.dt2!.millisecondsSinceEpoch, 1631297333000);
    expect(test.du, Duration(hours: 4));
  });

  test('combined chrono types precise', () async {
    final datetime_1 = DateTime.utc(2002, 02, 23, 12, 13, 55);
    final datetime_2 = DateTime.utc(1800, 01, 23, 12, 56, 25);
    final duration = Duration(hours: 4);

    final result = await testPreciseChronoTwinRustAsyncSse();

    expect(
        result.dt!.millisecondsSinceEpoch, datetime_1.millisecondsSinceEpoch);
    expect(
        result.dt2!.millisecondsSinceEpoch, datetime_2.millisecondsSinceEpoch);
    expect(result.du!.inHours, duration.inHours);
  });

  test('nested chrono types', () async {
    const duration = Duration(hours: 4);
    final naive = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
    final local = DateTime.now();
    final utc = DateTime.now().toUtc();
    final difference = await howLongDoesItTakeTwinRustAsyncSse(
        mine: FeatureChronoTwinRustAsyncSse(
            utc: utc, local: local, duration: duration, naive: naive));
    debugPrint('$difference');
  });
}
