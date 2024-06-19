import 'dart:math';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/generalized_typed_data/generalized_typed_data.dart';
import 'package:test/test.dart';

final i64maxb = BigInt.parse('0x7FFFFFFFFFFFFFFF');
final i64minb = BigInt.parse('-0x8000000000000000');
final u64maxb = BigInt.parse('0xFFFFFFFFFFFFFFFF');
const isWeb = bool.fromEnvironment('dart.library.js_interop');

void main() {
  group('big lists', () {
    test('Int64List', () {
      final buf = Int64List(1);
      buf[0] = i64maxb;
      expect(buf[0], i64maxb, reason: 'max');
      buf[0] += BigInt.one;
      if (isWeb) {
        expect(buf[0], i64maxb + BigInt.from(1), reason: 'max+1 (bigint)');
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
        expect(buf[0], u64maxb + BigInt.from(1), reason: 'max+1 (bigint)');
      } else {
        expect(buf[0], u64maxb, reason: 'max+1 (clamped)');
      }
      buf[1] += BigInt.one;
      expect(buf[1], i64maxb + BigInt.one, reason: 'i64max+1');
    });
  });

  group('read/write ByteData', () {
    for (final (name, setter, getter) in <(_Name, _Setter, _Getter)>[
      (_Name.uint64, byteDataSetUint64, byteDataGetUint64),
      (_Name.int64, byteDataSetInt64, byteDataGetInt64),
    ]) {
      group(name, () {
        for (final endian in [Endian.little, Endian.big]) {
          group('endian=$endian', () {
            for (final info in [
              _Info(
                integer: BigInt.parse('0'),
                expectLittleEndian: [0, 0, 0, 0, 0, 0, 0, 0],
                expectBigEndian: [0, 0, 0, 0, 0, 0, 0, 0],
              ),
              _Info(
                integer: BigInt.parse('1'),
                expectLittleEndian: [1, 0, 0, 0, 0, 0, 0, 0],
                expectBigEndian: [0, 0, 0, 0, 0, 0, 0, 1],
              ),
              if (name != _Name.uint64)
                _Info(
                  integer: BigInt.parse('-1'),
                  expectLittleEndian: [255, 255, 255, 255, 255, 255, 255, 255],
                  expectBigEndian: [255, 255, 255, 255, 255, 255, 255, 255],
                ),
              _Info(
                integer: BigInt.parse('2'),
                expectLittleEndian: [2, 0, 0, 0, 0, 0, 0, 0],
                expectBigEndian: [0, 0, 0, 0, 0, 0, 0, 2],
              ),
              if (name != _Name.uint64)
                _Info(
                  integer: BigInt.parse('-2'),
                  expectLittleEndian: [254, 255, 255, 255, 255, 255, 255, 255],
                  expectBigEndian: [255, 255, 255, 255, 255, 255, 255, 254],
                ),
              _Info(
                integer: BigInt.parse('2023'),
                expectLittleEndian: [231, 7, 0, 0, 0, 0, 0, 0],
                expectBigEndian: [0, 0, 0, 0, 0, 0, 7, 231],
              ),
              _Info(
                integer: BigInt.parse('0x112233445566'),
                expectLittleEndian: [0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0, 0],
                expectBigEndian: [0, 0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
              ),
              if (name != _Name.uint64)
                _Info(
                  integer: BigInt.parse('-0x112233445566'),
                  expectLittleEndian: [154, 170, 187, 204, 221, 238, 255, 255],
                  expectBigEndian: [255, 255, 238, 221, 204, 187, 170, 154],
                ),
              _Info(
                integer: BigInt.parse('0x110000000000'),
                expectLittleEndian: [0, 0, 0, 0, 0, 0x11, 0, 0],
                expectBigEndian: [0, 0, 0x11, 0, 0, 0, 0, 0],
              ),
              if (name != _Name.uint64)
                _Info(
                  integer: BigInt.parse('-0x110000000000'),
                  expectLittleEndian: [0, 0, 0, 0, 0, 239, 255, 255],
                  expectBigEndian: [255, 255, 239, 0, 0, 0, 0, 0],
                ),
              // Dart int does not support this yet...
              // _Info(
              //   // 2**64-1
              //   integer: BigInt.parse('18446744073709551615'),
              //   expectLittleEndian: [255, 255, 255, 255, 255, 255, 255, 255],
              //   expectBigEndian: [255, 255, 255, 255, 255, 255, 255, 255],
              // ),
              _Info(
                // 2**63-1
                integer: BigInt.parse('9223372036854775807'),
                expectLittleEndian: [255, 255, 255, 255, 255, 255, 255, 127],
                expectBigEndian: [127, 255, 255, 255, 255, 255, 255, 255],
              ),
              if (name != _Name.uint64)
                _Info(
                  // -2**63
                  integer: BigInt.parse('-9223372036854775808'),
                  expectLittleEndian: [0, 0, 0, 0, 0, 0, 0, 128],
                  expectBigEndian: [128, 0, 0, 0, 0, 0, 0, 0],
                ),
            ]) {
              test('$info', () => _body(setter, getter, info, endian));
            }

            test('random loopback', () {
              final random = Random();
              for (var iter = 0; iter < 1000; ++iter) {
                final oldByteData = ByteData(8);
                for (var i = 0; i < oldByteData.lengthInBytes; ++i) {
                  oldByteData.setUint8(i, random.nextInt(256));
                }

                final integer = getter(oldByteData, 0, endian);
                final newByteData = ByteData(8);
                setter(newByteData, 0, integer, endian);

                expect(oldByteData.buffer.asUint8List(),
                    newByteData.buffer.asUint8List());
              }
            });
          });
        }
      });
    }
  });
}

typedef _Setter = void Function(
    ByteData byteData, int byteOffset, BigInt value, Endian endian);
typedef _Getter = BigInt Function(
    ByteData byteData, int byteOffset, Endian endian);

void _body(_Setter setter, _Getter getter, _Info info, Endian endian) {
  final byteData = ByteData(60);
  const byteOffset = 40;

  setter(byteData, byteOffset, info.integer, endian);
  expect(byteData.buffer.asUint8List(byteOffset, 8), info.expectBytes(endian));
  expect(getter(byteData, byteOffset, endian), info.integer);
}

enum _Name { uint64, int64 }

class _Info {
  final BigInt integer;
  final List<int> expectLittleEndian;
  final List<int> expectBigEndian;

  const _Info({
    required this.integer,
    required this.expectLittleEndian,
    required this.expectBigEndian,
  });

  List<int> expectBytes(Endian endian) => switch (endian) {
        Endian.little => expectLittleEndian,
        Endian.big => expectBigEndian,
        _ => throw UnimplementedError(),
      };

  @override
  String toString() =>
      '_Info{integer: $integer, expectLittleEndian: $expectLittleEndian, expectBigEndian: $expectBigEndian}';
}
