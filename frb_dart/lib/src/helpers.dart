import 'dart:async';

import 'package:flutter_rust_bridge/src/basic.dart';
import 'package:flutter_rust_bridge/src/platform_independent.dart';
import 'package:meta/meta.dart';

/// Allow custom setup hooks before ffi can be executed.
/// All other ffi calls will wait (async) until the setup ffi finishes.
///
/// Usage:
///
/// 1. Please call [setupMixinConstructor] inside the constructor of your class.
/// 2. Inside your [setup], please call ffi functions with hint=[kHintSetup].
mixin FlutterRustBridgeSetupMixin<T extends FlutterRustBridgeWireBase> on FlutterRustBridgeBase<T> {
  /// Inside your [setup], please call ffi functions with hint=[kHintSetup].
  static const kHintSetup = _FlutterRustBridgeSetupMixinSkipWaitHint._();

  final _setupCompleter = Completer<void>();

  /// Please call it inside the constructor of your class.
  void setupMixinConstructor() {
    () async {
      try {
        log('FlutterRustBridgeSetupMixin.setupMixinConstructor start setup');
        await setup();
      } finally {
        log('FlutterRustBridgeSetupMixin.setupMixinConstructor complete setup');
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
      log('FlutterRustBridgeSetupMixin.beforeExecute start waiting setup to complete (task=${task.debugName})');
      await _setupCompleter.future;
      log('FlutterRustBridgeSetupMixin.beforeExecute end waiting setup to complete (task=${task.debugName})');
    }
  }

  /// Do your setup logic inside this function.
  @protected
  Future<void> setup();

  /// Configure a logger for error handling.
  @protected
  void log(String message) {}
}

class _FlutterRustBridgeSetupMixinSkipWaitHint {
  const _FlutterRustBridgeSetupMixinSkipWaitHint._();
}

/// Add a timeout to [executeNormal]
mixin FlutterRustBridgeTimeoutMixin<T extends FlutterRustBridgeWireBase> on FlutterRustBridgeBase<T> {
  @override
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> task) async {
    // capture a stack trace at *here*, such that when timeout, can have a good stack trace
    final stackTrace = StackTrace.current;

    return super.executeNormal(task).timeout(timeLimitForExecuteNormal,
        onTimeout: () =>
            throw FlutterRustBridgeTimeoutException(timeLimitForExecuteNormal, task.debugName, stackTrace));
  }

  /// The time limit for methods using [executeNormal]
  @protected
  Duration get timeLimitForExecuteNormal;
}
