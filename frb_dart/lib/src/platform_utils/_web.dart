import 'dart:js_interop';

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS('Number')
external int castInt(Object? value);

/// {@macro flutter_rust_bridge.internal}
@JS('console.warn')
external void jsConsoleWarn([a, b, c, d, e, f, g, h, i]);

@JS('Function')
class _Function {
  external dynamic call();

  external factory _Function(String script);
}

/// {@macro flutter_rust_bridge.internal}
dynamic jsEval(String script) => _Function(script)();

/// Whether the web platform has been isolated by COOP and COEP headers,
/// and is capable of sharing buffers between workers.
///
/// Note: not available on all browsers, in which case it will return null.
@JS()
external bool? get crossOriginIsolated;

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS('BigInt')
external Object castNativeBigInt(Object? value);

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt jsBigIntToDartBigInt(Object bigInt) {
  throw UnimplementedError('todo!');
  // return BigInt.parse(callMethod(bigInt, 'toString', const []));
}
