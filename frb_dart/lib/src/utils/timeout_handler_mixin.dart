import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/task.dart';
import 'package:meta/meta.dart';

/// Add a timeout to [executeNormal]
mixin TimeoutHandlerMixin on BaseHandler {
  @override
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    // capture a stack trace at *here*, such that when timeout, can have a good stack trace
    final stackTrace = StackTrace.current;

    final timeLimitForExecuteNormal = this.timeLimitForExecuteNormal;

    var future = super.executeNormal(task);
    if (timeLimitForExecuteNormal != null) {
      future = future.timeout(timeLimitForExecuteNormal,
          onTimeout: () => throw FrbTimeoutException(
              timeLimitForExecuteNormal, task.constMeta.debugName, stackTrace));
    }

    return future;
  }

  /// The time limit for methods using [executeNormal]. Return null means *disable* this functionality.
  @protected
  Duration? get timeLimitForExecuteNormal;
}

/// Exception when timeout happens using [FlutterRustBridgeTimeoutMixin]
@immutable
class FrbTimeoutException {
  /// The duration to trigger timeout
  final Duration duration;

  /// debugName of the task, usually the ffi function name
  final String debugName;

  /// The stack trace of the error
  final StackTrace stackTrace;

  /// Constructs the exception
  const FrbTimeoutException(this.duration, this.debugName, this.stackTrace);

  @override
  String toString() =>
      'FrbTimeoutException(debugName=$debugName, duration=$duration, stackTrace=$stackTrace)';
}
