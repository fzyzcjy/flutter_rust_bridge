import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/test.dart';

void main() {
  test('RustStreamSink stream before setup throws actionable StateError', () {
    expect(
      () => RustStreamSink<int>().stream,
      throwsA(
        isA<StateError>().having(
          (error) => error.message,
          'message',
          allOf(
            contains('RustStreamSink.stream is not ready yet'),
            contains('generated flutter_rust_bridge API'),
          ),
        ),
      ),
    );
  });
}
