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
class _UnaryFunction {
  external dynamic call();
  external factory _UnaryFunction(String script);
}

dynamic eval(String script) => _UnaryFunction(script)();

abstract class FlutterRustBridgeWireBase {
  void storeDartPostCObject() {}
  void free_WireSyncReturnStruct(WireSyncReturnStruct raw) {}
}

@JS()
@anonymous
class WireSyncReturnStruct {
  external final Uint8List buffer;
  external final int success;
}

class FlutterRustBridgeWasmWireBase extends FlutterRustBridgeWireBase {
  final Future<void> init;
  FlutterRustBridgeWasmWireBase(FutureOr<WasmModule> module)
      : init = Future.value(module)
            .then((module) => promiseToFuture(module()))
            .then((_) => eval("window.wasm_bindgen = wasm_bindgen"));
}

extension WireSyncReturnStructExt on WireSyncReturnStruct {
  bool get isSuccess => success > 0;
}
