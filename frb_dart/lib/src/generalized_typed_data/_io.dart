import 'dart:typed_data' as $data;

import 'package:flutter_rust_bridge/src/generalized_typed_data/common.dart';

/// A strict version of [$data.Int64List] which always returns a [BigInt].
class Int64List extends TypedList<BigInt, int> {
  @override
  final $data.Int64List inner;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Int64List.raw(this.inner);

  /// Construct a list of the [length].
  factory Int64List(int length) => Int64List.raw($data.Int64List(length));

  /// Construct a list raw `List<int>`.
  Int64List.fromList(List<int> ints) : inner = $data.Int64List.fromList(ints);

  /// Construct a list view
  Int64List.view($data.ByteBuffer buffer, [int offsetInBytes = 0, int? length])
      : inner = $data.Int64List.view(buffer, offsetInBytes, length);

  /// Construct a sub-list view
  Int64List.sublistView($data.TypedData data, [int start = 0, int? end])
      : inner = $data.Int64List.sublistView(data, start, end);

  @override
  int outer2inner(Object? value) {
    if (value is BigInt) return value.toInt();
    if (value is int) return value;
    throw ArgumentError.value(value);
  }

  @override
  BigInt inner2outer(int value) => BigInt.from(value);

  @override
  Int64List operator +(Object other) {
    if (other is Int64List) return Int64List.fromList(inner + other.inner);
    if (other is $data.Int64List) return Int64List.fromList(inner + other);
    if (other is List<int>) return Int64List.fromList(inner + other);
    if (other is Iterable<int>) {
      return Int64List.fromList(inner + other.toList(growable: false));
    }
    throw ArgumentError.value(other);
  }
}

/// A strict version of [$data.Uint64List] which always returns a [BigInt].
class Uint64List extends TypedList<BigInt, int> {
  @override
  final $data.Uint64List inner;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Uint64List.raw(this.inner);

  /// Construct a list of the [length].
  factory Uint64List(int length) => Uint64List.raw($data.Uint64List(length));

  /// Construct a list raw `List<int>`.
  Uint64List.fromList(List<int> ints) : inner = $data.Uint64List.fromList(ints);

  /// Construct a list view
  Uint64List.view($data.ByteBuffer buffer, [int offsetInBytes = 0, int? length])
      : inner = $data.Uint64List.view(buffer, offsetInBytes, length);

  /// Construct a sub-list view
  Uint64List.sublistView($data.TypedData data, [int start = 0, int? end])
      : inner = $data.Uint64List.sublistView(data, start, end);

  static final _maxI64b = BigInt.from(0x7FFFFFFFFFFFFFFF);
  static const _minI64 = 0x8000000000000000;

  @override
  BigInt inner2outer(int value) {
    if (value < 0) {
      // two's complement signed integer to unsigned bigint
      return _maxI64b + BigInt.from(value - _minI64) + BigInt.one;
    }
    return BigInt.from(value);
  }

  @override
  int outer2inner(Object? value) {
    if (value is int) return value;
    if (value is BigInt) {
      if (value > _maxI64b) {
        // unsigned bigint (64 bits) to two's complement signed integer
        value -= _maxI64b;
        value -= BigInt.one;
        return value.toInt() + _minI64;
      } else {
        return value.toInt();
      }
    }
    throw ArgumentError.value(value);
  }

  @override
  Uint64List operator +(Object other) {
    if (other is Uint64List) return Uint64List.fromList(inner + other.inner);
    if (other is $data.Uint64List) return Uint64List.fromList(inner + other);
    if (other is List<int>) return Uint64List.fromList(inner + other);
    if (other is Iterable<int>) {
      return Uint64List.fromList(inner + other.toList(growable: false));
    }
    throw ArgumentError.value(other);
  }
}

/// {@macro flutter_rust_bridge.internal}
void byteDataSetUint64($data.ByteData byteData, int byteOffset, BigInt value,
        $data.Endian endian) =>
    byteData.setUint64(byteOffset, value.toSigned(64).toInt(), endian);

/// {@macro flutter_rust_bridge.internal}
void byteDataSetInt64($data.ByteData byteData, int byteOffset, BigInt value,
        $data.Endian endian) =>
    byteData.setInt64(byteOffset, value.toInt(), endian);

/// {@macro flutter_rust_bridge.internal}
BigInt byteDataGetUint64(
        $data.ByteData byteData, int byteOffset, $data.Endian endian) =>
    BigInt.from(byteData.getUint64(byteOffset, endian)).toUnsigned(64);

/// {@macro flutter_rust_bridge.internal}
BigInt byteDataGetInt64(
        $data.ByteData byteData, int byteOffset, $data.Endian endian) =>
    BigInt.from(byteData.getInt64(byteOffset, endian));
