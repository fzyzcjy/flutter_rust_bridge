@TestOn('vm')
import 'dart:ffi';

import 'package:flutter_rust_bridge/src/dart_opaque/dart_opaque.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

void main() {
  test('encodeDartOpaque forwards object and handler port', () {
    final binding = _Binding();
    final pointer = Pointer<Void>.fromAddress(789);
    final object = Object();
    when(
      () => binding.dartOpaqueDart2RustEncode(object, 11),
    ).thenReturn(pointer);

    expect(encodeDartOpaque(object, 11, binding), same(pointer));
    verify(() => binding.dartOpaqueDart2RustEncode(object, 11)).called(1);
  });
}

class _Binding extends Mock implements GeneralizedFrbRustBinding {}
