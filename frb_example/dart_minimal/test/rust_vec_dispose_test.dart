@TestOn('vm')
library;

import 'dart:ffi';

import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_uint8list/rust_vec_u8.dart';
import 'package:test/test.dart';

void main() {
  test('disposing a Rust vector frees its original allocation length', () {
    final binding = _AllocationBinding();
    final buffer = RustVecU8(8, binding);

    buffer.dispose();

    expect(binding.freedLength, 8);
    expect(binding.freedPointer, binding.pointer);
  });
}

class _AllocationBinding implements GeneralizedFrbRustBinding {
  final pointer = Pointer<Uint8>.fromAddress(0x1000);
  int? freedLength;
  Pointer<Uint8>? freedPointer;

  @override
  Pointer<Uint8> rustVecU8New(int len) => pointer;

  @override
  void rustVecU8Free(Pointer<Uint8> ptr, int len) {
    freedLength = len;
    freedPointer = ptr;
  }

  @override
  dynamic noSuchMethod(Invocation invocation) =>
      throw UnsupportedError(invocation.memberName.toString());
}
