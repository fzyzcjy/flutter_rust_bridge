import 'dart:async';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/old/basic.dart';
import 'package:flutter_rust_bridge/src/old/platform_independent.dart';
import 'package:meta/meta.dart';
import 'package:uuid/uuid.dart';

export '../ffi.dart';

/// Add a timeout to [executeNormal]
mixin FlutterRustBridgeTimeoutMixin<T extends FlutterRustBridgeWireBase> on FlutterRustBridgeBase<T> {
  @override
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    // capture a stack trace at *here*, such that when timeout, can have a good stack trace
    final stackTrace = StackTrace.current;

    final timeLimitForExecuteNormal = this.timeLimitForExecuteNormal;

    var future = super.executeNormal(task);
    if (timeLimitForExecuteNormal != null) {
      future = future.timeout(timeLimitForExecuteNormal,
          onTimeout: () =>
              throw FlutterRustBridgeTimeoutException(timeLimitForExecuteNormal, task.debugName, stackTrace));
    }

    return future;
  }

  /// The time limit for methods using [executeNormal]. Return null means *disable* this functionality.
  @protected
  Duration? get timeLimitForExecuteNormal;
}

/// Exception when timeout happens using [FlutterRustBridgeTimeoutMixin]
@immutable
class FlutterRustBridgeTimeoutException {
  /// The duration to trigger timeout
  final Duration duration;

  /// debugName of the task, usually the ffi function name
  final String debugName;

  /// The stack trace of the error
  final StackTrace stackTrace;

  const FlutterRustBridgeTimeoutException(this.duration, this.debugName, this.stackTrace);

  @override
  String toString() =>
      'FlutterRustBridgeTimeoutException(debugName=$debugName, duration=$duration, stackTrace=$stackTrace)';
}

List<T?> mapNonNull<T, I>(List<I?> items, T Function(I) mapper) {
  final out = List<T?>.filled(items.length, null);
  for (var i = 0; i < items.length; ++i) {
    final item = items[i];
    if (item != null) out[i] = mapper(item);
  }
  return out;
}
