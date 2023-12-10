import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/generalized_typed_data/generalized_typed_data.dart';
import 'package:test/test.dart';

final i64maxb = BigInt.parse('0x7FFFFFFFFFFFFFFF');
final i64minb = BigInt.parse('-0x8000000000000000');
final u64maxb = BigInt.parse('0xFFFFFFFFFFFFFFFF');
const isWeb = bool.fromEnvironment('dart.library.html');

void main() {
  group('big lists', () {
    test('Int64List', () {
      final buf = Int64List(1);
      buf[0] = i64maxb;
      expect(buf[0], i64maxb, reason: 'max');
      buf[0] += BigInt.one;
      if (isWeb) {
        expect(buf[0], i64minb, reason: 'max+1 (wrapped)');
      } else {
        expect(buf[0], i64maxb, reason: 'max+1 (clamped)');
      }
    });

    test('Uint64List', () {
      final buf = Uint64List(2);
      buf[0] = u64maxb;
      buf[1] = i64maxb;
      expect(buf[0], u64maxb, reason: 'max');
      expect(buf[1], i64maxb, reason: 'i64max');
      buf[0] += BigInt.one;
      if (isWeb) {
        expect(buf[0], BigInt.zero, reason: 'max+1 (wrapped)');
      } else {
        expect(buf[0], u64maxb, reason: 'max+1 (clamped)');
      }
      buf[1] += BigInt.one;
      expect(buf[1], i64maxb + BigInt.one, reason: 'i64max+1');
    });
  });

  group('read/write ByteData', () {
    for (final (name, setter) in <(String, _Setter)>[
      ('Uint64', byteDataSetUint64),
      ('Int64', byteDataSetInt64),
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
          _Info(
            setValue: 0x110000000000,
            expectLittleEndian: [0, 0, 0, 0, 0, 0x11, 0, 0],
            expectBigEndian: [0, 0, 0x11, 0, 0, 0, 0, 0],
          ),
          _Info(
            setValue: -0x110000000000,
            expectLittleEndian: [0, 0, 0, 0, 0, 239, 255, 255],
            expectBigEndian: [255, 255, 239, 0, 0, 0, 0, 0],
          ),
          TODO_near_boundary,
          TODO_usebigint_and_test_more_ranges,
          TODO_about_reader,
          TODO_loopback_test,
        ]) {
          test('$info', () => _body(setter, info));
        }
      });
    }
  });
}

typedef _Setter = void Function(
    ByteData byteData, int byteOffset, BigInt value, Endian endian);

void _body(_Setter setter, _Info info) {
  final byteData = ByteData(60);

  setter(byteData, 40, info.setValue, Endian.little);
  expect(byteData.buffer.asUint8List(40, 8), info.expectLittleEndian);

  setter(byteData, 40, info.setValue, Endian.big);
  expect(byteData.buffer.asUint8List(40, 8), info.expectBigEndian);
}

class _Info {
  final BigInt setValue;
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
