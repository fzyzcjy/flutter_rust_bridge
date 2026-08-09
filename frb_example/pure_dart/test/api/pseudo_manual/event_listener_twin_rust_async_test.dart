// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `event_listener_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

import 'dart:async';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/event_listener_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  const cancellationTimeout = Duration(seconds: 5);

  test('dart register event listener & create event with delay', () async {
    unawaited(
      expectLater(
        await registerEventListenerTwinRustAsync(),
        emits(EventTwinRustAsync(address: 'foo', payload: 'bar')),
      ),
    );
    await Future.delayed(const Duration(milliseconds: 20));
    await createEventTwinRustAsync(address: 'foo', payload: 'bar');
    await closeEventListenerTwinRustAsync();
  });

  // #1836
  test('when send event before async gap, should receive it', () async {
    final logs = <String>[];

    final stream = await registerEventListenerTwinRustAsync();
    stream.listen((event) => logs.add(event.address));

    // main call to test #1836
    await createEventTwinRustAsync(address: 'one', payload: '');

    await createEventTwinRustAsync(address: 'two', payload: '');

    await closeEventListenerTwinRustAsync();

    await Future.delayed(const Duration(seconds: 1));

    expect(logs, ['one', 'two']);
  });

  test('idle event listener subscription cancels immediately', () async {
    final stream = await registerEventListenerTwinRustAsync();
    addTearDown(closeEventListenerTwinRustAsync);
    final subscription = stream.listen((_) {});

    await subscription.cancel().timeout(cancellationTimeout);

    final sent =
        await tryCreateEventTwinRustAsync(address: 'after', payload: '');
    if (!kIsWeb) expect(sent, isFalse);
  });

  test('settled idle event listener subscription cancels', () async {
    final stream = await registerEventListenerTwinRustAsync();
    addTearDown(closeEventListenerTwinRustAsync);
    final subscription = stream.listen((_) {});
    await Future<void>.delayed(Duration.zero);

    await subscription.cancel().timeout(cancellationTimeout);
  });

  test('paused idle event listener subscription cancels', () async {
    final stream = await registerEventListenerTwinRustAsync();
    addTearDown(closeEventListenerTwinRustAsync);
    final subscription = stream.listen((_) {})..pause();

    await subscription.cancel().timeout(cancellationTimeout);
  });

  test('event listener subscription cancels inside onData', () async {
    final stream = await registerEventListenerTwinRustAsync();
    addTearDown(closeEventListenerTwinRustAsync);
    final cancelled = Completer<void>();
    late final StreamSubscription<EventTwinRustAsync> subscription;
    subscription = stream.listen((_) {
      unawaited(
        _cancelAndComplete(
          subscription: subscription,
          completer: cancelled,
        ),
      );
    });

    await createEventTwinRustAsync(address: 'first', payload: '');
    await cancelled.future.timeout(cancellationTimeout);

    final sent =
        await tryCreateEventTwinRustAsync(address: 'after', payload: '');
    if (!kIsWeb) expect(sent, isFalse);
  });

  test('paused event listener buffers data until resume', () async {
    final stream = await registerEventListenerTwinRustAsync();
    addTearDown(closeEventListenerTwinRustAsync);
    final events = <EventTwinRustAsync>[];
    final ready = Completer<void>();
    final buffered = Completer<void>();
    final subscription = stream.listen((event) {
      events.add(event);
      if (event.address == 'ready') ready.complete();
      if (event.address == 'paused') buffered.complete();
    });

    await Future<void>.delayed(const Duration(milliseconds: 20));
    await createEventTwinRustAsync(address: 'ready', payload: 'event');
    await ready.future.timeout(cancellationTimeout);
    subscription.pause();
    await Future<void>.delayed(Duration.zero);

    await createEventTwinRustAsync(address: 'paused', payload: 'event');
    await Future<void>.delayed(Duration.zero);
    expect(buffered.isCompleted, isFalse);
    expect(events, [EventTwinRustAsync(address: 'ready', payload: 'event')]);

    subscription.resume();
    await buffered.future.timeout(cancellationTimeout);
    expect(events, [
      EventTwinRustAsync(address: 'ready', payload: 'event'),
      EventTwinRustAsync(address: 'paused', payload: 'event'),
    ]);
    await subscription.cancel().timeout(cancellationTimeout);
  }, skip: kIsWeb);

  test('new event listener works after previous subscription cancels',
      () async {
    final firstStream = await registerEventListenerTwinRustAsync();
    addTearDown(closeEventListenerTwinRustAsync);
    final firstSubscription = firstStream.listen((_) {});
    await firstSubscription.cancel().timeout(cancellationTimeout);

    final secondStream = await registerEventListenerTwinRustAsync();
    final secondEvents = secondStream.toList();
    await createEventTwinRustAsync(address: 'new', payload: 'listener');
    await closeEventListenerTwinRustAsync();

    expect(
      await secondEvents.timeout(cancellationTimeout),
      [EventTwinRustAsync(address: 'new', payload: 'listener')],
    );
  });

  test('event listener supports repeated register and cancel cycles', () async {
    addTearDown(closeEventListenerTwinRustAsync);

    for (var index = 0; index < 10; index++) {
      final stream = await registerEventListenerTwinRustAsync();
      final subscription = stream.listen((_) {});
      await subscription.cancel().timeout(cancellationTimeout);
    }
  });
}

Future<void> _cancelAndComplete<T>({
  required StreamSubscription<T> subscription,
  required Completer<void> completer,
}) async {
  try {
    await subscription.cancel();
    completer.complete();
  } catch (error, stackTrace) {
    completer.completeError(error, stackTrace);
  }
}
