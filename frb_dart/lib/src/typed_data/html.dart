@JS()
library html_typed_data;

import 'dart:collection';
import 'package:js/js.dart';
import 'package:fixnum/fixnum.dart';

import 'dart:typed_data' hide Int64List, Uint64List;

import 'package:js/js_util.dart';
export 'dart:typed_data' hide Int64List, Uint64List;

Int64List int64ListFrom(dynamic raw) => Int64List.from(raw);

Uint64List uint64ListFrom(dynamic raw) => Uint64List.from(raw);

// Borrowed from wasm_bindgen
final _int64Shim = BigInt64Array(1);
final _int32Shim = Int32List.view(_int64Shim.buffer);
final _byteShim = Uint8List.view(_int64Shim.buffer);

@JS('TypedArray')
class TypedArray {
  external ByteBuffer get buffer;
  external int length;
  external at(int index);
}

extension TypedArrayExt on TypedArray {
  void operator []=(int index, value) {
    setProperty(this, index, value);
  }
}

@JS('BigInt64Array')
class BigInt64Array extends TypedArray {
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
class BigUint64Array extends TypedArray {
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

abstract class TypedList extends ListMixin {
  TypedArray get inner;

  /// How to cast a raw JS value to an acceptable Dart value.
  dynamic _cast(Object? value);

  /// How to convert a Dart integer-like value to an acceptable JS value.
  dynamic _convert(Object? value);

  @override
  operator [](int index) => _cast(inner.at(index));

  @override
  void operator []=(int index, value) {
    inner[index] = _convert(value);
  }

  @override
  int get length => inner.length;

  @override
  set length(int newLength) => throw UnmodifiableTypedListException();
}

Int64 _castBigInt(Object bigInt) {
  _int64Shim[0] = bigInt;
  final lo = _int32Shim[0];
  final hi = _int32Shim[1];
  return Int64.fromInts(hi, lo);
}

Object _convertBigInt(Object dart) {
  if (dart is int) return BigInt.from(dart);
  if (dart is Int64) {
    _byteShim.setAll(0, dart.toBytes());
    return _int64Shim.at(0);
  }
  // Assume value is already JS safe.
  return dart;
}

class Int64List extends TypedList {
  @override
  final BigInt64Array inner;
  Int64List.from(this.inner);

  @override
  _cast(Object? value) => _castBigInt(value!);
  @override
  _convert(Object? value) => _convertBigInt(value!);

  factory Int64List(int length) => Int64List.from(BigInt64Array(length));
  factory Int64List.fromList(List<int> list) =>
      Int64List.from(BigInt64Array.fromList(list));
  factory Int64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
      Int64List.from(BigInt64Array.view(buffer, offset, length));
  factory Int64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Int64List.from(BigInt64Array.sublistView(array, offset, length));
}

class Uint64List extends TypedList {
  @override
  final BigUint64Array inner;
  Uint64List.from(this.inner);

  @override
  _cast(Object? value) => _castBigInt(value!);
  @override
  _convert(Object? value) => _convertBigInt(value!);

  factory Uint64List(int length) => Uint64List.from(BigUint64Array(length));
  factory Uint64List.fromList(List<int> list) =>
      Uint64List.from(BigUint64Array.fromList(list));
  factory Uint64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
      Uint64List.from(BigUint64Array.view(buffer, offset, length));
  factory Uint64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Uint64List.from(BigUint64Array.sublistView(array, offset, length));
}

class UnmodifiableTypedListException implements Exception {
  const UnmodifiableTypedListException();

  static const _message = 'Cannot modify the length of typed lists.';

  @override
  String toString() => _message;
}
