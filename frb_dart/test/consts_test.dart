import 'package:flutter_rust_bridge/src/consts.dart';
import 'package:test/test.dart';

void main() {
  test('kIsWeb identifies the active runtime', () {
    expect(kIsWeb, identical(0, 0.0));
  });
}
