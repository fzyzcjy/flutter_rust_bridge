import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:no_rust_async/src/rust/api/no_rust_async.dart';
import 'package:no_rust_async/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

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
    },
    success: () {
      completer.complete();
    });

    await completer.future.timeout(const Duration(seconds: 1));
  });

  test('dart call dartCallbackSyncFn', () async {
    final completer = Completer<void>();

    dartCallbackSyncFn(cb: () {
      return 'dart_callback_sync_fn';
    },
    success: () {
      completer.complete();
    });

    await completer.future.timeout(const Duration(seconds: 1));
  });

  test('dart call dartOpaqueThreadPoolFn', () async {
    expect(await dartOpaqueThreadPoolFn(opaque: 'dartOpaqueThreadPoolFn'), 'dartOpaqueThreadPoolFn');
  });

  test('dart call dartOpaqueSyncFn', () async {
    expect(dartOpaqueSyncFn(opaque: 'dartOpaqueSyncFn'), 'dartOpaqueSyncFn');
  });

  test('dart call dartCallbackSyncFn', () async {

    var sink = streamSinkThreadPoolFn();

    expect(await sink.toList(), [1, 2, 3]);
  });

  test('dart call dartCallbackSyncFn', () async {

    var sink = streamSinkSyncFn();

    expect(await sink.toList(), [1, 2, 3]);
  });
}
