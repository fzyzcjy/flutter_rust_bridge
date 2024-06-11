import 'dart:collection';

import 'package:flutter_rust_bridge/src/exceptions.dart';

/// {@macro flutter_rust_bridge.internal}
abstract class TypedList<T> extends ListMixin<T> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  List<BigInt> get inner;

  /// How to cast a raw JS value to an acceptable Dart value.
  T _raw2dart(BigInt value);

  /// How to convert a Dart integer-like value to an acceptable JS value.
  BigInt _dart2raw(Object? value);

  @override
  T operator [](int index) => _raw2dart(inner[index]);

  @override
  void operator []=(int index, value) => inner[index] = _dart2raw(value);

  @override
  int get length => inner.length;

  @override
  set length(int newLength) => throw const UnmodifiableTypedListException();

// ByteBuffer get buffer => inner.buffer;
}
