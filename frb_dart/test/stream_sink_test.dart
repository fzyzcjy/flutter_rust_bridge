import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/test.dart';

void main() {
  test('RustStreamSink stream is available before setup', () {
    expect(() => RustStreamSink<int>().stream, returnsNormally);
  });
}
