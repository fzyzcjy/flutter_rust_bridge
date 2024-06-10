@JS()
library html_typed_data;

import 'dart:collection';
import 'dart:typed_data' hide Int64List, Uint64List;

import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';
import 'package:js/js.dart';
import 'package:js/js_util.dart';

/// A list whose elements are Int64
class Int64List extends _Int64OrUint64List {
  @override
  final _BigInt64Array _inner;

  /// Construct a list
  Int64List._from(this._inner);

  /// Construct a list
  factory Int64List(int length) => Int64List._from(_BigInt64Array(length));

  /// Construct a list
  factory Int64List.fromList(List<int> list) =>
      Int64List._from(_BigInt64Array.fromList(list));

  /// Construct a list
  factory Int64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
      Int64List._from(_BigInt64Array.view(buffer, offset, length));

  /// Construct a list
  factory Int64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Int64List._from(_BigInt64Array.sublistView(array, offset, length));
}

/// A list whose elements are Uint64
class Uint64List extends _Int64OrUint64List {
  @override
  final _BigUint64Array _inner;

  /// Construct a list
  Uint64List._from(this._inner);

  /// Construct a list
  factory Uint64List(int length) => Uint64List._from(_BigUint64Array(length));

  /// Construct a list
  factory Uint64List.fromList(List<int> list) =>
      Uint64List._from(_BigUint64Array.fromList(list));

  /// Construct a list
  factory Uint64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
      Uint64List._from(_BigUint64Array.view(buffer, offset, length));

  /// Construct a list
  factory Uint64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Uint64List._from(_BigUint64Array.sublistView(array, offset, length));
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

Object _convertBigIntToJs(Object dart) {
  if (dart is int) return BigInt.from(dart);
  // Assume value is already JS safe.
  return dart;
}

abstract class _TypedList<T> extends _SetAnyListMixin<T> {
  _TypedArray get _inner;

  /// How to cast a raw JS value to an acceptable Dart value.
  T _js2dart(Object? value);

  /// How to convert a Dart integer-like value to an acceptable JS value.
  Object? _dart2js(Object? value);

  @override
  T operator [](int index) => _js2dart(_inner.at(index));

  @override
  void operator []=(int index, value) {
    _inner[index] = _dart2js(value);
  }

  @override
  int get length => _inner.length;

  @override
  set length(int newLength) => throw const UnmodifiableTypedListException();

  ByteBuffer get buffer => _inner.buffer;
}

abstract class _Int64OrUint64List extends _TypedList<BigInt> {
  @override
  BigInt _js2dart(Object? value) => jsBigIntToDartBigInt(value!);

  @override
  Object? _dart2js(Object? value) => _convertBigIntToJs(value!);
}

/// Opt out of type safety for setting the value.
/// Helpful if the array needs to accept multiple types.
abstract class _SetAnyListMixin<T> extends ListMixin<T> {
  @override
  void operator []=(int index, Object? value) {
    this[index] = value;
  }
}

@JS('TypedArray')
abstract class _TypedArray {
  external ByteBuffer get buffer;

  external int length;

  external BigInt at(int index);
}

extension on _TypedArray {
  operator []=(int index, Object? value) {
    setProperty(this, index, value);
  }
}

/// An array whose element is BigInt64
@JS('BigInt64Array')
abstract class _BigInt64Array extends _TypedArray {
  /// Construct the array
  external factory _BigInt64Array(Object lengthOrBuffer,
      [int? offset, int? length]);

  /// Construct the array from `List<int>`
  factory _BigInt64Array.fromList(List<int> list) =>
      _BigInt64Array(list.map((n) => BigInt.from(n)).toList());

  /// Construct an array view
  factory _BigInt64Array.view(
    ByteBuffer buffer, [
    int offset = 0,
    int? length,
  ]) =>
      _BigInt64Array(buffer, offset, length);

  /// Construct an array sub-list view
  factory _BigInt64Array.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      _BigInt64Array(array.buffer, offset, length);
}

/// An array whose element is BigUint64
@JS('BigUint64Array')
abstract class _BigUint64Array extends _TypedArray {
  /// Construct the array
  external factory _BigUint64Array(Object lengthOrBuffer,
      [int? offset, int? buffer]);

  /// Construct the array from `List<int>`
  factory _BigUint64Array.fromList(List<int> list) =>
      _BigUint64Array(list.map((n) => BigInt.from(n)).toList());

  /// Construct an array view
  factory _BigUint64Array.view(ByteBuffer buffer,
          [int offset = 0, int? length]) =>
      _BigUint64Array(buffer, offset, length);

  /// Construct an array sub-list view
  factory _BigUint64Array.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      _BigUint64Array(array.buffer, offset, length);
}
