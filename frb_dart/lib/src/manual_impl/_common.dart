import 'package:flutter_rust_bridge/src/consts.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';

// ------------------------------------- wire2api -------------------------------------------

/// {@macro flutter_rust_bridge.only_for_generated_code}
PanicException wire2apiPanicError(dynamic raw) => PanicException(raw as String);

/// {@macro flutter_rust_bridge.only_for_generated_code}
DateTime wire2apiTimestamp({required int ts, required bool isUtc}) {
  if (kIsWeb) {
    return DateTime.fromMillisecondsSinceEpoch(ts, isUtc: isUtc);
  }
  return DateTime.fromMicrosecondsSinceEpoch(ts, isUtc: isUtc);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
Duration wire2apiDuration(int ts) {
  if (kIsWeb) {
    return Duration(milliseconds: ts);
  }
  return Duration(microseconds: ts);
}
