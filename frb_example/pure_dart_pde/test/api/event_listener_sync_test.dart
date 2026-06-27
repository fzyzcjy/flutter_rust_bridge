// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'dart:async';

import 'package:frb_example_pure_dart_pde/src/rust/api/event_listener.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/event_listener_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  // #1836
  test('when Rust send event after Dart close stream', () async {
    final stream = await registerEventListenerTwinNormal();
    await Future.delayed(Duration.zero);
    final subscription = stream.listen((_) {});
    await Future.delayed(Duration.zero);
    unawaited(subscription.cancel());
    createEventSyncTwinNormal(address: '1', payload: '');
  });
}
