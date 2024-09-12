import 'package:flutter_rust_bridge/src/dart_opaque/_common.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
PlatformPointer encodeDartOpaque(Object raw, NativePortType dartHandlerPort,
        GeneralizedFrbRustBinding generalizedFrbRustBinding) =>
    encodeDartOpaqueCommon(
        _prepareDartOpaque(raw), dartHandlerPort, generalizedFrbRustBinding);

/// {@macro flutter_rust_bridge.only_for_generated_code}
Object decodeDartOpaque(
        dynamic raw, GeneralizedFrbRustBinding generalizedFrbRustBinding) =>
    _unprepareDartOpaque(
        decodeDartOpaqueCommon(raw, generalizedFrbRustBinding));

Object _prepareDartOpaque(Object raw) {
  // #2183
  if (raw is Function) {
    return AllowInteropFunctionWrapper(raw);
  }
  return raw;
}

Object _unprepareDartOpaque(Object raw) {
  if (raw is AllowInteropFunctionWrapper) {
    return raw.inner;
  }
  return raw;
}

/// {@macro flutter_rust_bridge.internal}
class AllowInteropFunctionWrapper {
  /// {@macro flutter_rust_bridge.internal}
  final Function inner;

  /// {@macro flutter_rust_bridge.internal}
  const AllowInteropFunctionWrapper(this.inner);
}
