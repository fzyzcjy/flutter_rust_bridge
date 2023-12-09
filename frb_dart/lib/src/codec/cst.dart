import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class CstCodec<S, E extends Object> extends BaseCodec<S, E, dynamic> {
  @override
  void freeWireSyncReturn(
          dynamic raw, GeneralizedFrbRustBinding generalizedFrbRustBinding) =>
      throw UnimplementedError('unreachable');

  @override
  S decodeObject(dynamic raw) => throw UnimplementedError('unreachable');

  @override
  S decodeWireSyncType(dynamic raw) => throw UnimplementedError('unreachable');
}
