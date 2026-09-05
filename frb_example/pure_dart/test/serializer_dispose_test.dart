@TestOn('vm')
library;

import 'dart:ffi';

import 'package:flutter_rust_bridge/src/codec/sse.dart';
import 'package:flutter_rust_bridge/src/droppable/_common.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_uint8list/rust_vec_u8.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:frb_example_pure_dart/src/rust/api/lifetimeable.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();
  test('disposing a Rust vector frees its original allocation length', () {
    final binding = _AllocationBinding();
    final buffer = RustVecU8(8, binding);

    buffer.dispose();

    expect(binding.freedLength, 8);
    expect(binding.freedPointer, binding.pointer);
  });

  test('disposed opaque encoding releases the serializer allocation', () async {
    final probe = await LtOwnedStructTwinNormal.createTwinNormal(value: 'probe');
    probe.dispose();
    final binding = _AllocationBinding();
    final api = _RecordingApi(RustLib.instance.api as RustLibApiImpl, binding);

    expect(
      () => api.crateApiLifetimeableLtOwnedStructTwinNormalComputeTypeWithLifetimeTwinNormal(
          that: probe),
      throwsA(isA<DroppableDisposedException>()),
    );
    expect(binding.freedLength, 8);
    expect(binding.freeCount, 1);
  });

  test('disposing a serializer releases its allocation exactly once', () {
    final binding = _AllocationBinding();
    final serializer = SseSerializer(binding);

    serializer.dispose();

    expect(binding.freedLength, 8);
    expect(binding.freedPointer, binding.pointer);
    expect(serializer.dispose, throwsStateError);
    expect(serializer.intoRaw, throwsStateError);
    expect(binding.freeCount, 1);
  });

  test('transferring a serializer prevents Dart from freeing Rust ownership',
      () {
    final binding = _AllocationBinding();
    final serializer = SseSerializer(binding);

    final raw = serializer.intoRaw();

    expect(raw.ptr, binding.pointer);
    expect(raw.rustVecLen, 8);
    expect(raw.dataLen, 0);
    expect(serializer.dispose, throwsStateError);
    expect(serializer.intoRaw, throwsStateError);
    expect(binding.freeCount, 0);
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
  int freeCount = 0;

  @override
  Pointer<Uint8> rustVecU8New(int len) => pointer;

  @override
  void rustVecU8Free(Pointer<Uint8> ptr, int len) {
    freedLength = len;
    freedPointer = ptr;
    freeCount++;
  }

  @override
  dynamic noSuchMethod(Invocation invocation) =>
      throw UnsupportedError(invocation.memberName.toString());
}
