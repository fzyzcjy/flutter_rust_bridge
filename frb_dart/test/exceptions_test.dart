import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:test/test.dart';

void main() {
  test('PanicException', () {
    expect(PanicException('hello').toString(), contains('PanicException'));
  });

  test('AnyhowException', () {
    expect(AnyhowException('hello').toString(), contains('AnyhowException'));
  });

  test('MissingHeaderException', () {
    expect(const MissingHeaderException().toString(),
        contains('cannot be shared'));
  });

  test('PlatformMismatchException', () {
    expect(const PlatformMismatchException().toString(),
        contains('Not implemented on'));
  });

  test('UnmodifiableTypedListException', () {
    expect(const UnmodifiableTypedListException().toString(),
        contains('Cannot modify'));
  });
}
