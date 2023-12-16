import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/utils/base_lazy_port_manager.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
Object decodeDartOpaque(
    dynamic raw, GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  return generalizedFrbRustBinding.dartOpaqueRust2DartDecode(raw);
}
