import 'dart:collection';
import 'dart:typed_data' hide Int64List, Uint64List;

import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/generalized_typed_data/common.dart';
import 'package:flutter_rust_bridge/src/generalized_typed_data/common.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';

Object _convertBigIntToJs(Object dart) {
  if (dart is int) return BigInt.from(dart);
  // Assume value is already JS safe.
  return dart;
}

/// A list whose elements are Int64
class Int64List extends TypedList<BigInt> {
  @override
  final BigInt64Array inner;

  /// Construct a list
  Int64List.from(this.inner);

  @override
  BigInt _inner2outer(Object? value) => jsBigIntToDartBigInt(value!);

  @override
  Object? _outer2inner(Object? value) => _convertBigIntToJs(value!);

  /// Construct a list
  factory Int64List(int length) => Int64List.from(BigInt64Array(length));

  /// Construct a list
  factory Int64List.fromList(List<int> list) =>
      Int64List.from(BigInt64Array.fromList(list));

  /// Construct a list
  factory Int64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
      Int64List.from(BigInt64Array.view(buffer, offset, length));

  /// Construct a list
  factory Int64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Int64List.from(BigInt64Array.sublistView(array, offset, length));
}

/// A list whose elements are Uint64
class Uint64List extends TypedList<BigInt> {
  @override
  final BigUint64Array inner;

  /// Construct a list
  Uint64List.from(this.inner);

  @override
  BigInt _inner2outer(Object? value) => jsBigIntToDartBigInt(value!);

  @override
  Object? _outer2inner(Object? value) => _convertBigIntToJs(value!);

  /// Construct a list
  factory Uint64List(int length) => Uint64List.from(BigUint64Array(length));

  /// Construct a list
  factory Uint64List.fromList(List<int> list) =>
      Uint64List.from(BigUint64Array.fromList(list));

  /// Construct a list
  factory Uint64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
      Uint64List.from(BigUint64Array.view(buffer, offset, length));

  /// Construct a list
  factory Uint64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Uint64List.from(BigUint64Array.sublistView(array, offset, length));
}

/// {@macro flutter_rust_bridge.internal}
void byteDataSetUint64(
        ByteData byteData, int byteOffset, BigInt value, Endian endian) =>
    byteDataSetInt64(byteData, byteOffset, value.toSigned(64), endian);

/// {@macro flutter_rust_bridge.internal}
void byteDataSetInt64(
    ByteData byteData, int byteOffset, BigInt value, Endian endian) {
  // Quite hacky, should improve if used frequently in the future
  // Or use `fixnum` https://pub.dev/documentation/fixnum/latest/fixnum/Int64/toBytes.html
  // Related: https://github.com/dart-lang/sdk/issues/10275
  final lo = (value & BigInt.from(0xffffffff)).toInt();
  final hi = (value >> 32).toInt();
  if (endian == Endian.little) {
    byteData.setInt32(byteOffset, lo, endian);
    byteData.setInt32(byteOffset + 4, hi, endian);
  } else if (endian == Endian.big) {
    byteData.setInt32(byteOffset, hi, endian);
    byteData.setInt32(byteOffset + 4, lo, endian);
  } else {
    throw UnimplementedError("Unknown endian");
  }
}

/// {@macro flutter_rust_bridge.internal}
BigInt byteDataGetUint64(ByteData byteData, int byteOffset, Endian endian) {
  final lo = BigInt.from(byteData.getUint32(byteOffset, endian));
  final hi = BigInt.from(byteData.getUint32(byteOffset + 4, endian));
  if (endian == Endian.little) {
    return lo + (hi << 32);
  } else if (endian == Endian.big) {
    return (lo << 32) + hi;
  } else {
    throw UnimplementedError("Unknown endian");
  }
}

/// {@macro flutter_rust_bridge.internal}
BigInt byteDataGetInt64(ByteData byteData, int byteOffset, Endian endian) {
  // Just a quick hack, should improve if used frequently in the future
  var ans = byteDataGetUint64(byteData, byteOffset, endian);
  if ((ans & (BigInt.from(1) << 63)) != BigInt.from(0)) {
    ans -= BigInt.from(1) << 64;
  }
  return ans;
}
