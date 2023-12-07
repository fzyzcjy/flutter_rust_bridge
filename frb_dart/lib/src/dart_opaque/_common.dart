import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/utils/base_lazy_port_manager.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
Object dcoDecodeDartOpaque(
    dynamic raw, GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  return generalizedFrbRustBinding.dartOpaqueRust2DartDecode(raw);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class DartOpaqueDropPortManager extends BaseLazyPortManager {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final GeneralizedFrbRustBinding _generalizedFrbRustBinding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  DartOpaqueDropPortManager(this._generalizedFrbRustBinding);

  @override
  void onData(dynamic message) => _generalizedFrbRustBinding
      .dartOpaqueDropThreadBoxPersistentHandle(message);
}
