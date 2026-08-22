import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:test/test.dart';

void main() {
  final decoder = _TestDecoder();

  test('simple decoder returns decoded successful values', () {
    expect(decoder.decode(0), 'success');
  });

  test('simple decoder throws the decoded error', () {
    expect(() => decoder.decode(1), throwsA(isA<_DecodedError>()));
  });

  test('simple decoder throws decoded panic objects', () {
    expect(() => decoder.decode(3), throwsA('panic'));
  });

  test('simple decoder signals stream closure distinctly', () {
    expect(() => decoder.decode(2), throwsA(isA<CloseStreamException>()));
  });

  test('simple decoder rejects unsupported actions', () {
    expect(
      () => decoder.decode(99),
      throwsA(
        isA<Exception>().having(
          (error) => error.toString(),
          'message',
          'Exception: Unsupported message (action=99)',
        ),
      ),
    );
  });
}

class _TestDecoder extends SimpleDecoder<String, _DecodedError> {
  @override
  _DecodedError decodeError() => _DecodedError();

  @override
  Object decodePanic() => 'panic';

  @override
  String decodeSuccess() => 'success';
}

class _DecodedError implements Exception {}
