import 'dart:async';

import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/platform_types.dart';
import 'package:flutter_rust_bridge/src/task.dart';

class BaseHandler {
  /// Execute a normal ffi call. Usually called by generated code instead of manually called.
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    final completer = Completer<dynamic>();
    final sendPort = singleCompletePort(completer);
    task.callFfi(sendPort.nativePort);
    return completer.future
        .then((dynamic raw) => _transformRust2DartMessage(raw, task.parseSuccessData, task.parseErrorData));
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
      return _transformRust2DartMessage(syncReturnAsDartObject, task.parseSuccessData, task.parseErrorData);
    } catch (err) {
      rethrow;
    } finally {
      inner.free_WireSyncReturn(syncReturn);
    }
  }

  /// Similar to [executeNormal], except that this will return a [Stream] instead of a [Future].
  Stream<S> executeStream<S, E extends Object>(StreamTask<S, E> task) async* {
    final func = task.constMeta.debugName;
    final nextIndex = _streamSinkNameIndex.update(func, (value) => value + 1, ifAbsent: () => 0);
    final name = '__frb_streamsink_${func}_$nextIndex'; // TODO improve
    final receivePort = broadcastPort(name);
    task.callFfi(receivePort.sendPort.nativePort);

    await for (final raw in receivePort) {
      try {
        yield _transformRust2DartMessage(raw, task.parseSuccessData, task.parseErrorData);
      } on _CloseStreamException {
        receivePort.close();
        break;
      }
    }
  }
}

S _transformRust2DartMessage<S, E extends Object>(
    List<dynamic> raw, S Function(dynamic) parseSuccessData, E Function(dynamic)? parseErrorData) {
  final action = raw[0];
  switch (_Rust2DartAction.values[action]) {
    case _Rust2DartAction.success:
      return _parseData<S>(raw, parseSuccessData);
    case _Rust2DartAction.error:
      throw _parseData<E>(raw, parseErrorData!);
    case _Rust2DartAction.panic:
      throw _parseData<PanicException>(raw, wire2apiPanicError);
    case _Rust2DartAction.closeStream:
      assert(raw.length == 1);
      throw _CloseStreamException();
    default:
      throw Exception('Unsupported message, action=$action raw=$raw');
  }
}

R _parseData<R>(List<dynamic> rawData, R Function(dynamic) function) {
  assert(rawData.length == 2);
  return function(rawData[1]);
}

/// NOTE: Please keep in sync with the Rust side
enum _Rust2DartAction { success, error, closeStream, panic }
