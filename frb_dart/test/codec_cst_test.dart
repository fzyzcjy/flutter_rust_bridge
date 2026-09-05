import 'package:flutter_rust_bridge/src/codec/cst.dart';
import 'package:flutter_rust_bridge/src/droppable/_common.dart';
import 'package:flutter_rust_bridge/src/misc/rust_opaque.dart';
import 'package:test/test.dart';

void main() {
  test('CST validation preserves a live opaque handle', () {
    final opaque = _Opaque();
    cstValidateRustOpaque(opaque);
    expect(opaque.isDisposed, isFalse);
  });

  test('CST validation reports the disposed opaque type', () {
    final opaque = _Opaque()..dispose();
    expect(
      () => cstValidateRustOpaque(opaque),
      throwsA(
        isA<DroppableDisposedException>().having(
          (error) => error.name,
          'name',
          opaque.runtimeType.toString(),
        ),
      ),
    );
  });
}

class _Opaque implements RustOpaqueInterface {
  bool _disposed = false;

  @override
  bool get isDisposed => _disposed;

  @override
  void dispose() => _disposed = true;
}
