// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `event_listener_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

import 'dart:async';

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/event_listener_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart register event listener & create event with delay', () async {
    unawaited(expectLater(registerEventListenerTwinRustAsync(),
        emits(EventTwinRustAsync(address: 'foo', payload: 'bar'))));
    await Future.delayed(const Duration(milliseconds: 20));
    await createEventTwinRustAsync(address: 'foo', payload: 'bar');
    await closeEventListenerTwinRustAsync();
  });

  // #1836
  test('when send event before async gap, should receive it', () async {
    final logs = <String>[];

    final stream = registerEventListenerTwinRustAsync();
    stream.listen((event) => logs.add(event.address));

    // main call to test #1836
    createEventSyncTwinRustAsync(address: 'one', payload: '');

    await Future.delayed(Duration.zero);
    createEventSyncTwinRustAsync(address: 'two', payload: '');

    await closeEventListenerTwinRustAsync();

    expect(logs, ['one', 'two']);
  });

  // #1836
  test('when Rust send event after Dart close stream', () async {
    final stream = registerEventListenerTwinRustAsync();
    await Future.delayed(Duration.zero);
    final subscription = stream.listen((_) {});
    await Future.delayed(Duration.zero);
    unawaited(subscription.cancel());
    createEventSyncTwinRustAsync(address: '1', payload: '');
  });
}
