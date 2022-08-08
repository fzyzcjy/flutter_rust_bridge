import 'dart:async';

import 'package:js/js.dart';
export 'package:js/js.dart';
import 'package:js/js_util.dart' show promiseToFuture;
export 'package:js/js_util.dart' show promiseToFuture;

import 'dart:typed_data';
import '../ffi.dart' show WasmModule;

typedef ExternalLibrary = FutureOr<WasmModule>;

@JS()
external bool? get crossOriginIsolated;

@JS('Function')
class _Function {
  external dynamic call();
  external factory _Function(String script);
}

dynamic eval(String script) => _Function(script)();

abstract class FlutterRustBridgeWireBase {
  void storeDartPostCObject() {}
  // ignore: non_constant_identifier_names
  void free_WireSyncReturnStruct(WireSyncReturnStruct raw) {}
}

@JS()
@anonymous
class WireSyncReturnStruct {
  external final Uint8List buffer;
  external final int success;
}

class FlutterRustBridgeWasmWireBase<T extends WasmModule> extends FlutterRustBridgeWireBase {
  late final Future<T> init;
  late final T inner;

  FlutterRustBridgeWasmWireBase(FutureOr<T> module) {
    init = Future.value(module).then((module) async {
      eval('window.wasm_bindgen = wasm_bindgen');
      inner = await promiseToFuture<T>(module());
      return inner;
    });
  }
}

extension WireSyncReturnStructExt on WireSyncReturnStruct {
  bool get isSuccess => success > 0;
}
