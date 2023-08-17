import 'package:test/test.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

final i64maxb = BigInt.parse('0x7FFFFFFFFFFFFFFF');
final i64minb = BigInt.parse('-0x8000000000000000');
final u64maxb = BigInt.parse('0xFFFFFFFFFFFFFFFF');
const isWeb = bool.fromEnvironment('dart.library.html');

void main() {
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
}
