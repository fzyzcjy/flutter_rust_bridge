import 'dart:async';

import 'package:flutter_rust_bridge/src/platform_independent.dart';
import 'package:flutter_rust_bridge/src/utils.dart';
import 'package:meta/meta.dart';
import 'ffi.dart';
export 'ffi.dart';
import 'isolate.dart';
export 'isolate.dart';

final _instances = <Type>{};
final _streamSinkNameIndex = <String, int>{};

class _DropIdPortGenerator {
  static final instance = _DropIdPortGenerator._();
  _DropIdPortGenerator._();

  int nextPort = 0;
  String create() => '__frb_dart_opaque_drop_${nextPort++}';
}

/// Base class for generated bindings of Flutter Rust Bridge.
/// Normally, users do not extend this class manually. Instead,
/// users should directly use the generated class.
abstract class FlutterRustBridgeBase<T extends FlutterRustBridgeWireBase> {
  FlutterRustBridgeBase(this.inner) {
    _sanityCheckSingleton();
    _setUpRustToDartComm();
  }

  @protected
  final T inner;

  late final _dropPort = _initDropPort();
  NativePortType get dropPort => _dropPort.sendPort.nativePort;

  ReceivePort _initDropPort() {
    var port = broadcastPort(_DropIdPortGenerator.instance.create());
    port.listen((message) {
      inner.drop_dart_object(message);
    });
    return port;
  }

  void dispose() {
    _dropPort.close();
  }

  void _sanityCheckSingleton() {
    if (_instances.contains(runtimeType)) {
      throw Exception(
        'Subclasses of `FlutterRustBridgeBase` should be singletons - '
        'there should not be two instances (runtimeType=$runtimeType)',
      );
    }
    _instances.add(runtimeType);
  }

  void _setUpRustToDartComm() {
    inner.storeDartPostCObject();
  }

  /// Execute a normal ffi call. Usually called by generated code instead of manually called.
  @protected
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> task) {
    final completer = Completer<dynamic>();
    final sendPort = singleCompletePort(completer);
    task.callFfi(sendPort.nativePort);
    return completer.future.then((dynamic raw) =>
        _transformRust2DartMessage(raw, task.parseSuccessData));
  }

  /// Similar to [executeNormal], except that this will return synchronously
  @protected
  S executeSync<S>(FlutterRustBridgeSyncTask task) {
    final WireSyncReturn syncReturn;
    try {
      syncReturn = task.callFfi();
    } catch (err, st) {
      throw FfiException('EXECUTE_SYNC_ABORT', '$err', st);
    }
    try {
      final syncReturnAsDartObject = wireSyncReturnIntoDart(syncReturn);
      assert(syncReturnAsDartObject.length == 2);
      final rawReturn = syncReturnAsDartObject[0];
      final isSuccess = syncReturnAsDartObject[1];
      if (isSuccess) {
        return task.parseSuccessData(rawReturn);
      } else {
        throw FfiException('EXECUTE_SYNC', rawReturn as String, null);
      }
    } catch (err, st) {
      if (err is FfiException) rethrow;
      throw FfiException('EXECUTE_SYNC_ABORT', '$err', st);
    } finally {
      inner.free_WireSyncReturn(syncReturn);
    }
  }

  /// Similar to [executeNormal], except that this will return a [Stream] instead of a [Future].
  @protected
  Stream<S> executeStream<S>(FlutterRustBridgeTask<S> task) async* {
    final func = task.constMeta.debugName;
    final nextIndex = _streamSinkNameIndex.update(func, (value) => value + 1,
        ifAbsent: () => 0);
    final name = '__frb_streamsink_${func}_$nextIndex';
    final receivePort = broadcastPort(name);
    task.callFfi(receivePort.sendPort.nativePort);

    await for (final raw in receivePort) {
      try {
        yield _transformRust2DartMessage(raw, task.parseSuccessData);
      } on _CloseStreamException {
        receivePort.close();
        break;
      }
    }
  }

  S _transformRust2DartMessage<S>(
      List<dynamic> raw, S Function(dynamic) parseSuccessData) {
    final action = raw[0];
    switch (action) {
      case _RUST2DART_ACTION_SUCCESS:
        assert(raw.length == 2);
        return parseSuccessData(raw[1]);
      case _RUST2DART_ACTION_ERROR:
        assert(raw.length == 4);
        throw FfiException(raw[1], raw[2], raw[3]);
      case _RUST2DART_ACTION_CLOSE_STREAM:
        assert(raw.length == 1);
        throw _CloseStreamException();
      default:
        throw Exception('Unsupported message, action=$action raw=$raw');
    }
  }

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_SUCCESS = 0;

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_ERROR = 1;

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_CLOSE_STREAM = 2;
}

/// A task to call FFI function.
@immutable
class FlutterRustBridgeTask<S> extends FlutterRustBridgeBaseTask {
  /// The underlying function to call FFI function, usually the generated wire function
  final void Function(NativePortType port) callFfi;

  /// Parse the returned data from the underlying function
  final S Function(dynamic) parseSuccessData;

  const FlutterRustBridgeTask({
    required this.callFfi,
    required this.parseSuccessData,
    required FlutterRustBridgeTaskConstMeta constMeta,
    required List<dynamic> argValues,
    required dynamic hint,
  }) : super(
          constMeta: constMeta,
          argValues: argValues,
          hint: hint,
        );
}

/// A task to call FFI function, but it is synchronous.
@immutable
class FlutterRustBridgeSyncTask<S> extends FlutterRustBridgeBaseTask {
  /// The underlying function to call FFI function, usually the generated wire function
  final WireSyncReturn Function() callFfi;

  /// Parse the returned data from the underlying function
  final S Function(dynamic) parseSuccessData;

  const FlutterRustBridgeSyncTask({
    required this.callFfi,
    required this.parseSuccessData,
    required FlutterRustBridgeTaskConstMeta constMeta,
    required List<dynamic> argValues,
    required dynamic hint,
  }) : super(
          constMeta: constMeta,
          argValues: argValues,
          hint: hint,
        );
}

class _CloseStreamException {}
