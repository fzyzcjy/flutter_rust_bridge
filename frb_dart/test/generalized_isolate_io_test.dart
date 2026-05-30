@TestOn('vm')
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:test/test.dart';

void main() {
  test('broadcastPort can opt out of keeping the isolate alive', () {
    final port = broadcastPort(
      'frb_generalized_isolate_test',
      keepIsolateAlive: false,
    );

    port.close();
  });
}
