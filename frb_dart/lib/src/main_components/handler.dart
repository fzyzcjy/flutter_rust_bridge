import 'dart:async';

import 'package:flutter_rust_bridge/src/dart_opaque/dart_opaque.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/task.dart';
import 'package:flutter_rust_bridge/src/utils/single_complete_port.dart';

/// Generically handles a Dart-Rust call.
class BaseHandler {
  /// Execute a normal ffi call. Usually called by generated code instead of manually called.
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    final completer = Completer<dynamic>();
    final SendPort sendPort = singleCompletePort(completer);
    task.callFfi(sendPort.nativePort);
    return completer.future.then(task.codec.decodeObject);
  }

  /// Similar to [executeNormal], except that this will return synchronously
  S executeSync<S, E extends Object, WireSyncType>(
      SyncTask<S, E, WireSyncType> task) {
    final WireSyncType syncReturn;
    try {
      syncReturn = task.callFfi();
    } catch (e, s) {
      if (e is FrbException) rethrow;
      // When in Web, because Rust only support `abort` (and not `unwind`)
      // we will get `JSObject0:<RuntimeError: unreachable>`.
      // Here we translate the exception.
      throw PanicException('EXECUTE_SYNC_ABORT $e $s');
    }
    try {
      return task.codec.decodeWireSyncType(syncReturn);
    } finally {
      task.codec.freeWireSyncRust2Dart(
          syncReturn, task.apiImpl.generalizedFrbRustBinding);
    }
  }

  /// When Rust invokes a Dart function
  void dartFnInvoke(List<dynamic> message,
      GeneralizedFrbRustBinding generalizedFrbRustBinding) {
    final [closureDartOpaque, ...args] = message;
    final closureDartObject =
        decodeDartOpaque(closureDartOpaque, generalizedFrbRustBinding)
            as Function;
    Function.apply(closureDartObject, args);
  }
}
