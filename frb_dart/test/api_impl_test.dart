import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/api_impl.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/main_components/port_manager.dart';
import 'package:flutter_rust_bridge/src/main_components/wire.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

class _MockBinding extends Mock implements GeneralizedFrbRustBinding {}

class _TestWire extends BaseWire {}

class _TestApiImpl extends BaseApiImpl<_TestWire> {
  _TestApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  BaseHandler get exposedHandler => handler;
  _TestWire get exposedWire => wire;
}

void main() {
  test('BaseApiImpl retains every generated dependency by identity', () {
    final binding = _MockBinding();
    final handler = BaseHandler();
    final portManager = PortManager(binding, handler);
    final wire = _TestWire();
    final apiImpl = _TestApiImpl(
      handler: handler,
      wire: wire,
      generalizedFrbRustBinding: binding,
      portManager: portManager,
    );

    try {
      expect(apiImpl.exposedHandler, same(handler));
      expect(apiImpl.exposedWire, same(wire));
      expect(apiImpl.generalizedFrbRustBinding, same(binding));
      expect(apiImpl.portManager, same(portManager));
    } finally {
      portManager.dispose();
    }
  });
}
