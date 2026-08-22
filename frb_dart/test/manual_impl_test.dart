import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/codec/sse.dart';
import 'package:flutter_rust_bridge/src/consts.dart';
import 'package:flutter_rust_bridge/src/manual_impl/manual_impl.dart';
import 'package:test/test.dart';

void main() {
  test('manual DCO decoders preserve panic text and platform time units', () {
    expect(dcoDecodePanicError('panic').message, 'panic');
    expect(
      dcoDecodeTimestamp(ts: 1234, isUtc: true),
      kIsWeb
          ? DateTime.fromMillisecondsSinceEpoch(1234, isUtc: true)
          : DateTime.fromMicrosecondsSinceEpoch(1234, isUtc: true),
    );
    expect(
      dcoDecodeDuration(1234),
      kIsWeb
          ? const Duration(milliseconds: 1234)
          : const Duration(microseconds: 1234),
    );
  });

  test('SSE panic decoder consumes its length-prefixed UTF-8 message', () {
    const message = 'panic: invalid state';
    final messageBytes = Uint8List.fromList(message.codeUnits);
    final bytes = ByteData(4 + messageBytes.length)
      ..setInt32(0, messageBytes.length, Endian.host)
      ..buffer.asUint8List(4).setAll(0, messageBytes);

    expect(sseDecodePanicError(SseDeserializer(bytes)).message, message);
  });

  test(
    'native manual integer decoders retain signed and unsigned boundaries',
    () {
      expect(dcoDecodeI64(-1), -1);
      expect(dcoDecodeU64(-1), BigInt.parse('18446744073709551615'));
      expect(sseEncodeCastedPrimitiveI64(-9), -9);
      expect(sseEncodeCastedPrimitiveU64(9), BigInt.from(9));
    },
  );
}
