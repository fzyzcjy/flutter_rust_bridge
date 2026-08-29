import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/main_components/port_manager.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

class _MockBinding extends Mock implements GeneralizedFrbRustBinding {}

class _MockHandler extends Mock implements BaseHandler {}

void main() {
  test('DartHandlerPortManager drops opaque handles through the binding', () {
    final binding = _MockBinding();
    final manager = DartHandlerPortManager(binding, _MockHandler());

    try {
      manager.onData([0, 123]);

      verify(
        () => binding.dartOpaqueDropThreadBoxPersistentHandle(123),
      ).called(1);
    } finally {
      manager.dispose();
    }
  });

  test(
    'DartHandlerPortManager delegates callback messages without the action',
    () {
      final binding = _MockBinding();
      final handler = _MockHandler();
      final manager = DartHandlerPortManager(binding, handler);

      try {
        manager.onData([1, 456, 'argument']);

        verify(
          () => handler.dartFnInvoke([456, 'argument'], binding),
        ).called(1);
      } finally {
        manager.dispose();
      }
    },
  );
}
