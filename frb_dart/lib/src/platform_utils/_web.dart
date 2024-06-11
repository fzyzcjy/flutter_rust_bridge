import 'dart:js_interop';
import 'dart:js_interop_unsafe';

/// {@macro flutter_rust_bridge.only_for_generated_code}
// @JS('Number')
int castInt(Object? value) {
  throw UnimplementedError('castInt($value) runtimeType=${value.runtimeType}');
}

@JS('Function')
extension type _Function._(JSObject _) implements JSObject {
  external factory _Function(String script);

  external JSAny? call();
}

/// {@macro flutter_rust_bridge.internal}
JSAny? jsEval(String script) => _Function(script)();

/// Whether the web platform has been isolated by COOP and COEP headers,
/// and is capable of sharing buffers between workers.
///
/// Note: not available on all browsers, in which case it will return null.
@JS()
external bool? get crossOriginIsolated;

/// {@macro flutter_rust_bridge.only_for_generated_code}
// @JS('BigInt')
Object castNativeBigInt(Object? value) {
  throw UnimplementedError(
      'castNativeBigInt($value) runtimeType=${value.runtimeType}');
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt jsBigIntToDartBigInt(Object raw) {
  final jsAny = raw.jsify();
  print(
      'hi ${jsAny.isA<JSObject>()} ${jsAny.isA<JSAny>()} ${jsAny.isA<JSBigInt>()}');
  if (jsAny.isA<JSObject>()) {
    final jsObject = jsAny as JSObject;
    return BigInt.parse(jsObject.callMethod('toString'.toJS));
  }
  throw Exception(
      'jsBigIntToDartBigInt see unexpected type=${raw.runtimeType} value=$raw');
}
