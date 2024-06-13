import 'dart:ffi';

import 'package:flutter_rust_bridge/src/dart_c_object_into_dart/_io.dart';
import 'package:flutter_rust_bridge/src/platform_types/_io.dart';
import 'package:flutter_rust_bridge/src/generalized_typed_data/_io.dart';
import 'dart:typed_data' as $data;

/// Generates the dynamic Dart object from either an FFI struct or a JS value
///
/// {@macro flutter_rust_bridge.internal}
List<dynamic> wireSyncRust2DartDcoIntoDart(WireSyncRust2DartDco syncReturn) =>
    dartCObjectIntoDart(syncReturn.ref);

/// {@macro flutter_rust_bridge.only_for_generated_code}
int dcoDecodeI64(int raw) => raw;

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt dcoDecodeU64(int raw) => BigInt.from(raw).toUnsigned(64);

/// {@macro flutter_rust_bridge.only_for_generated_code}
Int64List dcoDecodeInt64List($data.Int64List raw) => Int64List.raw(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
Uint64List dcoDecodeUint64List($data.Uint64List raw) => Uint64List.raw(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
int sseEncodeCastedPrimitiveI64(int raw) => raw;

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt sseEncodeCastedPrimitiveU64(int raw) => BigInt.from(raw);
