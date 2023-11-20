import 'dart:async';

import 'package:flutter_rust_bridge/src/task.dart';

class BaseHandler {
  /// Execute a normal ffi call. Usually called by generated code instead of manually called.
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    final completer = Completer<dynamic>();
    final sendPort = singleCompletePort(completer);
    task.callFfi(sendPort.nativePort);
    return completer.future.then((dynamic raw) =>
        _transformRust2DartMessage(raw, task.parseSuccessData, task.parseErrorData, wire2apiPanicError));
  }

  /// Similar to [executeNormal], except that this will return synchronously
  S executeSync<S, E extends Object>(SyncTask<S, E> task) {
    final WireSyncReturn syncReturn;
    try {
      syncReturn = task.callFfi();
    } catch (err, st) {
      throw PanicException('EXECUTE_SYNC_ABORT $err $st');
    }
    try {
      final syncReturnAsDartObject = wireSyncReturnIntoDart(syncReturn);
      return _transformRust2DartMessage(
          syncReturnAsDartObject, task.parseSuccessData, task.parseErrorData, wire2apiPanicError);
    } catch (err) {
      rethrow;
    } finally {
      inner.free_WireSyncReturn(syncReturn);
    }
  }

  /// Similar to [executeNormal], except that this will return a [Stream] instead of a [Future].
  Stream<S> executeStream<S, E extends Object>(StreamTask<S, E> task) {
    final func = task.constMeta.debugName;
    final nextIndex = _streamSinkNameIndex.update(func, (value) => value + 1, ifAbsent: () => 0);
    final name = '__frb_streamsink_${func}_$nextIndex';
    final receivePort = broadcastPort(name);
    task.callFfi(receivePort.sendPort.nativePort);

    await for (final raw in receivePort) {
      try {
        yield _transformRust2DartMessage(raw, task.parseSuccessData, task.parseErrorData, wire2apiPanicError);
      } on _CloseStreamException {
        receivePort.close();
        break;
      }
    }
  }
}
