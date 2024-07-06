import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
Object decodeDartOpaque(
    dynamic raw, GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  if (raw is BigInt) {
    raw = raw.toInt();
  }
  return generalizedFrbRustBinding.dartOpaqueRust2DartDecode(raw);
}

/// {@macro flutter_rust_bridge.internal}
PlatformPointer encodeDartOpaqueCommon(
    Object raw,
    NativePortType dartHandlerPort,
    GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  return generalizedFrbRustBinding.dartOpaqueDart2RustEncode(
      raw, dartHandlerPort);
}
