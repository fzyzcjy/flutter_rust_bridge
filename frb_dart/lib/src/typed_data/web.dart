@JS()
library html_typed_data;

import 'dart:collection';
import 'package:js/js.dart';
import 'package:js/js_util.dart';

import 'dart:typed_data' hide Int64List, Uint64List;

import '../helpers.dart' show UnmodifiableTypedListException;
export 'dart:typed_data' hide Int64List, Uint64List;

@JS('TypedArray')
abstract class TypedArray {
  external ByteBuffer get buffer;
  external int length;
  external BigInt at(int index);
}

extension on TypedArray {
  operator []=(int index, value) {
    setProperty(this, index, value);
  }
}

@JS('BigInt64Array')
abstract class BigInt64Array extends TypedArray {
  external factory BigInt64Array(Object lengthOrBuffer,
      [int? offset, int? length]);

  factory BigInt64Array.fromList(List<int> list) =>
      BigInt64Array(list.map((n) => BigInt.from(n)).toList());

  factory BigInt64Array.view(
    ByteBuffer buffer, [
    int offset = 0,
    int? length,
  ]) =>
      BigInt64Array(buffer, offset, length);

  factory BigInt64Array.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      BigInt64Array(array.buffer, offset, length);
}

@JS('BigUint64Array')
abstract class BigUint64Array extends TypedArray {
  external factory BigUint64Array(Object lengthOrBuffer,
      [int? offset, int? buffer]);

  factory BigUint64Array.fromList(List<int> list) =>
      BigUint64Array(list.map((n) => BigInt.from(n)).toList());

  factory BigUint64Array.view(ByteBuffer buffer,
          [int offset = 0, int? length]) =>
      BigUint64Array(buffer, offset, length);

  factory BigUint64Array.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      BigUint64Array(array.buffer, offset, length);
}

/// Opt out of type safety for setting the value.
/// Helpful if the array needs to accept multiple types.
abstract class _SetAnyListMixin<T> extends ListMixin<T> {
  @override
  void operator []=(int index, dynamic value) {
    this[index] = value;
  }
}

abstract class TypedList<T> extends _SetAnyListMixin<T> {
  TypedArray get inner;

  /// How to cast a raw JS value to an acceptable Dart value.
  T js2dart(Object? value);

  /// How to convert a Dart integer-like value to an acceptable JS value.
  dynamic dart2js(Object? value);

  @override
  T operator [](int index) => js2dart(inner.at(index));

  @override
  void operator []=(int index, value) {
    inner[index] = dart2js(value);
  }

  @override
  int get length => inner.length;

  @override
  set length(int newLength) => throw UnmodifiableTypedListException();

  ByteBuffer get buffer => inner.buffer;
}

BigInt _castBigInt(Object bigInt) {
  return BigInt.parse(callMethod(bigInt, 'toString', const []));
}

Object _convertBigInt(Object dart) {
  if (dart is int) return BigInt.from(dart);
  // Assume value is already JS safe.
  return dart;
}

class Int64List extends TypedList<BigInt> {
  @override
  final BigInt64Array inner;
  Int64List.from(this.inner);

  @override
  BigInt js2dart(Object? value) => _castBigInt(value!);

  @override
  dart2js(Object? value) => _convertBigInt(value!);

  factory Int64List(int length) => Int64List.from(BigInt64Array(length));
  factory Int64List.fromList(List<int> list) =>
      Int64List.from(BigInt64Array.fromList(list));
  factory Int64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
      Int64List.from(BigInt64Array.view(buffer, offset, length));
  factory Int64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Int64List.from(BigInt64Array.sublistView(array, offset, length));
}

class Uint64List extends TypedList<BigInt> {
  @override
  final BigUint64Array inner;
  Uint64List.from(this.inner);

  @override
  BigInt js2dart(Object? value) => _castBigInt(value!);

  @override
  dart2js(Object? value) => _convertBigInt(value!);

  factory Uint64List(int length) => Uint64List.from(BigUint64Array(length));
  factory Uint64List.fromList(List<int> list) =>
      Uint64List.from(BigUint64Array.fromList(list));
  factory Uint64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
      Uint64List.from(BigUint64Array.view(buffer, offset, length));
  factory Uint64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Uint64List.from(BigUint64Array.sublistView(array, offset, length));
}
