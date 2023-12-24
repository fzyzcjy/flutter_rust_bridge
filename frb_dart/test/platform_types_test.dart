import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:test/test.dart';

void main() {
  test('null pointer', () {
    expect(PlatformPointerUtil.isNullPtr(PlatformPointerUtil.nullPtr()), true);
  });
}
