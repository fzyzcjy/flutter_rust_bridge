import 'package:flutter_rust_bridge/src/consts.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/generalized_typed_data/generalized_typed_data.dart';
import 'package:meta/meta.dart';
import 'package:uuid/uuid.dart';

export '_io.dart' if (dart.library.html) '_web.dart';

/// This file contains functions that are manually written, but on the other hand,
/// the kinds of such functions are usually generated from frb_codegen.

// ------------------------------------- api2wire -------------------------------------------

Uint8List api2wireConcatenateBytes(List<UuidValue> raw) {
  var builder = BytesBuilder();
  for (final element in raw) {
    builder.add(element.toBytes());
  }
  return builder.toBytes();
}

// ------------------------------------- wire2api -------------------------------------------

@internal
PanicException wire2apiPanicError(dynamic raw) => PanicException(raw as String);

DateTime wire2apiTimestamp({required int ts, required bool isUtc}) {
  if (kIsWeb) {
    return DateTime.fromMillisecondsSinceEpoch(ts, isUtc: isUtc);
  }
  return DateTime.fromMicrosecondsSinceEpoch(ts, isUtc: isUtc);
}

Duration wire2apiDuration(int ts) {
  if (kIsWeb) {
    return Duration(milliseconds: ts);
  }
  return Duration(microseconds: ts);
}

const _kUuidSizeInBytes = 16;

List<UuidValue> wire2apiUuids(Uint8List raw) {
  return List<UuidValue>.generate(raw.lengthInBytes ~/ _kUuidSizeInBytes,
      (int i) => UuidValue.fromByteList(Uint8List.view(raw.buffer, i * _kUuidSizeInBytes, _kUuidSizeInBytes)),
      growable: false);
}
