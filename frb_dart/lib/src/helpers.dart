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
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> task) async {
    await _beforeExecute(task);
    return await super.executeNormal(task);
  }

  @override
  Stream<S> executeStream<S>(FlutterRustBridgeTask<S> task) async* {
    await _beforeExecute(task);
    yield* super.executeStream(task);
  }

  Future<void> _beforeExecute<S>(FlutterRustBridgeTask<S> task) async {
    if (!_setupCompleter.isCompleted && task.hint is! _FlutterRustBridgeSetupMixinSkipWaitHint) {
      await _setupCompleter.future;
    }
  }

  @protected
  Future<void> setup();

  @protected
  Duration get setupTimeout => const Duration(seconds: 5);
}

class _FlutterRustBridgeSetupMixinSkipWaitHint {
  const _FlutterRustBridgeSetupMixinSkipWaitHint._();
}
