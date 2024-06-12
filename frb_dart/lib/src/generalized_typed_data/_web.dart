import 'dart:typed_data' hide Int64List, Uint64List;

import 'package:flutter_rust_bridge/src/generalized_typed_data/common.dart';

class _Int64OrUint64List extends TypedList<BigInt, BigInt> {
  @override
  final List<BigInt> inner;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  _Int64OrUint64List.raw(this.inner);

  @override
  BigInt inner2outer(BigInt value) => value;

  @override
  BigInt outer2inner(Object? value) {
    if (value is int) return BigInt.from(value);
    if (value is BigInt) return value;
    throw ArgumentError.value(value);
  }
}

/// A list whose elements are Int64
class Int64List extends _Int64OrUint64List {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Int64List.raw(super.inner) : super.raw();

  /// Construct a list
  factory Int64List(int length) =>
      Int64List.raw(List.filled(length, BigInt.zero));

  /// Construct a list
  factory Int64List.fromList(List<int> list) =>
      Int64List.raw(list.map(BigInt.from).toList());

// /// Construct a list
// factory Int64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
//     Int64List.from(BigInt64Array.view(buffer, offset, length));
//
// /// Construct a list
// factory Int64List.sublistView(TypedData array,
//         [int offset = 0, int? length]) =>
//     Int64List.from(BigInt64Array.sublistView(array, offset, length));
}

/// A list whose elements are Uint64
class Uint64List extends _Int64OrUint64List {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Uint64List.raw(super.inner) : super.raw();

  /// Construct a list
  factory Uint64List(int length) =>
      Uint64List.raw(List.filled(length, BigInt.zero));

  /// Construct a list
  factory Uint64List.fromList(List<int> list) =>
      Uint64List.raw(list.map(BigInt.from).toList());

// /// Construct a list
// factory Uint64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
//     Uint64List.from(BigUint64Array.view(buffer, offset, length));
//
// /// Construct a list
// factory Uint64List.sublistView(TypedData array,
//         [int offset = 0, int? length]) =>
//     Uint64List.from(BigUint64Array.sublistView(array, offset, length));
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
