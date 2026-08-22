@TestOn('browser')
library;

import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

void main() {
  /// Replies to readiness probes only after the receive port is listening.
  test('broadcast port acknowledges readiness after listen', () async {
    final channelName =
        'frb-readiness-${DateTime.now().microsecondsSinceEpoch}';
    final port = broadcastPort(channelName);
    final subscription = port.listen((_) {});
    final probe = web.BroadcastChannel(channelName);
    addTearDown(() async {
      await subscription.cancel();
      port.close();
      probe.close();
    });

    final acknowledged = const web.EventStreamProvider<web.MessageEvent>(
      'message',
    ).forTarget(probe).first;
    probe.postMessage('__flutter_rust_bridge_ready'.toJS);

    expect(
      (await acknowledged.timeout(const Duration(seconds: 1))).data,
      '__flutter_rust_bridge_ready',
    );
  });
}
