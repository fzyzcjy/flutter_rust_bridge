import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/utils/base_lazy_port_manager.dart';

/// {@macro flutter_rust_bridge.internal}
class DartHandlerPortManager extends BaseLazyPortManager {
  final GeneralizedFrbRustBinding _generalizedFrbRustBinding;
  final BaseHandler _handler;

  /// {@macro flutter_rust_bridge.internal}
  DartHandlerPortManager(this._generalizedFrbRustBinding, this._handler);

  @override
  void onData(covariant List<dynamic> message) {
    switch (message[0]) {
      case _DartHandlerPortAction.dartOpaqueDrop:
        assert(message.length == 2);
        _generalizedFrbRustBinding
            .dartOpaqueDropThreadBoxPersistentHandle(message[1]);
      case _DartHandlerPortAction.dartFnInvoke:
        _handler.dartFnInvoke(message.sublist(1), _generalizedFrbRustBinding);
      default:
        throw UnimplementedError('Unsupported message: $message');
    }
  }
}

/// NOTE: Please keep in sync with the Rust side
class _DartHandlerPortAction {
  // Do not use enum, but use raw integers, to avoid extra overhead
  static const dartOpaqueDrop = 0;
  static const dartFnInvoke = 1;
}
