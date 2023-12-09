import 'dart:async';

import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/manual_impl/manual_impl.dart';
import 'package:flutter_rust_bridge/src/task.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:flutter_rust_bridge/src/utils/single_complete_port.dart';

/// Generically handles a Dart-Rust call.
class BaseHandler {
  /// Execute a normal ffi call. Usually called by generated code instead of manually called.
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    final completer = Completer<dynamic>();
    final SendPort sendPort = singleCompletePort(completer);
    task.callFfi(sendPort.nativePort);
    return completer.future.then(task.codec.decode);
  }

  /// Similar to [executeNormal], except that this will return synchronously
  S executeSync<S, E extends Object, R>(SyncTask<S, E, R> task) {
    final R syncReturn;
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
      final syncReturnAsDartObject = wireSyncReturnIntoDart(syncReturn);
      return task.codec.decode(syncReturnAsDartObject);
    } finally {
      task.codec.freeWireSyncReturn(
          syncReturn, task.apiImpl.generalizedFrbRustBinding);
    }
  }

  /// Similar to [executeNormal], except that this will return a [Stream] instead of a [Future].
  Stream<S> executeStream<S, E extends Object>(StreamTask<S, E> task) async* {
    final portName =
        ExecuteStreamPortGenerator.create(task.constMeta.debugName);
    final receivePort = broadcastPort(portName);

    task.callFfi(receivePort.sendPort.nativePort);

    await for (final raw in receivePort) {
      try {
        yield task.codec.decode(raw);
      } on CloseStreamException {
        receivePort.close();
        break;
      }
    }
  }

  /// When Rust invokes a Dart function
  Future<void> dartFnInvoke(List<dynamic> message) async {
    final [closure, ...args] = message;
    final output = await Future.value(Function.apply(closure, args));
    throw Exception("call rust");
  }
}
