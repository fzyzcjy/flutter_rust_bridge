import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/consts.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:uuid/uuid.dart';

// ------------------------------------- api2wire -------------------------------------------

/// {@macro flutter_rust_bridge.only_for_generated_code}
Uint8List api2wireConcatenateBytes(List<UuidValue> raw) {
  var builder = BytesBuilder();
  for (final element in raw) {
    builder.add(element.toBytes());
  }
  return builder.toBytes();
}

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

const _kUuidSizeInBytes = 16;

/// {@macro flutter_rust_bridge.only_for_generated_code}
List<UuidValue> wire2apiUuids(Uint8List raw) {
  return List<UuidValue>.generate(raw.lengthInBytes ~/ _kUuidSizeInBytes,
      (int i) => UuidValue.fromByteList(Uint8List.view(raw.buffer, i * _kUuidSizeInBytes, _kUuidSizeInBytes)),
      growable: false);
}
