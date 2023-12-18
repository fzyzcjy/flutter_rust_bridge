// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "syncSse"]}

import 'dart:async';

import 'package:frb_example_pure_dart/src/rust/api/event_listener.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart register event listener & create event with delay', () async {
    unawaited(expectLater(registerEventListenerTwinNormal(),
        emits(EventTwinNormal(address: 'foo', payload: 'bar'))));
    await Future.delayed(const Duration(milliseconds: 20));
    await createEventTwinNormal(address: 'foo', payload: 'bar');
    await closeEventListenerTwinNormal();
  });
}
