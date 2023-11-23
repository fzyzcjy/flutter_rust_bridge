// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<DateTime> datetimeUtc({required DateTime d, dynamic hint}) =>
    RustLib.instance.api.datetimeUtc(d: d, hint: hint);

Future<DateTime> datetimeLocal({required DateTime d, dynamic hint}) =>
    RustLib.instance.api.datetimeLocal(d: d, hint: hint);

Future<DateTime> naivedatetime({required DateTime d, dynamic hint}) =>
    RustLib.instance.api.naivedatetime(d: d, hint: hint);

Future<DateTime?> optionalEmptyDatetimeUtc({DateTime? d, dynamic hint}) =>
    RustLib.instance.api.optionalEmptyDatetimeUtc(d: d, hint: hint);

Future<Duration> duration({required Duration d, dynamic hint}) =>
    RustLib.instance.api.duration(d: d, hint: hint);

Future<List<Duration>> handleTimestamps(
        {required List<DateTime> timestamps,
        required DateTime epoch,
        dynamic hint}) =>
    RustLib.instance.api
        .handleTimestamps(timestamps: timestamps, epoch: epoch, hint: hint);

Future<List<DateTime>> handleDurations(
        {required List<Duration> durations,
        required DateTime since,
        dynamic hint}) =>
    RustLib.instance.api
        .handleDurations(durations: durations, since: since, hint: hint);

Future<TestChrono> testChrono({dynamic hint}) =>
    RustLib.instance.api.testChrono(hint: hint);

Future<TestChrono> testPreciseChrono({dynamic hint}) =>
    RustLib.instance.api.testPreciseChrono(hint: hint);

Future<Duration> howLongDoesItTake(
        {required FeatureChrono mine, dynamic hint}) =>
    RustLib.instance.api.howLongDoesItTake(mine: mine, hint: hint);

class FeatureChrono {
  final DateTime utc;
  final DateTime local;
  final Duration duration;
  final DateTime naive;

  const FeatureChrono({
    required this.utc,
    required this.local,
    required this.duration,
    required this.naive,
  });

  @override
  int get hashCode =>
      utc.hashCode ^ local.hashCode ^ duration.hashCode ^ naive.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureChrono &&
          runtimeType == other.runtimeType &&
          utc == other.utc &&
          local == other.local &&
          duration == other.duration &&
          naive == other.naive;
}

class TestChrono {
  final DateTime? dt;
  final DateTime? dt2;
  final Duration? du;

  const TestChrono({
    this.dt,
    this.dt2,
    this.du,
  });

  @override
  int get hashCode => dt.hashCode ^ dt2.hashCode ^ du.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TestChrono &&
          runtimeType == other.runtimeType &&
          dt == other.dt &&
          dt2 == other.dt2 &&
          du == other.du;
}
