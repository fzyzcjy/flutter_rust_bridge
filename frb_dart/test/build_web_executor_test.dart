@TestOn('vm')
import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:test/test.dart';

void main() {
  test('default path returns full threaded wasm rustflags', () {
    final resolution = computeWasmPackRustflagsResolution(argsOverride: null);

    expect(resolution.rustflags, buildWebDefaultWasmPackRustflags);
    expect(resolution.rustflags, contains('-C link-args=--shared-memory'));
    expect(resolution.rustflags, contains('-C link-args=--import-memory'));
    expect(
      resolution.rustflags,
      contains('-C link-args=--export=__wasm_init_tls'),
    );
    expect(resolution.warning, isNull);
  });

  test('override path returns override unchanged', () {
    const override = '-C target-feature=+atomics';

    final resolution = computeWasmPackRustflagsResolution(
      argsOverride: override,
    );

    expect(resolution.rustflags, override);
  });

  test('override path without default threaded wasm flags returns warning', () {
    const override = '-C target-feature=+atomics';

    final resolution = computeWasmPackRustflagsResolution(
      argsOverride: override,
    );

    expect(resolution.warning, contains('default threaded-WASM flags'));
    expect(
      resolution.warning,
      contains('WebAssembly.Memory could not be cloned'),
    );
  });

  test('override path with default threaded wasm flags does not warn', () {
    const override =
        '$buildWebDefaultWasmPackRustflags -C link-args=--stack-first';

    final resolution = computeWasmPackRustflagsResolution(
      argsOverride: override,
    );

    expect(resolution.rustflags, override);
    expect(resolution.warning, isNull);
  });
}
