import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
Object decodeDartOpaque(int raw,
    GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  return generalizedFrbRustBinding.dartOpaqueRust2DartDecode(raw);
}
