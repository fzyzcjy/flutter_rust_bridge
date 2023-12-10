import 'package:flutter_rust_bridge/src/dart_opaque/dart_opaque.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/misc/dart_fn.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class PortManager {
  final DartOpaqueDropPortManager _dartOpaqueDropPortManager;
  final DartFnInvokePortManager _dartFnInvokePortManager;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  PortManager(
      GeneralizedFrbRustBinding generalizedFrbRustBinding, BaseHandler handler)
      : _dartOpaqueDropPortManager =
            DartOpaqueDropPortManager(generalizedFrbRustBinding),
        _dartFnInvokePortManager =
            DartFnInvokePortManager(handler, generalizedFrbRustBinding);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  NativePortType get dartOpaqueDropPort => _dartOpaqueDropPortManager.port;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  NativePortType get dartFnInvokePort => _dartFnInvokePortManager.port;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dispose() {
    _dartOpaqueDropPortManager.dispose();
    _dartFnInvokePortManager.dispose();
  }
}
