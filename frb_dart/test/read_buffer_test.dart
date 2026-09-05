import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/third_party/flutter_foundation_serialization/read_buffer.dart';
import 'package:test/test.dart';

void main() {
  test('ReadBuffer reads scalar values and copied byte lists sequentially', () {
    final data = ByteData(7)
      ..setUint8(0, 9)
      ..setInt16(1, -2, Endian.big)
      ..setFloat32(3, 1.5, Endian.big);
    final buffer = ReadBuffer(data);

    expect(buffer.getUint8(), 9);
    expect(buffer.getInt16(endian: Endian.big), -2);
    expect(buffer.getFloat32(endian: Endian.big), 1.5);
    expect(buffer.hasRemaining, isFalse);
  });

  test('ReadBuffer copied byte list does not retain the source storage', () {
    final data = ByteData.sublistView(Uint8List.fromList([1, 2, 3]));
    final buffer = ReadBuffer(data);

    final decoded = buffer.getUint8List(3);
    data.setUint8(0, 9);

    expect(decoded, [1, 2, 3]);
    expect(buffer.hasRemaining, isFalse);
  });

  test('ReadBuffer keeps its position when a read exceeds remaining bytes', () {
    final buffer = ReadBuffer(ByteData.sublistView(Uint8List.fromList([7])));

    expect(buffer.getUint16, throwsA(isA<ArgumentError>()));
    expect(buffer.getUint8(), 7);
    expect(buffer.hasRemaining, isFalse);
  });
}
