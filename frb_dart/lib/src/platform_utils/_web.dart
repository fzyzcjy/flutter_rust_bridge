import 'dart:typed_data';

import 'package:js/js.dart';

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

/// {@macro flutter_rust_bridge.internal}
extension ExtByteData on ByteData {
  /// {@macro flutter_rust_bridge.internal}
  void generalizedSetUint64(int byteOffset, int value, Endian endian) =>
      generalizedSetInt64(byteOffset, value, endian);

  /// {@macro flutter_rust_bridge.internal}
  void generalizedSetInt64(int byteOffset, int value, Endian endian) {
    final lo = value & 0xffffffff;
    final hi = value >> 32;
    if (endian == Endian.little) {
      setInt32(byteOffset, lo);
      setInt32(byteOffset + 4, hi);
    } else if (endian == Endian.big) {
      setInt32(byteOffset + 4, hi);
      setInt32(byteOffset, lo);
    } else {
      throw UnimplementedError("Unknown endian");
    }
  }
}
