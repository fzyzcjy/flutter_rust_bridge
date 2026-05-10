@TestOn('vm')
import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:test/test.dart';

void main() {
  tearDown(resetBuildWebCommandOverridesForTest);

  test('executeBuildWeb runs the full release command sequence', () async {
    final rustCrateDir = await Directory.systemTemp.createTemp();
    addTearDown(() async => await rustCrateDir.delete(recursive: true));
    File('${rustCrateDir.path}/Cargo.toml').writeAsStringSync('[package]\n');

    final checkedBinaries = <String>[];
    final calls = <_CommandCall>[];
    setBuildWebCommandOverridesForTest(
      isBinaryInstalled: (binaryName) async {
        checkedBinaries.add(binaryName);
        return true;
      },
      supportsAnsiEscapes: true,
      runCommand:
          (
            command,
            arguments, {
            pwd,
            env,
            shell = true,
            silent = false,
            checkExitCode,
            printCommandInStderr = false,
          }) async {
            calls.add(
              _CommandCall(
                command: command,
                arguments: arguments,
                pwd: pwd,
                env: env,
                silent: silent,
              ),
            );

            if (command == 'cargo' &&
                arguments.length == 1 &&
                arguments.first == 'read-manifest') {
              return RunCommandOutput(
                stdout: jsonEncode({
                  'targets': [
                    {
                      'kind': ['cdylib'],
                      'name': 'demo_crate',
                    },
                  ],
                }),
                stderr: '',
                exitCode: 0,
              );
            }

            return const RunCommandOutput(stdout: '', stderr: '', exitCode: 0);
          },
    );

    await executeBuildWeb(
      BuildWebArgs(
        output: '/tmp/out',
        release: true,
        verbose: true,
        rustCrateDir: rustCrateDir.path,
        cargoBuildArgs: const ['--features', 'threaded'],
        wasmBindgenArgs: const ['--weak-refs'],
        wasmPackRustupToolchain: 'nightly-custom',
        wasmPackRustflags: buildWebDefaultWasmPackRustflags,
        dartCompileJsEntrypoint: 'web/main.dart',
      ),
    );

    expect(checkedBinaries, ['wasm-pack', 'wasm-bindgen']);
    expect(calls.map((call) => call.command), [
      'cargo',
      'wasm-pack',
      'wasm-bindgen',
      'dart',
    ]);

    final wasmPackCall = calls.singleWhere(
      (call) => call.command == 'wasm-pack',
    );
    expect(wasmPackCall.arguments, containsAllInOrder(['-d', '/tmp/out/pkg']));
    expect(wasmPackCall.arguments, isNot(contains('--dev')));
    expect(wasmPackCall.arguments, containsAll(['--features', 'threaded']));
    expect(wasmPackCall.env, {
      'RUSTUP_TOOLCHAIN': 'nightly-custom',
      'RUSTFLAGS': buildWebDefaultWasmPackRustflags,
      'CARGO_TERM_COLOR': 'always',
    });

    final wasmBindgenCall = calls.singleWhere(
      (call) => call.command == 'wasm-bindgen',
    );
    expect(
      wasmBindgenCall.arguments.first,
      '${rustCrateDir.path}/target/wasm32-unknown-unknown/release/demo_crate.wasm',
    );
    expect(wasmBindgenCall.arguments, contains('--weak-refs'));

    final dartCall = calls.singleWhere((call) => call.command == 'dart');
    expect(
      dartCall.arguments,
      containsAllInOrder([
        'compile',
        'js',
        '-o',
        '/tmp/out/main.dart.js',
        '-O2',
        '--enable-diagnostic-colors',
        '--verbose',
        'web/main.dart',
      ]),
    );
  });

  test('executeBuildWeb installs missing wasm-pack before dev build', () async {
    final rustCrateDir = await Directory.systemTemp.createTemp();
    addTearDown(() async => await rustCrateDir.delete(recursive: true));
    File('${rustCrateDir.path}/Cargo.toml').writeAsStringSync('[package]\n');

    var wasmPackChecks = 0;
    final calls = <_CommandCall>[];
    setBuildWebCommandOverridesForTest(
      isBinaryInstalled: (binaryName) async {
        if (binaryName != 'wasm-pack') return true;
        wasmPackChecks++;
        return wasmPackChecks > 1;
      },
      supportsAnsiEscapes: false,
      runCommand:
          (
            command,
            arguments, {
            pwd,
            env,
            shell = true,
            silent = false,
            checkExitCode,
            printCommandInStderr = false,
          }) async {
            calls.add(
              _CommandCall(
                command: command,
                arguments: arguments,
                pwd: pwd,
                env: env,
                silent: silent,
              ),
            );

            if (command == 'cargo' &&
                arguments.length == 1 &&
                arguments.first == 'read-manifest') {
              return RunCommandOutput(
                stdout: jsonEncode({
                  'targets': [
                    {
                      'kind': ['cdylib'],
                      'name': 'demo_crate',
                    },
                  ],
                }),
                stderr: '',
                exitCode: 0,
              );
            }

            return const RunCommandOutput(stdout: '', stderr: '', exitCode: 0);
          },
    );

    await executeBuildWeb(
      BuildWebArgs(
        output: '/tmp/out',
        release: false,
        verbose: false,
        rustCrateDir: rustCrateDir.path,
        cargoBuildArgs: const [],
        wasmBindgenArgs: const [],
        wasmPackRustupToolchain: null,
        wasmPackRustflags: null,
        dartCompileJsEntrypoint: null,
      ),
    );

    expect(
      calls
          .where(
            (call) =>
                call.command == 'cargo' &&
                call.arguments.join(' ') == 'install wasm-pack',
          )
          .length,
      1,
    );

    final wasmPackCall = calls.singleWhere(
      (call) => call.command == 'wasm-pack',
    );
    expect(wasmPackCall.arguments, contains('--dev'));
    expect(wasmPackCall.env, {
      'RUSTUP_TOOLCHAIN': 'nightly',
      'RUSTFLAGS': buildWebDefaultWasmPackRustflags,
    });
  });

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
}

class _CommandCall {
  final String command;
  final List<String> arguments;
  final String? pwd;
  final Map<String, String>? env;
  final bool silent;

  const _CommandCall({
    required this.command,
    required this.arguments,
    required this.pwd,
    required this.env,
    required this.silent,
  });
}
