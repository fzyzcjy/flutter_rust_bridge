import 'package:flutter_rust_bridge/src/consts.dart';
import 'package:test/test.dart';

void main() {
  test('kIsWeb is false on the VM', () {
    expect(kIsWeb, isFalse);
  });
}
