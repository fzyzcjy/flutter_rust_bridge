import 'dart:async';
import 'dart:js_util';

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:js/js.dart';

abstract class FlutterRustBridgeWireBase {
  void storeDartPostCObject() {}

  // ignore: non_constant_identifier_names
  void free_WireSyncReturn(WireSyncReturn raw) {}

  // ignore: non_constant_identifier_names
  Object get_dart_object(int ptr) {
    return getDartObject(ptr);
  }

  // ignore: non_constant_identifier_names
  void drop_dart_object(int ptr) {
    dropDartObject(ptr);
  }

  // ignore: non_constant_identifier_names
  int new_dart_opaque(Object obj, NativePortType port) {
    throw UnimplementedError();
  }
}

class FlutterRustBridgeWasmWireBase<T extends WasmModule> extends FlutterRustBridgeWireBase {
  final Future<T> init;

  FlutterRustBridgeWasmWireBase(FutureOr<T> module)
      : init = Future.value(module).then((module) => promiseToFuture(module()));
}

@JS("wasm_bindgen.get_dart_object")
// ignore: non_constant_identifier_names
external Object getDartObject(int ptr);

@JS("wasm_bindgen.drop_dart_object")
// ignore: non_constant_identifier_names
external void dropDartObject(int ptr);
