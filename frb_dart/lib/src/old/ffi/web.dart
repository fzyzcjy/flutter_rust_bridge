import 'dart:async';
import 'dart:html';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge/src/utils/web_utils.dart';
import 'package:js/js.dart';

export 'package:js/js.dart';
export 'package:js/js_util.dart' show promiseToFuture, getProperty;

typedef ExternalLibrary = FutureOr<WasmModule>;
typedef DartPostCObject = void;

@JS()
external bool? get crossOriginIsolated;

@JS('Number')
external int castInt(Object? value);

@JS('BigInt')
external Object castNativeBigInt(Object? value);

@JS('Function')
class _Function {
  external dynamic call();

  external factory _Function(String script);
}

dynamic eval(String script) => _Function(script)();

abstract class DartApiDl {}
