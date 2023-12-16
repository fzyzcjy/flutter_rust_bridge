import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/api.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/main_components/port_manager.dart';
import 'package:flutter_rust_bridge/src/main_components/wire.dart';
import 'package:meta/meta.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseApiImpl<W extends BaseWire> implements BaseApi {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  final BaseHandler handler;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  final W wire;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final GeneralizedFrbRustBinding generalizedFrbRustBinding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final PortManager portManager;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  BaseApiImpl({
    required this.handler,
    required this.wire,
    required this.generalizedFrbRustBinding,
    required this.portManager,
  });
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef ApiImplConstructor<A extends BaseApiImpl, W extends BaseWire> = A
    Function({
  required BaseHandler handler,
  required W wire,
  required GeneralizedFrbRustBinding generalizedFrbRustBinding,
  required PortManager portManager,
});
