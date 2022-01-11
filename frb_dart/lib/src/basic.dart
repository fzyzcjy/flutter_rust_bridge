import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'dart:isolate';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/platform_independent.dart';
import 'package:flutter_rust_bridge/src/utils.dart';
import 'package:meta/meta.dart';

final _instances = <Type>{};

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

  void _sanityCheckSingleton() {
    if (_instances.contains(runtimeType)) {
      throw Exception(
          'Subclasses of `FlutterRustBridgeBase` should be singletons - there should not be two instances (runtimeType=$runtimeType)');
    }
    _instances.add(runtimeType);
  }

  void _setUpRustToDartComm() {
    inner.store_dart_post_cobject(NativeApi.postCObject.cast());
  }

  /// Execute a normal ffi call. Usually called by generated code instead of manually called.
  @protected
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> task) {
    final completer = Completer<dynamic>();
    final sendPort = singleCompletePort(completer);
    task.callFfi(sendPort.nativePort);
    return completer.future.then((dynamic raw) => _transformRust2DartMessage(raw, task.parseSuccessData));
  }

  /// Similar to [executeNormal], except that this will return synchronously
  @protected
  Uint8List executeSync(FlutterRustBridgeSyncTask task) {
    final raw = task.callFfi();

    final bytes = Uint8List.fromList(raw.ptr.asTypedList(raw.len));
    final success = raw.success > 0;

    inner.free_WireSyncReturnStruct(raw);

    if (success) {
      return bytes;
    } else {
      throw FfiException('EXECUTE_SYNC', utf8.decode(bytes), null);
    }
  }

  /// Similar to [executeNormal], except that this will return a [Stream] instead of a [Future].
  @protected
  Stream<S> executeStream<S>(FlutterRustBridgeTask<S> task) async* {
    final receivePort = ReceivePort();
    task.callFfi(receivePort.sendPort.nativePort);

    await for (final raw in receivePort) {
      try {
        yield _transformRust2DartMessage(raw, task.parseSuccessData);
      } on _CloseStreamException {
        receivePort.close();
      }
    }
  }

  S _transformRust2DartMessage<S>(dynamic raw, S Function(dynamic) parseSuccessData) {
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

  static const _RUST2DART_ACTION_SUCCESS = 0; // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_ERROR = 1; // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_CLOSE_STREAM = 2; // ignore: constant_identifier_names
}

/// A task to call FFI function.
@immutable
class FlutterRustBridgeTask<S> extends FlutterRustBridgeBaseTask {
  final void Function(int port) callFfi;
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
class FlutterRustBridgeSyncTask extends FlutterRustBridgeBaseTask {
  final WireSyncReturnStruct Function() callFfi;

  const FlutterRustBridgeSyncTask({
    required this.callFfi,
    required FlutterRustBridgeTaskConstMeta constMeta,
    required List<dynamic> argValues,
    required dynamic hint,
  }) : super(
          constMeta: constMeta,
          argValues: argValues,
          hint: hint,
        );
}

/// This class, together with its subclasses, are only for internal usage.
/// Usually it should not be used by normal users.
abstract class FlutterRustBridgeWireBase {
  // ignore: non_constant_identifier_names
  void store_dart_post_cobject(
    ffi.Pointer<ffi.NativeFunction<ffi.Uint8 Function(ffi.Int64, ffi.Pointer<ffi.Void>)>> ptr,
  );

  // ignore: non_constant_identifier_names
  void free_WireSyncReturnStruct(WireSyncReturnStruct val);
}

class _CloseStreamException {}

// NOTE for maintainer: Please manually keep in sync with [WireSyncReturnStruct] in Rust
/// This class is only for internal usage.
class WireSyncReturnStruct extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;

  @ffi.Uint8()
  external int success;
}
