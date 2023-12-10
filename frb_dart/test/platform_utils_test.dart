import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/platform_utils/platform_utils.dart';
import 'package:test/test.dart';

void main() {
  group('generalizedSetUint64', () {
    for (final info in [
      _Info(
        setValue: setValue,
        expectLittleEndian: expectLittleEndian,
        expectBigEndian: expectBigEndian,
      ),
    ]) {
      test('$info', () => _body((b) => b.generalizedSetUint64, info));
    }
  });

  group('generalizedSetInt64', () {
    for (final info in [
      _Info(
        setValue: setValue,
        expectLittleEndian: expectLittleEndian,
        expectBigEndian: expectBigEndian,
      ),
    ]) {
      test('$info', () => _body((b) => b.generalizedSetInt64, info));
    }
  });
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
  final Uint8List expectLittleEndian;
  final Uint8List expectBigEndian;

  const _Info({
    required this.setValue,
    required this.expectLittleEndian,
    required this.expectBigEndian,
  });

  @override
  String toString() =>
      '_Info{setValue: $setValue, expectLittleEndian: $expectLittleEndian, expectBigEndian: $expectBigEndian}';
}
