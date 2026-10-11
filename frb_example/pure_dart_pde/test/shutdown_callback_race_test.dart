// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// This isolate never calls `RustLib.init()`: once counted, it would keep the
// isolate count above zero, and the race could not happen.
@TestOn('vm')
library;

import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

void main() {
  test('RustLib.init() works while other isolates shut down', () async {
    final hammer = await _Hammer.start();
    addTearDown(hammer.stop);

    for (var round = 0; round < 20; round++) {
      expect(await _initAndCallInNewIsolate(), 'ok', reason: 'round $round');
    }
  }, timeout: const Timeout(Duration(minutes: 2)));
}

/// Initializes the library in a new isolate and makes one async call there.
Future<String> _initAndCallInNewIsolate() async {
  final port = ReceivePort();
  final isolate = await Isolate.spawn(
    _initAndCall,
    port.sendPort,
    onError: port.sendPort,
  );
  try {
    // A hung init never replies, so any limit far above an init will do.
    final reply = await port.first.timeout(
      const Duration(seconds: 10),
      onTimeout: () => 'hang',
    );
    return '$reply';
  } finally {
    isolate.kill(priority: Isolate.immediate);
    port.close();
  }
}

Future<void> _initAndCall(SendPort reply) async {
  await RustLib.init();
  reply.send(await simpleAdderTwinNormal(a: 1, b: 2) == 3 ? 'ok' : 'wrong sum');
}

typedef _ShutdownCallback = Void Function(Pointer<Void>);

/// An isolate that takes and calls shutdown callbacks in a loop, so that the
/// count keeps dropping to zero and installing the no-op.
class _Hammer {
  _Hammer._(this._control, this._messages);

  final SendPort _control;
  final StreamIterator<Object?> _messages;

  static Future<_Hammer> start() async {
    final port = ReceivePort();
    final messages = StreamIterator<Object?>(port);
    await Isolate.spawn(
      _hammer,
      port.sendPort,
      onError: port.sendPort,
      onExit: port.sendPort,
    );
    await messages.moveNext();
    final control = messages.current;
    if (control is! SendPort) {
      await messages.cancel();
      throw StateError('the hammer did not start: $control');
    }
    return _Hammer._(control, messages);
  }

  /// Stops between take-and-calls; a kill could leave the count above zero.
  Future<void> stop() async {
    _control.send(null);
    await _messages.moveNext();
    final exit = _messages.current;
    await _messages.cancel();
    // A hammer that died early would have let the test pass untested.
    if (exit != null) throw StateError('the hammer failed: $exit');
  }
}

Future<void> _hammer(SendPort messages) async {
  final library = await loadExternalLibrary(
    RustLib.kDefaultExternalLibraryLoaderConfig,
  );
  final create = library.ffiDynamicLibrary.lookupFunction<
      Pointer<NativeFunction<_ShutdownCallback>> Function(),
      Pointer<NativeFunction<_ShutdownCallback>> Function()>(
    'frb_create_shutdown_callback',
  );
  final shutdown = create().asFunction<void Function(Pointer<Void>)>();
  shutdown(nullptr);

  var stopped = false;
  final control = ReceivePort()..listen((_) => stopped = true);
  messages.send(control.sendPort);
  while (!stopped) {
    for (var i = 0; i < 1000; i++) {
      create();
      shutdown(nullptr);
    }
    // Lets the stop message in.
    await Future<void>.delayed(Duration.zero);
  }
  control.close();
}
