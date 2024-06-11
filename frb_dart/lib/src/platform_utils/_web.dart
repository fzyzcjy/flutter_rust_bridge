import 'dart:js_interop';

/// {@macro flutter_rust_bridge.only_for_generated_code}
// @JS('Number')
int castInt(Object? value) {
  throw UnimplementedError('castInt($value) runtimeType=${value.runtimeType}');
}

// @JS('Function')
// class _Function {
//   external dynamic call();
//
//   external factory _Function(String script);
// }
//
// /// {@macro flutter_rust_bridge.internal}
// dynamic jsEval(String script) => _Function(script)();

dynamic jsEval(String script) {
  throw UnimplementedError('jsEval(script=$script)');
}

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
BigInt jsBigIntToDartBigInt(Object bigInt) {
  throw UnimplementedError('todo!');
  // return BigInt.parse(callMethod(bigInt, 'toString', const []));
}
