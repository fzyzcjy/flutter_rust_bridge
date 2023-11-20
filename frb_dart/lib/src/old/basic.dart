import 'dart:async';

import 'package:flutter_rust_bridge/src/old/platform_independent.dart';
import 'package:flutter_rust_bridge/src/old/utils.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:flutter_rust_bridge/src/wire_base/wire_base.dart';
import 'package:meta/meta.dart';

import 'ffi.dart';
import 'isolate.dart';

export 'ffi.dart';
export 'isolate.dart';

final _instances = <Type>{};

/// Base class for generated bindings of Flutter Rust Bridge.
/// Normally, users do not extend this class manually. Instead,
/// users should directly use the generated class.
abstract class FlutterRustBridgeBase<T extends FlutterRustBridgeWireBase> {
  FlutterRustBridgeBase(this.inner) {
    // _sanityCheckSingleton();
    _setUpRustToDartComm();
  }

  @protected
  final T inner;

  NativePortType get dropPort => _dropPort.sendPort.nativePort;
  late final _dropPort = _initDropPort();

  ReceivePort _initDropPort() {
    var port = broadcastPort(DropIdPortGenerator.create());
    port.listen((message) {
      inner.drop_dart_object(message);
    });
    return port;
  }

  void dispose() {
    _dropPort.close();
  }

  // void _sanityCheckSingleton() {
  //   if (_instances.contains(runtimeType)) {
  //     throw Exception(
  //       'Subclasses of `FlutterRustBridgeBase` should be singletons - '
  //       'there should not be two instances (runtimeType=$runtimeType)',
  //     );
  //   }
  //   _instances.add(runtimeType);
  // }

  void _setUpRustToDartComm() {
    inner.storeDartPostCObject();
  }

// DONE
// @protected
// Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
//   // MOVED
// }
//
// @protected
// S executeSync<S, E extends Object>(SyncTask<S, E> task) {
//   // MOVEd
// }
//
// @protected
// Stream<S> executeStream<S, E extends Object>(NormalTask<S, E> task) async* {
//   // MOVED
// }
}
