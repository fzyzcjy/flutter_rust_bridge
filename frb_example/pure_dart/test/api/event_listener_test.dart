// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

import 'dart:async';

import 'package:frb_example_pure_dart/src/rust/api/event_listener.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart register event listener & create event with delay', () async {
    unawaited(expectLater(await registerEventListenerTwinNormal(),
        emits(EventTwinNormal(address: 'foo', payload: 'bar'))));
    await Future.delayed(const Duration(milliseconds: 20));
    await createEventTwinNormal(address: 'foo', payload: 'bar');
    await closeEventListenerTwinNormal();
  });

  // #1836
  test('when send event before async gap, should receive it', () async {
    final logs = <String>[];

    final stream = await registerEventListenerTwinNormal();
    stream.listen((event) => logs.add(event.address));

    // main call to test #1836
    await createEventTwinNormal(address: 'one', payload: '');

    await createEventTwinNormal(address: 'two', payload: '');

    await closeEventListenerTwinNormal();

    await Future.delayed(const Duration(seconds: 1));

    expect(logs, ['one', 'two']);
  });

  // FRB_INTERNAL_GENERATOR_DISABLE_DUPLICATOR_START
  // #1836
  test('when Rust send event after Dart close stream', () async {
    final stream = await registerEventListenerTwinNormal();
    await Future.delayed(Duration.zero);
    final subscription = stream.listen((_) {});
    await Future.delayed(Duration.zero);
    unawaited(subscription.cancel());
    createEventSyncTwinNormal(address: '1', payload: '');
  });
  // FRB_INTERNAL_GENERATOR_DISABLE_DUPLICATOR_END
}
