import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseCodec<S, E extends Object, WireSyncType> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const BaseCodec();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  S decodeObject(dynamic raw);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  S decodeWireSyncType(WireSyncType raw);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncReturn(
      WireSyncType raw, GeneralizedFrbRustBinding generalizedFrbRustBinding);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class CloseStreamException implements Exception {}

/// {@macro flutter_rust_bridge.internal}
class SimpleDecoder<S> {
  /// {@macro flutter_rust_bridge.internal}
  S decode(int action) {
    switch (action) {
      case _Rust2DartAction.success:
        assert(rawList.length == 2);
        return decodeSuccessData(rawList[1]);

      case _Rust2DartAction.error:
        assert(rawList.length == 2);
        final parseErrorData = this.decodeErrorData;
        if (parseErrorData == null) {
          throw Exception(
              'transformRust2DartMessage received error message, but no parseErrorData to parse it. '
              'Raw data: $raw');
        }
        throw parseErrorData(rawList[1]);

      case _Rust2DartAction.panic:
        assert(rawList.length == 2);
        throw dcoDecodePanicError(rawList[1]);

      case _Rust2DartAction.closeStream:
        assert(rawList.length == 1);
        throw CloseStreamException();

      default:
        throw Exception('Unsupported message (action=$action)');
    }
  }
}

/// NOTE: Please keep in sync with the Rust side
class _Rust2DartAction {
  // Do not use enum, but use raw integers, to avoid extra overhead
  static const success = 0;
  static const error = 1;
  static const closeStream = 2;
  static const panic = 3;
}
