import 'dart:async';

import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';

// NOTE XXX copy from: https://github.com/dart-archive/isolate/blob/master/lib/ports.dart
// Because [package:isolate] is not maintained anymore, so the code is copied and maintained by ourselves.

/// Create a [SendPort] that accepts only one message.
///
/// When the first message is received, the [callback] function is
/// called with the message as argument,
/// and the [completer] is completed with the result of that call.
/// All further messages are ignored.
///
/// If `callback` is omitted, it defaults to an identity function.
/// The `callback` call may return a future, and the completer will
/// wait for that future to complete. If [callback] is omitted, the
/// message on the port must be an instance of [R].
///
/// If [timeout] is supplied, it is used as a limit on how
/// long it can take before the message is received. If a
/// message isn't received in time, the [onTimeout] is called,
/// and `completer` is completed with the result of that call
/// instead.
/// The [callback] function will not be interrupted by the time-out,
/// as long as the initial message is received in time.
/// If `onTimeout` is omitted, it defaults to completing the `completer` with
/// a [TimeoutException].
///
/// The [completer] may be a synchronous completer. It is only
/// completed in response to another event, either a port message or a timer.
///
/// Returns the `SendPort` expecting the single message.
SendPort singleCompletePort<R, P>(
  Completer<R> completer,
  // {
  // FutureOr<R> Function(P message)? callback,
  // Duration? timeout,
  // FutureOr<R> Function()? onTimeout,
  // }
) {
  // NOTE: Since we never use those complex arguments, we comment out that part
  // if (callback == null && timeout == null) {
  return _singleCallbackPort<Object>((response) {
    _castComplete<R>(completer, response);
  });
  // }
  // var responsePort = RawReceivePort();
  // Timer? timer;
  // if (callback == null) {
  //   responsePort.handler = (response) {
  //     responsePort.close();
  //     timer?.cancel();
  //     _castComplete<R>(completer, response);
  //   };
  // } else {
  //   var zone = Zone.current;
  //   var action = zone.registerUnaryCallback((response) {
  //     try {
  //       // Also catch it if callback throws.
  //       completer.complete(callback(response as P));
  //     } catch (error, stack) {
  //       completer.completeError(error, stack);
  //     }
  //   });
  //   responsePort.handler = (response) {
  //     responsePort.close();
  //     timer?.cancel();
  //     zone.runUnary(action, response as P);
  //   };
  // }
  // if (timeout != null) {
  //   timer = Timer(timeout, () {
  //     responsePort.close();
  //     if (onTimeout != null) {
  //       /// workaround for incomplete generic parameters promotion.
  //       /// example is available in 'TimeoutFirst with invalid null' test
  //       try {
  //         completer.complete(Future.sync(onTimeout));
  //       } catch (e, st) {
  //         completer.completeError(e, st);
  //       }
  //     } else {
  //       completer
  //           .completeError(TimeoutException('Future not completed', timeout));
  //     }
  //   });
  // }
  // return responsePort.sendPort;
}

/// Helper function for [singleCallbackPort].
///
/// Replace [singleCallbackPort] with this
/// when removing the deprecated parameters.
SendPort _singleCallbackPort<P>(void Function(P) callback) {
  var responsePort = RawReceivePort();
  var zone = Zone.current;
  callback = zone.registerUnaryCallback(callback);
  responsePort.handler = (response) {
    responsePort.close();
    zone.runUnary(callback, response as P);
  };
  return responsePort.sendPort;
}

// Helper function that casts an object to a type and completes a
// corresponding completer, or completes with the error if the cast fails.
void _castComplete<R>(Completer<R> completer, Object? value) {
  try {
    completer.complete(value as R);
  } catch (error, stack) {
    completer.completeError(error, stack);
  }
}
