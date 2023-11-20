import 'package:meta/meta.dart';

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
