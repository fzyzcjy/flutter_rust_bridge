import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/_io.dart';
import 'package:flutter_rust_bridge/src/manual_impl/manual_impl.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class DcoCodec<S, E extends Object> extends BaseCodec<S, E, WireSyncReturnDco> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final S Function(dynamic) parseSuccessData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final E Function(dynamic)? parseErrorData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const DcoCodec({
    required this.parseSuccessData,
    required this.parseErrorData,
  });

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @override
  S decodeObject(dynamic raw) {
    final rawList = raw as List<dynamic>;
    return SimpleDecoder().decode(rawList[0]);
  }

  @override
  S decodeWireSyncType(WireSyncReturnDco raw) =>
      decodeObject(wireSyncReturnIntoDart(raw));

  @override
  void freeWireSyncReturn(WireSyncReturnDco raw,
          GeneralizedFrbRustBinding generalizedFrbRustBinding) =>
      generalizedFrbRustBinding.freeWireSyncReturnDco(raw);
}
