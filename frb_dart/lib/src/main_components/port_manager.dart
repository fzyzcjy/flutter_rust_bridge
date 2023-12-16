import 'package:flutter_rust_bridge/src/dart_opaque/dart_opaque.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/misc/dart_fn.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/utils/base_lazy_port_manager.dart';

/// {@macro flutter_rust_bridge.internal}
class DartHandlerPortManager extends BaseLazyPortManager {
  final GeneralizedFrbRustBinding _generalizedFrbRustBinding;
  final BaseHandler _handler;

  /// {@macro flutter_rust_bridge.internal}
  DartHandlerPortManager(this._generalizedFrbRustBinding, this._handler);

  @override
  void onData(Object? message) {
    TODO;

    // TODO drop port
    // _generalizedFrbRustBinding.dartOpaqueDropThreadBoxPersistentHandle(message)

    // TODO invoke port
  }
}
