import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:meta/meta.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseCodec<S, E extends Object, WireSyncType> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const BaseCodec();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  S decodeObject(dynamic raw);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  S decodeWireSyncType(WireSyncType raw);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncRust2Dart(
      WireSyncType raw, GeneralizedFrbRustBinding generalizedFrbRustBinding);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class CloseStreamException implements Exception {}

/// {@macro flutter_rust_bridge.internal}
abstract class SimpleDecoder<S, E extends Object> {
  /// {@macro flutter_rust_bridge.internal}
  S decode(int action) {
    switch (action) {
      case _Rust2DartAction.success:
        return decodeSuccess();

      case _Rust2DartAction.error:
        throw decodeError();

      case _Rust2DartAction.panic:
        throw decodePanic();

      case _Rust2DartAction.closeStream:
        throw CloseStreamException();

      // coverage:ignore-start
      default:
        throw Exception('Unsupported message (action=$action)');
      // coverage:ignore-end
    }
  }

  /// {@macro flutter_rust_bridge.internal}
  @protected
  S decodeSuccess();

  /// {@macro flutter_rust_bridge.internal}
  @protected
  E decodeError();

  /// {@macro flutter_rust_bridge.internal}
  @protected
  Object decodePanic();
}

/// NOTE: Please keep in sync with the Rust side
class _Rust2DartAction {
  // Do not use enum, but use raw integers, to avoid extra overhead
  static const success = 0;
  static const error = 1;
  static const closeStream = 2;
  static const panic = 3;
}
