import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/platform_utils/platform_utils.dart';
import 'package:test/test.dart';

void main() {
  for (final (name, getFunction)
      in <(String, _GeneralizedSetter Function(ByteData))>[
    ('Uint64', (b) => b.generalizedSetUint64),
    ('Int64', (b) => b.generalizedSetInt64),
  ]) {
    group(name, () {
      for (final info in const [
        _Info(
          setValue: 0,
          expectLittleEndian: [0, 0, 0, 0, 0, 0, 0, 0],
          expectBigEndian: [0, 0, 0, 0, 0, 0, 0, 0],
        ),
        _Info(
          setValue: 2,
          expectLittleEndian: [2, 0, 0, 0, 0, 0, 0, 0],
          expectBigEndian: [0, 0, 0, 0, 0, 0, 0, 2],
        ),
        _Info(
          setValue: -2,
          expectLittleEndian: [254, 255, 255, 255, 255, 255, 255, 255],
          expectBigEndian: [255, 255, 255, 255, 255, 255, 255, 254],
        ),
        _Info(
          setValue: 2023,
          expectLittleEndian: [231, 7, 0, 0, 0, 0, 0, 0],
          expectBigEndian: [0, 0, 0, 0, 0, 0, 7, 231],
        ),
        _Info(
          setValue: 0x112233445566,
          expectLittleEndian: [0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0, 0],
          expectBigEndian: [0, 0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        ),
        _Info(
          setValue: -0x112233445566,
          expectLittleEndian: [154, 170, 187, 204, 221, 238, 255, 255],
          expectBigEndian: [255, 255, 238, 221, 204, 187, 170, 154],
        ),
      ]) {
        test('$info', () => _body(getFunction, info));
      }
    });
  }
}

typedef _GeneralizedSetter = void Function(
    int byteOffset, int value, Endian endian);

void _body(_GeneralizedSetter Function(ByteData) getFunction, _Info info) {
  final byteData = ByteData(60);

  getFunction(byteData)(40, info.setValue, Endian.little);
  expect(byteData.buffer.asUint8List(40, 8), info.expectLittleEndian);

  getFunction(byteData)(40, info.setValue, Endian.big);
  expect(byteData.buffer.asUint8List(40, 8), info.expectBigEndian);
}

class _Info {
  final int setValue;
  final List<int> expectLittleEndian;
  final List<int> expectBigEndian;

  const _Info({
    required this.setValue,
    required this.expectLittleEndian,
    required this.expectBigEndian,
  });

  @override
  String toString() =>
      '_Info{setValue: $setValue, expectLittleEndian: $expectLittleEndian, expectBigEndian: $expectBigEndian}';
}
