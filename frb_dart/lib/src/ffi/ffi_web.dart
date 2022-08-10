import 'dart:async';

import 'package:js/js.dart';
export 'package:js/js.dart';
import 'package:js/js_util.dart';
export 'package:js/js_util.dart' show promiseToFuture, getProperty;

import 'dart:typed_data';
import '../ffi.dart' show WasmModule;

typedef ExternalLibrary = FutureOr<WasmModule>;

@JS('console.log')
external void nativeLog([Object? _]);

@JS()
external bool? get crossOriginIsolated;

@JS('Number')
external int castInt(Object? value);

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

typedef WireSyncReturnStruct = List<dynamic>;

extension WireSyncReturnStructExt on WireSyncReturnStruct {
  Uint8List get buffer => Uint8List.view(getProperty(this[0], 'buffer'));
  bool get isSuccess => this[1];
}

class FlutterRustBridgeWasmWireBase<T extends WasmModule>
    extends FlutterRustBridgeWireBase {
  final Future<T> init;

  FlutterRustBridgeWasmWireBase(FutureOr<T> module)
      : init = Future.value(module).then((module) => promiseToFuture(module()));
}
