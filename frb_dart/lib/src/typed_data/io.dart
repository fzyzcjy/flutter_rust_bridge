export 'dart:typed_data' hide Int64List, Uint64List;
import 'dart:collection';
import 'dart:typed_data' as $data;

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

abstract class _Int64List extends ListMixin<Int64> {
  List<int> get inner;
  @override
  _Int64List operator +(Object other);

  @override
  Int64 operator [](int index) => Int64(inner[index]);

  @override
  void operator []=(int index, dynamic value) =>
      inner[index] = value is Int64 ? value.toInt() : value;

  @override
  int get length => inner.length;

  @override
  set length(int newLength) => throw const UnmodifiableTypedListException();
}

/// A strict version of [$data.Int64List] which always returns an [Int64].
class Int64List extends _Int64List {
  @override
  final $data.Int64List inner;
  Int64List.from(this.inner);
  Int64List.fromList(List<int> ints) : inner = $data.Int64List.fromList(ints);
  Int64List.view($data.ByteBuffer buffer, [int offsetInBytes = 0, int? length])
      : inner = $data.Int64List.view(buffer, offsetInBytes, length);
  Int64List.sublistView($data.TypedData data, [int start = 0, int? end])
      : inner = $data.Int64List.sublistView(data, start, end);

  @override
  Int64List operator +(Object other) {
    if (other is Int64List) return Int64List.fromList(inner + other.inner);
    if (other is $data.Int64List) return Int64List.fromList(inner + other);
    if (other is List<int>) return Int64List.fromList(inner + other);
    if (other is Iterable<int>) {
      return Int64List.fromList(inner + other.toList(growable: false));
    }
    throw UnimplementedError(
      'Cannot add list of unrelated type: ${other.runtimeType}',
    );
  }
}

/// A strict version of [$data.Uint64List] which always returns an [Int64].
class Uint64List extends _Int64List {
  @override
  final $data.Uint64List inner;
  Uint64List.from(this.inner);
  Uint64List.fromList(List<int> ints) : inner = $data.Uint64List.fromList(ints);
  Uint64List.view($data.ByteBuffer buffer, [int offsetInBytes = 0, int? length])
      : inner = $data.Uint64List.view(buffer, offsetInBytes, length);
  Uint64List.sublistView($data.TypedData data, [int start = 0, int? end])
      : inner = $data.Uint64List.sublistView(data, start, end);

  @override
  Uint64List operator +(Object other) {
    if (other is Uint64List) return Uint64List.fromList(inner + other.inner);
    if (other is $data.Uint64List) return Uint64List.fromList(inner + other);
    if (other is List<int>) return Uint64List.fromList(inner + other);
    if (other is Iterable<int>) {
      return Uint64List.fromList(inner + other.toList(growable: false));
    }
    throw UnimplementedError(
      'Cannot add list of unrelated type: ${other.runtimeType}',
    );
  }
}
