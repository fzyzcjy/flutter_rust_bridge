import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/utils/base_lazy_port_manager.dart';

// Why `PortManager` and `DartHandlerPortManager` are two different classes?
// Because in case we have multiple ports in the future.
/// {@macro flutter_rust_bridge.internal}
class PortManager {
  final DartHandlerPortManager _dartHandlerPortManager;

  /// {@macro flutter_rust_bridge.internal}
  NativePortType get dartHandlerPort => _dartHandlerPortManager.port;

  /// {@macro flutter_rust_bridge.internal}
  PortManager(
    GeneralizedFrbRustBinding generalizedFrbRustBinding,
    BaseHandler handler,
  ) : _dartHandlerPortManager =
            DartHandlerPortManager(generalizedFrbRustBinding, handler);

  /// {@macro flutter_rust_bridge.internal}
  void dispose() {
    _dartHandlerPortManager.dispose();
  }
}

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
      // coverage:ignore-start
      default:
        throw UnimplementedError('Unsupported message: $message');
      // coverage:ignore-end
    }
  }
}

/// NOTE: Please keep in sync with the Rust side
class _DartHandlerPortAction {
  // Do not use enum, but use raw integers, to avoid extra overhead
  static const dartOpaqueDrop = 0;
  static const dartFnInvoke = 1;
}
