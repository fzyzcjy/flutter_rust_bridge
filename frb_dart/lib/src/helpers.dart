import 'dart:async';

import 'package:flutter_rust_bridge/src/basic.dart';
import 'package:meta/meta.dart';

/// Allow custom setup hooks before ffi can be executed.
/// All other ffi calls will wait (async) until the setup ffi finishes.
///
/// Usage: Please call [setupMixinConstructor] inside the constructor of your class
mixin FlutterRustBridgeSetupMixin<T extends FlutterRustBridgeWireBase> on FlutterRustBridgeBase<T> {
  Completer<void>? _setupCompleter;

  void setupMixinConstructor() {
    () async {
      final setupFfiCallFuture = setupFfiCall();

      assert(_setupCompleter == null);
      _setupCompleter = Completer();

      try {
        await setupFfiCallFuture;
      } finally {
        _setupCompleter!.complete();
      }
    }();
  }

  @override
  Future<S> execute<S>(
      String debugName, void Function(int port) callFfi, S Function(dynamic) parseSuccessData, dynamic hint) async {
    final setupCompleter = _setupCompleter;
    if (setupCompleter != null && !setupCompleter.isCompleted) {
      await setupCompleter.future;
    }

    return await super.execute(debugName, callFfi, parseSuccessData, hint);
  }

  @protected
  SetupFfiCall get setupFfiCall;

  @protected
  Duration get setupTimeout => const Duration(seconds: 5);
}

typedef SetupFfiCall = Future<void> Function();
