import 'dart:async';

import 'package:flutter_rust_bridge/src/basic.dart';
import 'package:meta/meta.dart';

/// Allow custom setup hooks before ffi can be executed.
/// All other ffi calls will wait (async) until the setup ffi finishes.
///
/// Usage:
///
/// 1. Please call [setupMixinConstructor] inside the constructor of your class.
/// 2. Inside your [setup], please call ffi functions with hint=[kHintSetup].
mixin FlutterRustBridgeSetupMixin<T extends FlutterRustBridgeWireBase> on FlutterRustBridgeBase<T> {
  static const kHintSetup = _FlutterRustBridgeSetupMixinSkipWaitHint._();

  final _setupCompleter = Completer<void>();

  void setupMixinConstructor() {
    () async {
      try {
        await setup();
      } finally {
        _setupCompleter.complete();
      }
    }();
  }

  @override
  Future<S> execute<S>(
      String debugName, void Function(int port) callFfi, S Function(dynamic) parseSuccessData, dynamic hint) async {
    if (!_setupCompleter.isCompleted && hint is! _FlutterRustBridgeSetupMixinSkipWaitHint) {
      await _setupCompleter.future;
    }

    return await super.execute(debugName, callFfi, parseSuccessData, hint);
  }

  @protected
  Future<void> setup();

  @protected
  Duration get setupTimeout => const Duration(seconds: 5);
}

class _FlutterRustBridgeSetupMixinSkipWaitHint {
  const _FlutterRustBridgeSetupMixinSkipWaitHint._();
}
