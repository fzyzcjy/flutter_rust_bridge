import 'dart:async';
import 'dart:ffi' as ffi;
import 'dart:ffi';

import 'package:dart_rust_bridge/src/utils.dart';
import 'package:meta/meta.dart';

final _instances = <Type>{};

abstract class DartRustBridgeBase<T extends DartRustBridgeWireBase> {
  DartRustBridgeBase(this.inner) {
    _sanityCheckSingleton();
    _setUpRustToDartComm();
  }

  @protected
  final T inner;

  void _sanityCheckSingleton() {
    if (_instances.contains(runtimeType)) {
      throw Exception(
          'Subclasses of `DartRustBridgeBase` should be singletons - there should not be two instances (runtimeType=$runtimeType)');
    }
    _instances.add(runtimeType);
  }

  void _setUpRustToDartComm() {
    inner.store_dart_post_cobject(NativeApi.postCObject.cast());
  }

  @protected
  Future<S> execute<S>(void Function(int port) callFfi, S Function(dynamic) parseSuccessData) {
    final completer = Completer<dynamic>();
    final sendPort = singleCompletePort(completer);

    callFfi(sendPort.nativePort);

    return completer.future.then((dynamic raw) {
      // print('execute see raw=$raw');
      final action = raw[0];
      switch (action) {
        case _RUST2DART_ACTION_SUCCESS:
          assert(raw.length == 2);
          return parseSuccessData(raw[1]);
        case _RUST2DART_ACTION_ERROR:
          assert(raw.length == 4);
          throw FfiException(raw[1], raw[2], raw[3]);
        default:
          throw Exception('Unsupported message, action=$action raw=$raw');
      }
    });
  }

  static const _RUST2DART_ACTION_SUCCESS = 0; // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_ERROR = 1; // ignore: constant_identifier_names
}

class WrappedRustVec {
  final ffi.Pointer<ffi.Uint8> ptr;
  final int len;

  WrappedRustVec({required this.ptr, required this.len});
}

@immutable
class FfiException {
  final String code;
  final String message;
  final Object? details;

  FfiException(this.code, this.message, this.details);

  @override
  String toString() => 'FfiException($code, $message, $details)';
}

abstract class DartRustBridgeWireBase {
  // ignore: non_constant_identifier_names
  void store_dart_post_cobject(
    ffi.Pointer<ffi.NativeFunction<ffi.Uint8 Function(ffi.Int64, ffi.Pointer<ffi.Void>)>> ptr,
  );
}
