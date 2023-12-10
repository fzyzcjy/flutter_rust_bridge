import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/platform_utils/platform_utils.dart';
import 'package:test/test.dart';

void main() {
  group('generalizedSetUint64', () {
    for (final info in [
      _Info(setValue: setValue, endian: endian, expectBytes: expectBytes),
    ]) {
      test('$info', () => _body((b) => b.generalizedSetUint64, info));
    }
  });

  group('generalizedSetInt64', () {
    for (final info in [
      _Info(setValue: setValue, endian: endian, expectBytes: expectBytes),
    ]) {
      test('$info', () => _body((b) => b.generalizedSetInt64, info));
    }
  });
}

typedef _GeneralizedSetter = void Function(
    int byteOffset, int value, Endian endian);

void _body(_GeneralizedSetter Function(ByteData) getFunction, _Info info) {
  final byteData = ByteData(100);
  getFunction(byteData)(50, info.setValue, info.endian);
  expect(byteData.buffer.asUint8List(50, 8), info.expectBytes);
}

class _Info {
  final int setValue;
  final Endian endian;
  final Uint8List expectBytes;

  const _Info({
    required this.setValue,
    required this.endian,
    required this.expectBytes,
  });

  @override
  String toString() =>
      '_Info{setValue: $setValue, endian: $endian, expectBytes: $expectBytes}';
}
