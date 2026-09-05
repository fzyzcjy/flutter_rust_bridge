@TestOn('vm')
library;

import 'dart:ffi';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_uint8list/rust_vec_u8.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

void main() {
  test('disposing a Rust vector frees its original allocation length', () {
    final binding = _AllocationBinding();
    final buffer = RustVecU8(8, binding);

    buffer.dispose();

    expect(binding.freedLength, 8);
    expect(binding.freedPointer, binding.pointer);
  });

  test('disposed opaque encoding releases the serializer allocation', () async {
    await RustLib.init();
    final probe = await DisposalProbe.newInstance();
    probe.dispose();
    final binding = _AllocationBinding();
    final api = _RecordingApi(RustLib.instance.api as RustLibApiImpl, binding);

    expect(
      () => api.crateApiMinimalDisposalProbeRead(that: probe),
      throwsA(isA<DroppableDisposedException>()),
    );
    expect(binding.freedLength, 8);
  });
}

class _RecordingApi extends RustLibApiImpl {
  _RecordingApi(RustLibApiImpl original, GeneralizedFrbRustBinding binding)
      : super(
          handler: BaseHandler(),
          wire: original.wire,
          generalizedFrbRustBinding: binding,
          portManager: original.portManager,
        );
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
