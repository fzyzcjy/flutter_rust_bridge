import 'dart:async';

import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/manual_impl.dart';
import 'package:flutter_rust_bridge/src/platform_types.dart';
import 'package:flutter_rust_bridge/src/task.dart';
import 'package:flutter_rust_bridge/src/utils/single_complete_port.dart';

import 'generalized_isolate/generalized_isolate.dart';

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
    } finally {
      inner.free_WireSyncReturn(syncReturn);
    }
  }

  /// Similar to [executeNormal], except that this will return a [Stream] instead of a [Future].
  Stream<S> executeStream<S, E extends Object>(StreamTask<S, E> task) async* {
    final portName = _ExecuteStreamPortNamer._nextName(task.constMeta.debugName);
    final receivePort = broadcastPort(portName);

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

class _ExecuteStreamPortNamer {
  static final _streamSinkNameIndex = <String, int>{};

  static String _nextName(String funcName) {
    final nextIndex = _streamSinkNameIndex.update(funcName, (value) => value + 1, ifAbsent: () => 0);
    return '__frb_streamsink_${funcName}_$nextIndex';
  }
}

S _transformRust2DartMessage<S, E extends Object>(
    List<dynamic> raw, S Function(dynamic) parseSuccessData, E Function(dynamic)? parseErrorData) {
  switch (_Rust2DartAction.values[raw[0]]) {
    case _Rust2DartAction.success:
      assert(raw.length == 2);
      return parseSuccessData(raw);

    case _Rust2DartAction.error:
      assert(raw.length == 2);
      throw parseErrorData!(raw);

    case _Rust2DartAction.panic:
      assert(raw.length == 2);
      throw wire2apiPanicError(raw);

    case _Rust2DartAction.closeStream:
      assert(raw.length == 1);
      throw _CloseStreamException();

    default:
      throw Exception('Unsupported message (raw=$raw)');
  }
}

/// NOTE: Please keep in sync with the Rust side
enum _Rust2DartAction { success, error, closeStream, panic }

class _CloseStreamException implements Exception {}
