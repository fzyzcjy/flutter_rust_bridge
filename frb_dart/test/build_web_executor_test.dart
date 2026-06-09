@TestOn('vm')
import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
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

  test(
    'executeBuildWeb compiles configured Dart JavaScript and wasm outputs',
    () async {
      final crateDir = await Directory.systemTemp.createTemp(
        'frb_build_web_test_',
      );
      addTearDown(() async {
        if (await crateDir.exists()) {
          await crateDir.delete(recursive: true);
        }
      });
      await File('${crateDir.path}/Cargo.toml').writeAsString('[package]\n');
      final calls = <_CommandCall>[];

      await executeBuildWeb(
        BuildWebArgs(
          output: 'web',
          release: true,
          verbose: true,
          rustCrateDir: crateDir.path,
          cargoBuildArgs: const ['--features', 'demo_feature'],
          wasmBindgenArgs: const [],
          wasmPackRustupToolchain: null,
          wasmPackRustflags: null,
          dartCompileJsEntrypoint: 'web/main.dart',
          dartCompileWasmEntrypoint: 'web/main.dart',
        ),
        runCommandImpl:
            (
              command,
              arguments, {
              pwd,
              env,
              shell = true,
              silent = false,
              checkExitCode,
              printCommandInStderr = false,
              removedParentEnvKeys = const [],
              timeout,
            }) async {
              calls.add(
                _CommandCall(
                  command: command,
                  arguments: arguments,
                  pwd: pwd,
                  env: env,
                  silent: silent,
                  removedParentEnvKeys: removedParentEnvKeys,
                ),
              );
              if (command == 'cargo' && arguments.first == 'read-manifest') {
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
              return const RunCommandOutput(
                stdout: '',
                stderr: '',
                exitCode: 0,
              );
            },
      );

      expect(calls.map((call) => call.command), [
        isNotEmpty,
        'cargo',
        'wasm-pack',
        'dart',
        'dart',
      ]);

      final wasmPack = calls.singleWhere((call) => call.command == 'wasm-pack');
      expect(wasmPack.arguments, containsAllInOrder(['-d', 'web/pkg']));
      expect(
        wasmPack.arguments,
        containsAllInOrder(['--out-name', 'demo_crate']),
      );
      expect(wasmPack.arguments, containsAll(['--features', 'demo_feature']));
      expect(wasmPack.env, containsPair('RUSTUP_TOOLCHAIN', 'nightly'));
      expect(
        wasmPack.env,
        containsPair('RUSTFLAGS', buildWebDefaultWasmPackRustflags),
      );
      expect(wasmPack.removedParentEnvKeys, contains('LLVM_PROFILE_FILE'));

      final dartCompiles = calls
          .where((call) => call.command == 'dart')
          .map((call) => call.arguments)
          .toList();
      expect(dartCompiles, [
        [
          'compile',
          'js',
          '-o',
          'web/main.dart.js',
          '-O2',
          if (stdout.supportsAnsiEscapes) '--enable-diagnostic-colors',
          '--verbose',
          'web/main.dart',
        ],
        [
          'compile',
          'wasm',
          '-o',
          'web/main.dart.wasm',
          '-Dfrb.test.dart_wasm=true',
          '--verbose',
          'web/main.dart',
        ],
      ]);
    },
  );

  test('executeBuildWeb runs wasm-bindgen when flags are configured', () async {
    final crateDir = await Directory.systemTemp.createTemp(
      'frb_build_web_test_',
    );
    addTearDown(() async {
      if (await crateDir.exists()) {
        await crateDir.delete(recursive: true);
      }
    });
    await File('${crateDir.path}/Cargo.toml').writeAsString('[package]\n');
    final calls = <_CommandCall>[];

    await executeBuildWeb(
      BuildWebArgs(
        output: 'web',
        release: false,
        verbose: false,
        rustCrateDir: crateDir.path,
        cargoBuildArgs: const [],
        wasmBindgenArgs: const ['--weak-refs'],
        wasmPackRustupToolchain: 'nightly-2026-01-01',
        wasmPackRustflags: '-C target-feature=+atomics',
        dartCompileJsEntrypoint: null,
      ),
      runCommandImpl:
          (
            command,
            arguments, {
            pwd,
            env,
            shell = true,
            silent = false,
            checkExitCode,
            printCommandInStderr = false,
            removedParentEnvKeys = const [],
            timeout,
          }) async {
            calls.add(
              _CommandCall(
                command: command,
                arguments: arguments,
                pwd: pwd,
                env: env,
                silent: silent,
                removedParentEnvKeys: removedParentEnvKeys,
              ),
            );
            if (command == 'cargo' && arguments.first == 'read-manifest') {
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

    final wasmPack = calls.singleWhere((call) => call.command == 'wasm-pack');
    expect(wasmPack.arguments, contains('--dev'));
    expect(
      wasmPack.env,
      containsPair('RUSTUP_TOOLCHAIN', 'nightly-2026-01-01'),
    );
    expect(
      wasmPack.env,
      containsPair('RUSTFLAGS', '-C target-feature=+atomics'),
    );

    final wasmBindgen = calls.singleWhere(
      (call) => call.command == 'wasm-bindgen',
    );
    expect(
      wasmBindgen.arguments,
      containsAllInOrder([
        '${crateDir.path}/target/wasm32-unknown-unknown/debug/demo_crate.wasm',
        '--out-dir',
        'web/pkg',
        '--weak-refs',
      ]),
    );
    expect(calls.where((call) => call.command == 'dart'), isEmpty);
  });
}

class _CommandCall {
  final String command;
  final List<String> arguments;
  final String? pwd;
  final Map<String, String>? env;
  final bool silent;
  final List<String> removedParentEnvKeys;

  const _CommandCall({
    required this.command,
    required this.arguments,
    required this.pwd,
    required this.env,
    required this.silent,
    required this.removedParentEnvKeys,
  });
}
