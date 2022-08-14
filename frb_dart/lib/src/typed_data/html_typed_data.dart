import 'package:js/js.dart';
import 'package:js/js_util.dart';

import 'dart:typed_data' hide Int64List, Uint64List;
export 'dart:typed_data' hide Int64List, Uint64List;

@JS('Number')
external int _castInt(Object? value);

/// Copied from https://tc39.es/ecma262/multipage/indexed-collections.html#sec-properties-of-the-%typedarrayprototype%-object
@JS('TypedArray')
abstract class TypedArray implements List {
  @override
  external int get length;
  external ByteBuffer get buffer;
  @JS('at')
  @override
  external dynamic elementAt(int index);
  external int get byteLength;
  external int get byteOffset;
  external TypedArray copyWithin(target, int start, [int? end]);
  external Iterable<dynamic> entries();
  @override
  external bool every(callback);
  @override
  external bool any(callback);
  @override
  external List<T> map<T>(T Function(dynamic) callback);
  @JS('includes')
  @override
  external bool contains(Object? obj);
}

extension TypedArrayExt on TypedArray {
  int operator [](int index) => _castInt(getProperty(this, index));
  Iterator<int> get iterator => entries().map(_castInt).iterator;
}

@JS('BigInt64Array')
abstract class Int64List extends TypedArray {
  external factory Int64List(Object lengthOrBuffer, [int? offset, int? length]);

  factory Int64List.fromList(List<int> list) =>
      Int64List(list.map((n) => BigInt.from(n)).toList());

  factory Int64List.view(
    ByteBuffer buffer, [
    int offset = 0,
    int? length,
  ]) =>
      Int64List(buffer, offset, length);

  factory Int64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Int64List(array.buffer, offset, length);
}

@JS('BigUint64Array')
abstract class Uint64List extends TypedArray {
  external factory Uint64List(Object lengthOrBuffer,
      [int? offset, int? buffer]);

  factory Uint64List.fromList(List<int> list) =>
      Uint64List(list.map((n) => BigInt.from(n)).toList());

  factory Uint64List.view(ByteBuffer buffer, [int offset = 0, int? length]) =>
      Uint64List(buffer, offset, length);

  factory Uint64List.sublistView(TypedData array,
          [int offset = 0, int? length]) =>
      Uint64List(array.buffer, offset, length);
}
