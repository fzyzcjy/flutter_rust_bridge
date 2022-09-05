import 'package:test/test.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

const i64max = 0x7FFFFFFFFFFFFFFF;
const u64max = 0xFFFFFFFFFFFFFFFF;
final i64maxb = BigInt.from(i64max);
final u64maxb = BigInt.parse('0xFFFFFFFFFFFFFFFF');

void main() {
  test('Int64List', () {
    final buf = Int64List.fromList([i64max]);
    expect(buf[0], i64maxb, reason: 'max');
    buf[0] += BigInt.one;
    expect(buf[0], i64maxb, reason: 'max+1 (clamped)');
  });
  test('Uint64List', () {
    final buf = Uint64List.fromList([u64max, i64max]);
    expect(buf[0], u64maxb, reason: 'max');
    expect(buf[1], i64maxb, reason: 'i64max');
    buf[0] += BigInt.one;
    expect(buf[0], u64maxb, reason: 'max+1 (clamped)');
    buf[1] += BigInt.one;
    expect(buf[1], i64maxb + BigInt.one, reason: 'i64max+1');
  });
}
