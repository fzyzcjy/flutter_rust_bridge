@TestOn('vm')
import 'package:flutter_rust_bridge/src/platform_utils/_io.dart';
import 'package:test/test.dart';

void main() {
  test('native maybeDartify preserves its input identity', () {
    final value = Object();

    expect(maybeDartify(value), same(value));
    expect(maybeDartify(null), isNull);
  });
}
