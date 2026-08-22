@TestOn('browser')
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';
import 'package:test/test.dart';

void main() {
  test('wasm module initializer is available to browser callers', () {
    const Future<void> Function({required String root, String wasmBindgenName})
    initializer = initializeWasmModule;

    expect(initializer, isNotNull);
  });
}
