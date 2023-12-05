import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/misc/dart_opaque.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class PortManager {
  final DropPortManager _dropPortManager;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  PortManager(GeneralizedFrbRustBinding generalizedFrbRustBinding)
      : _dropPortManager = DropPortManager(generalizedFrbRustBinding);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  NativePortType get dropPort => _dropPortManager.port;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dispose() {
    _dropPortManager.dispose();
  }
}
