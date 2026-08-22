// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

import 'dart:async';

import 'package:frb_example_pure_dart/src/rust/api/event_listener.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();
  if (kIsWeb) return;

  const cancellationTimeout = Duration(seconds: 5);

  test('dart register event listener & create event with delay', () async {
    unawaited(
      expectLater(
        await registerEventListenerTwinNormal(),
        emits(EventTwinNormal(address: 'foo', payload: 'bar')),
      ),
    );
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
    // Cancelling must complete promptly even though Rust never sends another
    // event nor closes the stream (regression for the idle-cancel deadlock).
    await subscription.cancel().timeout(const Duration(seconds: 5));
    // Rust sending after the Dart side already cancelled must not crash.
    createEventSyncTwinNormal(address: '1', payload: '');
  });
  // FRB_INTERNAL_GENERATOR_DISABLE_DUPLICATOR_END

  test('idle event listener subscription cancels immediately', () async {
    final stream = await registerEventListenerTwinNormal();
    addTearDown(closeEventListenerTwinNormal);
    final subscription = stream.listen((_) {});

    await subscription.cancel().timeout(cancellationTimeout);

    final sent = await tryCreateEventTwinNormal(address: 'after', payload: '');
    if (!kIsWeb) expect(sent, isFalse);
  });

  test('settled idle event listener subscription cancels', () async {
    final stream = await registerEventListenerTwinNormal();
    addTearDown(closeEventListenerTwinNormal);
    final subscription = stream.listen((_) {});
    await Future<void>.delayed(Duration.zero);

    await subscription.cancel().timeout(cancellationTimeout);
  });

  test('paused idle event listener subscription cancels', () async {
    final stream = await registerEventListenerTwinNormal();
    addTearDown(closeEventListenerTwinNormal);
    final subscription = stream.listen((_) {})..pause();

    await subscription.cancel().timeout(cancellationTimeout);
  });

  test('event listener subscription cancels inside onData', () async {
    final stream = await registerEventListenerTwinNormal();
    addTearDown(closeEventListenerTwinNormal);
    final cancelled = Completer<void>();
    late final StreamSubscription<EventTwinNormal> subscription;
    subscription = stream.listen((_) {
      unawaited(
        _cancelAndComplete(
          subscription: subscription,
          completer: cancelled,
        ),
      );
    });

    await createEventTwinNormal(address: 'first', payload: '');
    await cancelled.future.timeout(cancellationTimeout);

    final sent = await tryCreateEventTwinNormal(address: 'after', payload: '');
    if (!kIsWeb) expect(sent, isFalse);
  });

  test('paused event listener buffers data until resume', () async {
    final stream = await registerEventListenerTwinNormal();
    addTearDown(closeEventListenerTwinNormal);
    final events = <EventTwinNormal>[];
    final ready = Completer<void>();
    final buffered = Completer<void>();
    final subscription = stream.listen((event) {
      events.add(event);
      if (event.address == 'ready') ready.complete();
      if (event.address == 'paused') buffered.complete();
    });

    await Future<void>.delayed(const Duration(milliseconds: 20));
    await createEventTwinNormal(address: 'ready', payload: 'event');
    await ready.future.timeout(cancellationTimeout);
    subscription.pause();
    await Future<void>.delayed(Duration.zero);

    await createEventTwinNormal(address: 'paused', payload: 'event');
    await Future<void>.delayed(Duration.zero);
    expect(buffered.isCompleted, isFalse);
    expect(events, [EventTwinNormal(address: 'ready', payload: 'event')]);

    subscription.resume();
    await buffered.future.timeout(cancellationTimeout);
    expect(events, [
      EventTwinNormal(address: 'ready', payload: 'event'),
      EventTwinNormal(address: 'paused', payload: 'event'),
    ]);
    await subscription.cancel().timeout(cancellationTimeout);
  }, skip: kIsWeb);

  test('new event listener works after previous subscription cancels',
      () async {
    final firstStream = await registerEventListenerTwinNormal();
    addTearDown(closeEventListenerTwinNormal);
    final firstSubscription = firstStream.listen((_) {});
    await firstSubscription.cancel().timeout(cancellationTimeout);

    final secondStream = await registerEventListenerTwinNormal();
    final secondEvents = secondStream.toList();
    await createEventTwinNormal(address: 'new', payload: 'listener');
    await closeEventListenerTwinNormal();

    expect(
      await secondEvents.timeout(cancellationTimeout),
      [EventTwinNormal(address: 'new', payload: 'listener')],
    );
  });

  test('event listener supports repeated register and cancel cycles', () async {
    addTearDown(closeEventListenerTwinNormal);

    for (var index = 0; index < 10; index++) {
      final stream = await registerEventListenerTwinNormal();
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
