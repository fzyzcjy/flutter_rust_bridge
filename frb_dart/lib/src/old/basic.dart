import 'dart:async';

import 'package:flutter_rust_bridge/src/old/platform_independent.dart';
import 'package:flutter_rust_bridge/src/old/utils.dart';
import 'package:meta/meta.dart';

import 'ffi.dart';
import 'isolate.dart';

export 'ffi.dart';
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

  @protected
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    // MOVED
  }

  @protected
  S executeSync<S, E extends Object>(SyncTask<S, E> task) {
    // MOVEd
  }

  @protected
  Stream<S> executeStream<S, E extends Object>(NormalTask<S, E> task) async* {
    // MOVED
  }

  S _transformRust2DartMessage<S, E extends Object>(List<dynamic> raw, S Function(dynamic) parseSuccessData,
      E Function(dynamic)? parseErrorData, PanicException Function(dynamic)? parsePanicData) {
    final action = raw[0];
    switch (action) {
      case _RUST2DART_ACTION_SUCCESS:
        return _parseData<S>(raw, parseSuccessData);
      case _RUST2DART_ACTION_ERROR:
        throw _parseData<E>(raw, parseErrorData);
      case _RUST2DART_ACTION_PANIC:
        throw _parseData<PanicException>(raw, parsePanicData);
      case _RUST2DART_ACTION_CLOSE_STREAM:
        assert(raw.length == 1);
        throw _CloseStreamException();
      default:
        throw Exception('Unsupported message, action=$action raw=$raw');
    }
  }

  R _parseData<R>(List<dynamic> rawData, R Function(dynamic)? function) {
    assert(rawData.length == 2);
    if (function != null) {
      return function(rawData[1]);
    }

    throw Exception("tried to parse data but function is null");
  }

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_SUCCESS = 0;

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_ERROR = 1;

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_CLOSE_STREAM = 2;

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_PANIC = 3;
}

class _CloseStreamException {}

class FrbException implements Exception {}

class PanicException extends FrbException {
  final String error;

  PanicException(this.error);
}

PanicException wire2apiPanicError(dynamic raw) {
  return PanicException(raw as String);
}

class FrbAnyhowException implements FrbException {
  final String anyhow;

  FrbAnyhowException(this.anyhow);

  @override
  String toString() => 'FrbAnyhowException($anyhow)';
}

abstract class FrbBacktracedException extends FrbException {
  String get backtrace;
}
