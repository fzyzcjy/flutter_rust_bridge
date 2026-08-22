import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/codec/sse.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:test/test.dart';

void main() {
  final codec = SseCodec<int, _DecodedError>(
    decodeSuccessData: (deserializer) => deserializer.buffer.getInt32(),
    decodeErrorData: (deserializer) =>
        _DecodedError(deserializer.buffer.getInt32()),
  );

  test('SseCodec decodes successful values', () {
    expect(codec.decodeObject(_message(action: 0, value: 42)), 42);
  });

  test('SseCodec throws decoded errors', () {
    expect(
      () => codec.decodeObject(_message(action: 1, value: -7)),
      throwsA(isA<_DecodedError>().having((error) => error.value, 'value', -7)),
    );
  });

  test('SseCodec signals stream closure', () {
    expect(
      () => codec.decodeObject(_message(action: 2)),
      throwsA(isA<CloseStreamException>()),
    );
  });

  test('SseCodec decodes panic text', () {
    final bytes = ByteData(1 + 4 + 5)
      ..setUint8(0, 3)
      ..setInt32(1, 5, Endian.host);
    bytes.buffer.asUint8List(5, 5).setAll(0, 'panic'.codeUnits);

    expect(
      () => codec.decodeObject(bytes.buffer.asUint8List()),
      throwsA(
        isA<PanicException>().having(
          (error) => error.message,
          'message',
          'panic',
        ),
      ),
    );
  });

  test('SseCodec rejects trailing bytes', () {
    final bytes = ByteData(10)
      ..setUint8(0, 0)
      ..setInt32(1, 1, Endian.host)
      ..setUint8(9, 99);

    expect(
      () => codec.decodeObject(bytes.buffer.asUint8List()),
      throwsA(isA<AssertionError>()),
    );
  });

  test('SseCodec fallback reports DCO panic messages', () {
    expect(
      () => codec.decodeObject(<dynamic>[3, 'worker panic']),
      throwsA(
        isA<PanicException>().having(
          (error) => error.message,
          'message',
          'worker panic',
        ),
      ),
    );
  });

  test('SseCodec rejects errors without an error decoder', () {
    final codecWithoutError = SseCodec<int, _DecodedError>(
      decodeSuccessData: (deserializer) => deserializer.buffer.getInt32(),
      decodeErrorData: null,
    );

    expect(
      () => codecWithoutError.decodeObject(_message(action: 1, value: 9)),
      throwsA(
        isA<Exception>().having(
          (error) => error.toString(),
          'message',
          contains('no decodeErrorData'),
        ),
      ),
    );
  });
}

Uint8List _message({required int action, int? value}) {
  final bytes = ByteData(value == null ? 1 : 5)..setUint8(0, action);
  if (value != null) {
    bytes.setInt32(1, value, Endian.host);
  }
  return bytes.buffer.asUint8List();
}

class _DecodedError implements Exception {
  final int value;

  const _DecodedError(this.value);
}
