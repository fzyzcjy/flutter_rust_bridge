@TestOn('browser')
import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_isolate/_web.dart';
import 'package:test/test.dart';

@JS('BroadcastChannel.prototype.postMessage')
external JSFunction get _postMessage;

@JS('BroadcastChannel.prototype.postMessage')
external set _postMessage(JSFunction value);

@JS('crypto.randomUUID')
external JSFunction? get _randomUUID;

@JS('crypto.randomUUID')
external set _randomUUID(JSFunction? value);

void main() {
  test('failed initialization retries without randomUUID', () async {
    final original = _postMessage;
    final originalRandomUUID = _randomUUID;
    try {
      _randomUUID = null;
      _postMessage = _failPostMessage.toJS;
      await expectLater(initializeBroadcastChannel(), throwsA(anything));
      _postMessage = original;
      await initializeBroadcastChannel();
    } finally {
      _postMessage = original;
      _randomUUID = originalRandomUUID;
    }
  });
}

void _failPostMessage(JSAny? value) {
  throw StateError('injected initialization failure');
}
