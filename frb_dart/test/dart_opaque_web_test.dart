@TestOn('browser')
import 'package:flutter_rust_bridge/src/dart_opaque/dart_opaque.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

void main() {
  test('encodeDartOpaque wraps functions for JavaScript interop', () {
    final binding = _Binding();
    final port = web.EventTarget();
    void function() {}

    expect((encodeDartOpaque as dynamic)(function, port, binding), 789);
    final captured = binding.encodedObject;
    expect(captured.runtimeType.toString(), 'AllowInteropFunctionWrapper');
    expect((captured as dynamic).inner, same(function));
  });
}

class _Binding implements GeneralizedFrbRustBinding {
  Object? encodedObject;

  @override
  dynamic noSuchMethod(Invocation invocation) {
    if (invocation.memberName == #dartOpaqueDart2RustEncode) {
      encodedObject = invocation.positionalArguments.first;
      return 789;
    }
    return super.noSuchMethod(invocation);
  }
}
