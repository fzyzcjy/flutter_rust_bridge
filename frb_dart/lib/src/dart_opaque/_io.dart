import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
PlatformPointer api2wireDartOpaque(
    Object raw, GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  final ptr = generalizedFrbRustBinding.newDartOpaque(raw);
  // TODO let the binding directly give a pointer instead
  return ffi.Pointer.fromAddress(ptr);
}
