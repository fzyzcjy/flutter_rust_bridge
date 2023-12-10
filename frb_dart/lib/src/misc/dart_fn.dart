import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/utils/base_lazy_port_manager.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class DartFnInvokePortManager extends BaseLazyPortManager {
  final BaseHandler _handler;
  final GeneralizedFrbRustBinding _generalizedFrbRustBinding;

  /// {@macro flutter_rust_bridge.internal}
  DartFnInvokePortManager(this._handler, this._generalizedFrbRustBinding);

  @override
  void onData(dynamic message) =>
      _handler.dartFnInvoke(message, _generalizedFrbRustBinding);
}
