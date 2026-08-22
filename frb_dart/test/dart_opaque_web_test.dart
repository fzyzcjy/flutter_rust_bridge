@TestOn('browser')

import 'package:flutter_rust_bridge/src/dart_opaque/dart_opaque.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

void main() {
  test('encodeDartOpaque wraps functions for JavaScript interop', () {
    final binding = _Binding();
    final port = web.EventTarget();
    final function = () {};
    when(
      () => binding.dartOpaqueDart2RustEncode(any(), port),
    ).thenReturn(789);

    expect(encodeDartOpaque(function, port, binding), 789);
    final captured = verify(
      () => binding.dartOpaqueDart2RustEncode(captureAny(), port),
    ).captured.single;
    expect(captured, isA<AllowInteropFunctionWrapper>());
    expect((captured as AllowInteropFunctionWrapper).inner, same(function));
  });
}

class _Binding extends Mock implements GeneralizedFrbRustBinding {}
