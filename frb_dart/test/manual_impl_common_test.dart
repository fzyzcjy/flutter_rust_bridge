@TestOn('vm')
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/manual_impl/_common.dart';
import 'package:test/test.dart';

void main() {
  test('dcoDecodePanicError creates PanicException from raw message', () {
    final exception = dcoDecodePanicError('panic message');

    expect(exception, isA<PanicException>());
    expect(exception.message, 'panic message');
  });

  test('dcoDecodeTimestamp decodes native microseconds timestamp', () {
    final timestamp = dcoDecodeTimestamp(ts: 1234567, isUtc: true);

    expect(
      timestamp,
      DateTime.fromMicrosecondsSinceEpoch(1234567, isUtc: true),
    );
  });

  test('dcoDecodeDuration decodes native microseconds duration', () {
    expect(dcoDecodeDuration(1234567), const Duration(microseconds: 1234567));
  });
}
