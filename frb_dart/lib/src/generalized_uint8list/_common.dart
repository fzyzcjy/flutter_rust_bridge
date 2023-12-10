import 'dart:typed_data';

/// {@macro flutter_rust_bridge.internal}
typedef BaseGeneralizedUint8ListRaw<PtrType> = ({PtrType ptr, int length});

/// {@macro flutter_rust_bridge.internal}
abstract interface class BaseGeneralizedUint8List<PtrType> {
  /// {@macro flutter_rust_bridge.internal}
  int get length;

  /// {@macro flutter_rust_bridge.internal}
  void resize(int newLen);

  /// {@macro flutter_rust_bridge.internal}
  void dispose();

  /// {@macro flutter_rust_bridge.internal}
  void operator []=(int index, int value);

  /// {@macro flutter_rust_bridge.internal}
  void setRange(int start, int end, Uint8List data);

  /// {@macro flutter_rust_bridge.internal}
  BaseGeneralizedUint8ListRaw<PtrType> intoRaw();
}
