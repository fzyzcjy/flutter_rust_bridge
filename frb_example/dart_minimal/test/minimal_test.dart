import 'dart:async';

import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });

  test('dart call simpleThreadPoolFn', () async {
    expect(await simpleThreadPoolFn(a: 100), 100);
  });

  test('dart call simpleSyncFn', () async {
    expect(simpleSyncFn(a: 200), 200);
  });

  test('dart call dartCallbackThreadPoolFn', () async {
    final completer = Completer<void>();

    await dartCallbackThreadPoolFn(cb: () {
      return 'dart_callback_thread_pool_fn';
    }, success: () {
      completer.complete();
    });

    await completer.future.timeout(const Duration(seconds: 1));
  });

  test('dart call dartCallbackSyncFn', () async {
    final completer = Completer<void>();

    dartCallbackSyncFn(cb: () {
      return 'dart_callback_sync_fn';
    }, success: () {
      completer.complete();
    });

    await completer.future.timeout(const Duration(seconds: 1));
  });

  test('dart call dartOpaqueThreadPoolFn', () async {
    expect(await dartOpaqueThreadPoolFn(opaque: 'dartOpaqueThreadPoolFn'),
        'dartOpaqueThreadPoolFn');
  });

  test('dart call dartOpaqueSyncFn', () async {
    expect(dartOpaqueSyncFn(opaque: 'dartOpaqueSyncFn'), 'dartOpaqueSyncFn');
  });

  test('dart call streamSinkThreadPoolFn', () async {
    var sink = streamSinkThreadPoolFn();
    expect(await sink.toList(), [1, 2, 3]);
  });

  test('dart call streamSinkSyncFn', () async {
    var sink = streamSinkSyncFn();
    expect(await sink.toList(), [1, 2, 3]);
  });

  print('Action: Configure tests (end)');
}
