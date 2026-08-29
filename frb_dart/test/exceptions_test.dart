import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:test/test.dart';

void main() {
  test('PanicException preserves its message in the diagnostic text', () {
    final exception = PanicException('rust panic details');

    expect(exception, isA<FrbException>());
    expect(exception.message, 'rust panic details');
    expect(exception.toString(), 'PanicException(rust panic details)');
  });

  test('AnyhowException preserves its message in the diagnostic text', () {
    final exception = AnyhowException('anyhow details');

    expect(exception, isA<FrbException>());
    expect(exception.message, 'anyhow details');
    expect(exception.toString(), 'AnyhowException(anyhow details)');
  });

  test('PlatformMismatchException has a stable platform diagnostic', () {
    const exception = PlatformMismatchException();

    expect(exception, isA<FrbException>());
    expect(exception.toString(), 'Not implemented on non-WASM platforms');
  });

  test('UnmodifiableTypedListException has a stable mutation diagnostic', () {
    const exception = UnmodifiableTypedListException();

    expect(exception, isA<FrbException>());
    expect(exception.toString(), 'Cannot modify the length of typed lists.');
  });
}
