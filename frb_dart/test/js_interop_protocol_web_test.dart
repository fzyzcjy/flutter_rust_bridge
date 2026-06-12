@TestOn('browser')
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';
import 'package:test/test.dart';

@JS('BigInt64Array')
extension type _JSBigInt64Array(JSObject _) implements JSObject {}

@JS('BigUint64Array')
extension type _JSBigUint64Array(JSObject _) implements JSObject {}

void main() {
  test('DCO primitive integer decoder accepts integral JavaScript numbers', () {
    expect(dcoDecodePrimitiveInt(42.0), 42);
  });

  test('DCO string decoder accepts JavaScript strings without dartify', () {
    expect(dcoDecodeString('hello'.toJS), 'hello');
  });

  test('CST int64 list encoder emits BigInt64Array', () {
    final encoded = cstEncodeInt64List([BigInt.from(-1), BigInt.zero]);

    expect(encoded.isA<_JSBigInt64Array>(), isTrue);
  });

  test('CST uint64 list encoder emits BigUint64Array', () {
    final encoded = cstEncodeUint64List([
      BigInt.zero,
      BigInt.parse('18446744073709551615'),
    ]);

    expect(encoded.isA<_JSBigUint64Array>(), isTrue);
  });
}
