import 'dart:collection';

import 'package:flutter_rust_bridge/src/exceptions.dart';

/// {@macro flutter_rust_bridge.internal}
abstract class TypedList<T, TInner> extends ListMixin<T> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  List<TInner> get inner;

  @override
  TypedList<T, TInner> operator +(Object other);

  T _inner2outer(TInner value);

  TInner _outer2inner(Object? value);

  @override
  T operator [](int index) => _inner2outer(inner[index]);

  @override
  void operator []=(int index, Object? value) =>
      inner[index] = _outer2inner(value);

  @override
  int get length => inner.length;

  @override
  set length(int newLength) => throw const UnmodifiableTypedListException();
}
