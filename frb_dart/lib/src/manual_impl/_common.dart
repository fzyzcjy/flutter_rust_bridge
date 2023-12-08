import 'package:flutter_rust_bridge/src/codec/sse.dart';
import 'package:flutter_rust_bridge/src/consts.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

// ------------------------------------- dco decode -------------------------------------------

/// {@macro flutter_rust_bridge.only_for_generated_code}
PanicException dcoDecodePanicError(dynamic raw) =>
    PanicException(raw as String);

/// {@macro flutter_rust_bridge.only_for_generated_code}
DateTime dcoDecodeTimestamp({required int ts, required bool isUtc}) {
  if (kIsWeb) {
    return DateTime.fromMillisecondsSinceEpoch(ts, isUtc: isUtc);
  }
  return DateTime.fromMicrosecondsSinceEpoch(ts, isUtc: isUtc);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
Duration dcoDecodeDuration(int ts) {
  if (kIsWeb) {
    return Duration(milliseconds: ts);
  }
  return Duration(microseconds: ts);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
void sseEncodeNativePort(NativePortType port, SseSerializer serializer) {
  TODO;
}
