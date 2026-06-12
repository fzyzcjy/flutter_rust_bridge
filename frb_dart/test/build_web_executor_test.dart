@TestOn('vm')
import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:test/test.dart';

void main() {
  test('default path returns full threaded wasm rustflags', () {
    final resolution = computeWasmPackRustflagsResolution(argsOverride: null);

    expect(resolution.rustflags, buildWebDefaultWasmPackRustflags);
    expect(resolution.rustflags, contains('-C link-args=--shared-memory'));
    expect(resolution.rustflags, contains('-C link-args=--import-memory'));
    expect(resolution.rustflags, contains('-C link-args=--export=__heap_base'));
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
    final override =
        '$buildWebDefaultWasmPackRustflags -C link-args=--stack-first';

    final resolution = computeWasmPackRustflagsResolution(
      argsOverride: override,
    );

    expect(resolution.rustflags, override);
    expect(resolution.warning, isNull);
  });

  test(
    'override path with reordered default threaded wasm flags does not warn',
    () {
      const override =
          '-C link-args=--stack-first '
          '-C link-args=--export=__tls_base '
          '-C target-feature=+atomics,+bulk-memory,+mutable-globals '
          '-C link-args=--export=__tls_align '
          '-C link-args=--max-memory=1073741824 '
          '-C link-args=--import-memory '
          '-C link-args=--export=__heap_base '
          '-C link-args=--shared-memory '
          '-C link-args=--export=__tls_size '
          '-C link-args=--export=__wasm_init_tls';

      final resolution = computeWasmPackRustflagsResolution(
        argsOverride: override,
      );

      expect(resolution.rustflags, override);
      expect(resolution.warning, isNull);
    },
  );

  test('wasm-pack environment drops cargo llvm-cov variables', () {
    final resolution = computeWasmPackEnvironmentResolution(
      baseEnvironment: const {
        'PATH': '/usr/bin',
        'HOME': '/home/test',
        'FRB_CUSTOM_TOOLCHAIN_ENV': 'kept',
        'CARGO_LLVM_COV': '1',
        'CARGO_LLVM_COV_EXTRA': 'extra',
        'CARGO_LLVM_COV_TARGET_DIR': '/tmp/target',
        'LLVM_PROFILE_FILE': '/tmp/profile-%p.profraw',
        'RUSTC_WRAPPER': 'cargo-llvm-cov',
        '__CARGO_LLVM_COV_RUSTC_WRAPPER': '1',
        '__CARGO_LLVM_COV_EXTRA': 'extra',
        'RUSTFLAGS': '-C instrument-coverage',
      },
      rustupToolchain: 'nightly',
      rustflags: '-C target-feature=+atomics',
      cargoTermColor: true,
    );
    final environment = resolution.environment;

    expect(environment['PATH'], '/usr/bin');
    expect(environment['HOME'], '/home/test');
    expect(environment['FRB_CUSTOM_TOOLCHAIN_ENV'], 'kept');
    expect(environment['RUSTUP_TOOLCHAIN'], 'nightly');
    expect(environment['RUSTFLAGS'], '-C target-feature=+atomics');
    expect(environment['CARGO_TERM_COLOR'], 'always');
    expect(environment, isNot(contains('CARGO_LLVM_COV')));
    expect(environment, isNot(contains('CARGO_LLVM_COV_EXTRA')));
    expect(environment, isNot(contains('CARGO_LLVM_COV_TARGET_DIR')));
    expect(environment, isNot(contains('LLVM_PROFILE_FILE')));
    expect(environment, isNot(contains('RUSTC_WRAPPER')));
    expect(environment, isNot(contains('__CARGO_LLVM_COV_RUSTC_WRAPPER')));
    expect(environment, isNot(contains('__CARGO_LLVM_COV_EXTRA')));
    expect(
      resolution.removedCoverageEnvironmentKeys,
      containsAllInOrder([
        'CARGO_LLVM_COV',
        'CARGO_LLVM_COV_EXTRA',
        'CARGO_LLVM_COV_TARGET_DIR',
        'LLVM_PROFILE_FILE',
        'RUSTC_WRAPPER',
        'RUSTFLAGS',
        '__CARGO_LLVM_COV_EXTRA',
        '__CARGO_LLVM_COV_RUSTC_WRAPPER',
      ]),
    );
  });
}
