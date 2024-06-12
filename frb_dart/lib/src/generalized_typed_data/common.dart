import 'dart:collection';

import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:meta/meta.dart';

/// {@macro flutter_rust_bridge.internal}
abstract class TypedList<T, TInner> extends ListMixin<T> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  List<TInner> get inner;

  /// {@macro flutter_rust_bridge.internal}
  @protected
  T inner2outer(TInner value);

  /// {@macro flutter_rust_bridge.internal}
  @protected
  TInner outer2inner(Object? value);

  @override
  T operator [](int index) => inner2outer(inner[index]);

  @override
  void operator []=(int index, Object? value) =>
      inner[index] = outer2inner(value);

  @override
  int get length => inner.length;

  @override
  set length(int newLength) => throw const UnmodifiableTypedListException();
}
