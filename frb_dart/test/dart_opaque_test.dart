import 'package:flutter_rust_bridge/src/dart_opaque/dart_opaque.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

void main() {
  test('decodeDartOpaqueCommon converts BigInt pointers before decoding', () {
    final binding = _Binding();
    when(() => binding.dartOpaqueRust2DartDecode(123)).thenReturn('decoded');

    expect(decodeDartOpaqueCommon(BigInt.from(123), binding), 'decoded');
    verify(() => binding.dartOpaqueRust2DartDecode(123)).called(1);
  });

  test('decodeDartOpaque forwards native pointer values unchanged', () {
    final binding = _Binding();
    when(() => binding.dartOpaqueRust2DartDecode(456)).thenReturn('decoded');

    expect(decodeDartOpaque(456, binding), 'decoded');
    verify(() => binding.dartOpaqueRust2DartDecode(456)).called(1);
  });

}

class _Binding extends Mock implements GeneralizedFrbRustBinding {}
