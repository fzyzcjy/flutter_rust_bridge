import 'dart:async';

import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/manual_impl/manual_impl.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
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
    return completer.future.then((dynamic raw) => _transformRust2DartMessage(
        raw, task.parseSuccessData, task.parseErrorData));
  }

  /// Similar to [executeNormal], except that this will return synchronously
  S executeSync<S, E extends Object>(SyncTask<S, E> task) {
    final syncReturn = task.callFfi();
    try {
      final syncReturnAsDartObject = wireSyncReturnIntoDart(syncReturn);
      return _transformRust2DartMessage(
          syncReturnAsDartObject, task.parseSuccessData, task.parseErrorData);
    } finally {
      task.apiImpl.generalizedFrbRustBinding.freeWireSyncReturn(syncReturn);
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
        yield _transformRust2DartMessage(
            raw, task.parseSuccessData, task.parseErrorData);
      } on _CloseStreamException {
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

S _transformRust2DartMessage<S, E extends Object>(List<dynamic> raw,
    S Function(dynamic) parseSuccessData, E Function(dynamic)? parseErrorData) {
  switch (_Rust2DartAction.values[raw[0]]) {
    case _Rust2DartAction.success:
      assert(raw.length == 2);
      return parseSuccessData(raw[1]);

    case _Rust2DartAction.error:
      assert(raw.length == 2);
      if (parseErrorData == null) {
        throw Exception(
            'transformRust2DartMessage received error message, but no parseErrorData to parse it. '
            'Raw data: $raw');
      }
      throw parseErrorData(raw[1]);

    case _Rust2DartAction.panic:
      assert(raw.length == 2);
      throw dcoDecodePanicError(raw[1]);

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
