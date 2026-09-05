import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/codec/dco.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:test/test.dart';

void main() {
  const codec = DcoCodec<String, _DecodedError>(
    decodeSuccessData: _decodeSuccess,
    decodeErrorData: _decodeError,
  );

  test('DcoCodec decodes successful values', () {
    expect(codec.decodeObject(<dynamic>[0, 'value']), 'success:value');
  });

  test('DcoCodec throws decoded errors', () {
    expect(
      () => codec.decodeObject(<dynamic>[1, 'failure']),
      throwsA(
        isA<_DecodedError>().having((error) => error.value, 'value', 'failure'),
      ),
    );
  });

  test('DcoCodec throws decoded panics', () {
    expect(
      () => codec.decodeObject(<dynamic>[3, 'panic text']),
      throwsA(
        isA<PanicException>().having(
          (error) => error.message,
          'message',
          'panic text',
        ),
      ),
    );
  });

  test('DcoCodec signals stream closure', () {
    expect(
      () => codec.decodeObject(<dynamic>[2]),
      throwsA(isA<CloseStreamException>()),
    );
  });

  test('DcoCodec rejects errors without an error decoder', () {
    const codecWithoutError = DcoCodec<String, _DecodedError>(
      decodeSuccessData: _decodeSuccess,
      decodeErrorData: null,
    );

    expect(
      () => codecWithoutError.decodeObject(<dynamic>[1, 'failure']),
      throwsA(
        isA<Exception>().having(
          (error) => error.toString(),
          'message',
          contains('no decodeErrorData'),
        ),
      ),
    );
  });

  test('DcoCodec rejects malformed two-item action framing', () {
    expect(
      () => codec.decodeObject(<dynamic>[0]),
      throwsA(isA<AssertionError>()),
    );
    expect(
      () => codec.decodeObject(<dynamic>[1]),
      throwsA(isA<AssertionError>()),
    );
    expect(
      () => codec.decodeObject(<dynamic>[3]),
      throwsA(isA<AssertionError>()),
    );
  });
}

String _decodeSuccess(dynamic raw) => 'success:$raw';

_DecodedError _decodeError(dynamic raw) => _DecodedError(raw as String);

class _DecodedError implements Exception {
  final String value;

  const _DecodedError(this.value);
}
