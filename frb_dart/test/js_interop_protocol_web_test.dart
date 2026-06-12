@TestOn('browser')
import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/manual_impl/manual_impl.dart';
import 'package:test/test.dart';

void main() {
  test('DCO primitive integer decoder accepts integral JavaScript numbers', () {
    expect(dcoDecodePrimitiveInt(42.0), 42);
  });

  test('DCO string decoder accepts JavaScript strings without dartify', () {
    expect(dcoDecodeString('hello'.toJS), 'hello');
  });
}
