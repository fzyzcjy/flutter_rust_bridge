import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseCodec<S, E extends Object, WireSyncType> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const BaseCodec();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // TODO wrong type
  S decode(WireSyncType raw);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncReturn(
      WireSyncType raw, GeneralizedFrbRustBinding generalizedFrbRustBinding);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class CloseStreamException implements Exception {}
