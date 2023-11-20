import 'dart:async';
import 'dart:js_util';

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';
import 'package:js/js.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  void free_WireSyncReturn(WireSyncReturn raw) {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  Object get_dart_object(int ptr) {
    return getDartObject(ptr);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  void drop_dart_object(int ptr) {
    dropDartObject(ptr);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  int new_dart_opaque(Object obj, NativePortType port) {
    throw UnimplementedError();
  }
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class BaseWasmWire<T extends WasmModule> extends BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final Future<T> init;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  BaseWasmWire(FutureOr<T> module) : init = Future.value(module).then((module) => promiseToFuture(module()));
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.get_dart_object")
// ignore: non_constant_identifier_names
external Object getDartObject(int ptr);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.drop_dart_object")
// ignore: non_constant_identifier_names
external void dropDartObject(int ptr);
