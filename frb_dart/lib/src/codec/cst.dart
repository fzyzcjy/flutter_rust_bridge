import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';

// This is nothing but "unreachable", so by definition it will not be covered
// coverage:ignore-start
/// {@macro flutter_rust_bridge.only_for_generated_code}
class CstCodec<S, E extends Object> extends BaseCodec<S, E, dynamic> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const CstCodec();

  @override
  void freeWireSyncRust2Dart(
          dynamic raw, GeneralizedFrbRustBinding generalizedFrbRustBinding) =>
      throw UnimplementedError('unreachable');

  @override
  S decodeObject(dynamic raw) => throw UnimplementedError('unreachable');

  @override
  S decodeWireSyncType(dynamic raw) => throw UnimplementedError('unreachable');
}
// coverage:ignore-end
