import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/generalized_uint8list/generalized_uint8list.dart';

/// {@macro flutter_rust_bridge.internal}
class AdaptedUint8List implements BaseGeneralizedUint8List {
  Uint8List _inner;

  @override
  void operator []=(int index, int value) => _inner[index] = value;

  @override
  void dispose() {}

  @override
  TODO intoRaw() => TODO;

  @override
  void resize(int newLen) {
    final old = _inner;
    _inner = Uint8List(newLen);
    _inner.setRange(0, old.length, old);
  }

  @override
  void setRange(int start, int end, Uint8List data) =>
      _inner.setRange(start, end, data);
}
