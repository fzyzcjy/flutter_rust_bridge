@TestOn('browser')
import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_isolate/_web.dart';
import 'package:test/test.dart';

@JS('BroadcastChannel.prototype.postMessage')
external JSFunction get _postMessage;

@JS('BroadcastChannel.prototype.postMessage')
external set _postMessage(JSFunction value);

void main() {
  test('broadcast initialization can retry after a send error', () async {
    final original = _postMessage;
    try {
      _postMessage = ((JSAny? value) {
        throw StateError('injected initialization failure');
      }).toJS;
      await expectLater(initializeBroadcastChannel(), throwsA(anything));
    } finally {
      _postMessage = original;
    }

    await initializeBroadcastChannel();
  });
}
