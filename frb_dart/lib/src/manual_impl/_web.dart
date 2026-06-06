import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_typed_data/_web.dart';
import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';

/// {@macro flutter_rust_bridge.internal}
List<dynamic> wireSyncRust2DartDcoIntoDart(WireSyncRust2DartDco syncReturn) =>
    dcoDecodeList(syncReturn);

/// {@macro flutter_rust_bridge.only_for_generated_code}
int dcoDecodePrimitiveInt(Object? raw) {
  if (raw is int) return raw;
  if (raw is double && raw.isFinite && raw == raw.truncateToDouble()) {
    return raw.toInt();
  }
  throw Exception(
    'dcoDecodePrimitiveInt see unexpected type=${raw.runtimeType} value=$raw',
  );
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
String dcoDecodeString(Object? raw) {
  if (raw is String) return raw;
  if (raw is JSAny) {
    final dartified = raw.dartify();
    if (dartified is String) return dartified;
  }
  return String.fromCharCode(dcoDecodePrimitiveInt(raw));
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
List<dynamic> dcoDecodeList(Object? raw) {
  if (raw is List<dynamic>) return raw;
  if (raw is JSAny) {
    final dartified = raw.dartify();
    if (dartified is List<dynamic>) return dartified;
  }
  throw Exception(
    'dcoDecodeList see unexpected type=${raw.runtimeType} value=$raw',
  );
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt dcoDecodeI64(Object? raw) => jsBigIntToDartBigInt(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt dcoDecodeU64(Object? raw) => jsBigIntToDartBigInt(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
Int64List dcoDecodeInt64List(Object? raw) =>
    Int64List.raw(_toListBigInt(dcoDecodeList(raw)));

/// {@macro flutter_rust_bridge.only_for_generated_code}
Uint64List dcoDecodeUint64List(Object? raw) =>
    Uint64List.raw(_toListBigInt(dcoDecodeList(raw)));

List<BigInt> _toListBigInt(List<dynamic> raw) =>
    raw.map((x) => jsBigIntToDartBigInt(x!)).toList();

/// {@macro flutter_rust_bridge.only_for_generated_code}
JSAny cstEncodeInt64List(List<BigInt> raw) => _cstEncodeBigIntList(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
JSAny cstEncodeUint64List(List<BigInt> raw) => _cstEncodeBigIntList(raw);

JSAny _cstEncodeBigIntList(List<BigInt> raw) =>
    raw.map(castNativeBigInt).toList().toJS;

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt sseEncodeCastedPrimitiveI64(int raw) => BigInt.from(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt sseEncodeCastedPrimitiveU64(int raw) => BigInt.from(raw);
