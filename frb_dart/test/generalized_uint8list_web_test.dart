@TestOn('browser')
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_uint8list/adapted_uint8list.dart';
import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/third_party/flutter_foundation_serialization/write_buffer.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

void main() {
  test('AdaptedUint8List preserves the prefix when resized', () {
    final list = AdaptedUint8List(3, _binding());
    list[0] = 1;
    list[1] = 2;
    list[2] = 3;

    list.resize(2);

    expect(list.length, 2);
    expect(list.intoRaw().length, 2);

    list.resize(4);

    expect(list.length, 4);
    expect(list.intoRaw().length, 4);
    expect(
      wireSyncRust2DartSseAsUint8ListView(list.intoRaw().ptr),
      Uint8List.fromList([1, 2, 0, 0]),
    );
  });

  test('AdaptedUint8List forwards range writes and validates bounds', () {
    final list = AdaptedUint8List(3, _binding());

    list.setRange(1, 3, Uint8List.fromList([4, 5]));

    expect(
      wireSyncRust2DartSseAsUint8ListView(list.intoRaw().ptr),
      Uint8List.fromList([0, 4, 5]),
    );
    expect(() => list[3] = 6, throwsRangeError);
    expect(
      () => list.setRange(2, 4, Uint8List.fromList([6, 7])),
      throwsRangeError,
    );
  });

  test('WriteBuffer rejects invalid capacity and writes after intoRaw', () {
    expect(
      () => WriteBuffer(startCapacity: 0, binding: _binding()),
      throwsArgumentError,
    );

    final buffer = WriteBuffer(binding: _binding());
    buffer.putUint8(42);
    final raw = buffer.intoRaw();

    expect(raw.dataLen, 1);
    expect(() => buffer.putUint8(7), throwsA(isA<StateError>()));
    expect(buffer.intoRaw, throwsA(isA<StateError>()));
  });
}

GeneralizedFrbRustBinding _binding() => _Binding();

class _Binding extends Mock implements GeneralizedFrbRustBinding {}
