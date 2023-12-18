import 'dart:convert';

import 'package:flutter_rust_bridge/src/codec/sse.dart';
import 'package:flutter_rust_bridge/src/consts.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';

// ------------------------------------- decode -------------------------------------------

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
PanicException sseDecodePanicError(SseDeserializer deserializer) {
  // NOTE copied from auto-generated SSE deserialization code
  final len = deserializer.buffer.getInt32();
  final inner = deserializer.buffer.getUint8List(len);
  return PanicException(utf8.decoder.convert(inner));
}
