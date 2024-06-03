// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `event_listener_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

import 'dart:async';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/event_listener_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart register event listener & create event with delay', () async {
    unawaited(expectLater(await registerEventListenerTwinSse(),
        emits(EventTwinSse(address: 'foo', payload: 'bar'))));
    await Future.delayed(const Duration(milliseconds: 20));
    await createEventTwinSse(address: 'foo', payload: 'bar');
    await closeEventListenerTwinSse();
  });

  // #1836
  test('when send event before async gap, should receive it', () async {
    final logs = <String>[];

    final stream = await registerEventListenerTwinSse();
    stream.listen((event) => logs.add(event.address));

    // main call to test #1836
    await createEventTwinSse(address: 'one', payload: '');

    await createEventTwinSse(address: 'two', payload: '');

    await closeEventListenerTwinSse();

    await Future.delayed(const Duration(seconds: 1));

    expect(logs, ['one', 'two']);
  });
}
