export 'dart:typed_data' hide Int64List, Uint64List;
import 'dart:collection';
import 'dart:typed_data' as $data;

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

abstract class _TypedList<T> extends ListMixin<T> {
  List<int> get inner;
  @override
  _TypedList<T> operator +(Object other);

  T raw2dart(int value);
  int dart2raw(dynamic value);

  @override
  T operator [](int index) => raw2dart(inner[index]);

  @override
  void operator []=(int index, dynamic value) => inner[index] = dart2raw(value);

  @override
  int get length => inner.length;

  @override
  set length(int newLength) => throw const UnmodifiableTypedListException();
}

/// A strict version of [$data.Int64List] which always returns an [Int64].
class Int64List extends _TypedList<BigInt> {
  @override
  final $data.Int64List inner;
  Int64List.from(this.inner);
  factory Int64List(int length) => Int64List.from($data.Int64List(length));
  Int64List.fromList(List<int> ints) : inner = $data.Int64List.fromList(ints);
  Int64List.view($data.ByteBuffer buffer, [int offsetInBytes = 0, int? length])
      : inner = $data.Int64List.view(buffer, offsetInBytes, length);
  Int64List.sublistView($data.TypedData data, [int start = 0, int? end])
      : inner = $data.Int64List.sublistView(data, start, end);

  @override
  int dart2raw(value) {
    if (value is BigInt) return value.toInt();
    if (value is int) return value;
    throw ArgumentError.value(value);
  }

  @override
  BigInt raw2dart(int value) => BigInt.from(value);

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

/// A strict version of [$data.Uint64List] which always returns an [Int64].
class Uint64List extends _TypedList<BigInt> {
  @override
  final $data.Uint64List inner;
  Uint64List.from(this.inner);
  factory Uint64List(int length) => Uint64List.from($data.Uint64List(length));
  Uint64List.fromList(List<int> ints) : inner = $data.Uint64List.fromList(ints);
  Uint64List.view($data.ByteBuffer buffer, [int offsetInBytes = 0, int? length])
      : inner = $data.Uint64List.view(buffer, offsetInBytes, length);
  Uint64List.sublistView($data.TypedData data, [int start = 0, int? end])
      : inner = $data.Uint64List.sublistView(data, start, end);

  static final _maxI64b = BigInt.from(0x7FFFFFFFFFFFFFFF);
  static const _minI64 = 0x8000000000000000;

  @override
  BigInt raw2dart(int value) {
    if (value < 0) {
      // two's complement signed integer to unsigned bigint
      return _maxI64b + BigInt.from(value - _minI64) + BigInt.one;
    }
    return BigInt.from(value);
  }

  @override
  int dart2raw(value) {
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
