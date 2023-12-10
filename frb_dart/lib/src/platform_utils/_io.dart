import 'dart:typed_data';

/// {@macro flutter_rust_bridge.only_for_generated_code}
int castInt(Object? value) => value as int;

/// {@macro flutter_rust_bridge.internal}
extension ExtByteData on ByteData {
  /// {@macro flutter_rust_bridge.internal}
  void generalizedSetUint64(int byteOffset, int value, Endian endian) =>
      setUint64(byteOffset, value, endian);

  /// {@macro flutter_rust_bridge.internal}
  void generalizedSetInt64(int byteOffset, int value, Endian endian) =>
      setInt64(byteOffset, value, endian);
}
