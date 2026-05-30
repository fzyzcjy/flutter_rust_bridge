// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/chrono_type.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('DateTime<Utc>', () async {
    final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 0);
    final resp = await datetimeUtcTwinNormal(d: date);
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
    final date = DateTime(2022, 09, 10, 20, 48, 53, 123, 0);
    final resp = await datetimeLocalTwinNormal(d: date);
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
    final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 0);
    final resp = await naivedatetimeTwinNormal(d: date);
    expect(resp.year, date.year);
    expect(resp.month, date.month);
    expect(resp.day, date.day);
    expect(resp.hour, date.hour);
    expect(resp.minute, date.minute);
    expect(resp.second, date.second);
    expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
    expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
  });

  test('NaiveDate', () async {
    final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 0);
    final resp = await naivedateTwinNormal(d: date);
    expect(resp.isUtc, true);
    expect(resp.year, date.year);
    expect(resp.month, date.month);
    expect(resp.day, date.day);
    expect(resp.hour, 0);
    expect(resp.minute, 0);
    expect(resp.second, 0);
    expect(resp.millisecond, 0);
    expect(resp.microsecond, 0);
  });

  test('Empty DateTime', () async {
    final resp = await optionalEmptyDatetimeUtcTwinNormal(d: null);
    expect(resp, null);
  });

  test('Duration', () async {
    final d = Duration(hours: 4);
    final resp = await durationTwinNormal(d: d);
    expect(resp.inHours, d.inHours);
  });

  test('std::time::Duration', () async {
    final d = Duration(hours: 4);
    final resp = await stdTimeDurationTwinNormal(d: d);
    expect(resp, d);
  });

  test('std::time::SystemTime', () async {
    final date =
        DateTime.fromMillisecondsSinceEpoch(1631297333000, isUtc: true);
    final resp = await stdTimeSystemTimeTwinNormal(d: date);
    expect(resp.isUtc, true);
    expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
  });

  test('std::time::SystemTime before UNIX epoch',
      skip: skipWeb('wasm SystemTime does not support pre-epoch values'),
      () async {
    final date = DateTime.fromMicrosecondsSinceEpoch(-1000000000, isUtc: true);
    final resp = await stdTimeSystemTimeBeforeEpochTwinNormal(d: date);
    expect(resp.isUtc, true);
    expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
  });

  test('std::time::Instant', skip: skipWeb('wasm Instant::now panics'),
      () async {
    final date = DateTime.now().toUtc().add(Duration(seconds: 5));
    final resp = await stdTimeInstantTwinNormal(d: date);
    expect(resp.difference(date).abs() < Duration(seconds: 1), true);
  });

  test('tokio::time::Duration', () async {
    final d = Duration(hours: 4);
    final resp = await tokioTimeDurationTwinNormal(d: d);
    expect(resp, d);
  });

  test('tokio::time::Instant', skip: skipWeb('wasm Instant::now panics'),
      () async {
    final date = DateTime.now().toUtc().add(Duration(seconds: 5));
    final resp = await tokioTimeInstantTwinNormal(d: date);
    expect(resp.difference(date).abs() < Duration(seconds: 1), true);
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
    final durations = await handleTimestampsTwinNormal(
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
        await handleDurationsTwinNormal(durations: expected, since: now);
    expect(result, expected.map(now.subtract));
  });

  test('Combined Chrono types', () async {
    final test = await testChronoTwinNormal();
    expect(test.dt!.millisecondsSinceEpoch, 1631297333000);
    expect(test.dt2!.millisecondsSinceEpoch, 1631297333000);
    expect(test.da!.millisecondsSinceEpoch, 1631232000000);
    expect(test.du, Duration(hours: 4));
  });

  test('combined chrono types precise', () async {
    final datetime_1 = DateTime.utc(2002, 02, 23, 12, 13, 55);
    final datetime_2 = DateTime.utc(1800, 01, 23, 12, 56, 25);
    final date_2 = DateTime.utc(1800, 01, 23);
    final duration = Duration(hours: 4);

    final result = await testPreciseChronoTwinNormal();

    expect(
        result.dt!.millisecondsSinceEpoch, datetime_1.millisecondsSinceEpoch);
    expect(
        result.dt2!.millisecondsSinceEpoch, datetime_2.millisecondsSinceEpoch);
    expect(result.da!.millisecondsSinceEpoch, date_2.millisecondsSinceEpoch);
    expect(result.du!.inHours, duration.inHours);
  });

  test('nested chrono types', () async {
    const duration = Duration(hours: 4);
    final naive_date_time = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 0);
    final local = DateTime.now();
    final utc = DateTime.now().toUtc();
    final difference = await howLongDoesItTakeTwinNormal(
        mine: FeatureChronoTwinNormal(
            utc: utc,
            local: local,
            duration: duration,
            naiveDate: naive_date_time,
            naiveDateTime: naive_date_time));
    debugPrint('$difference');
  });
}
