@TestOn('browser')
import 'package:flutter_rust_bridge/src/consts.dart';
import 'package:test/test.dart';

void main() {
  test('kIsWeb is true in browser runtimes', () {
    expect(kIsWeb, isTrue);
  });
}
