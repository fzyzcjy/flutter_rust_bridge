@TestOn('vm')
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:test/test.dart';

void main() {
  group('BuildWebExecutor', () {
    test(
      'execute runs wasm-pack wasm-bindgen and dart compile when enabled',
      () async {
        final commands = <RecordedCommand>[];
        final tempDir = await Directory.systemTemp.createTemp(
          'build_web_executor',
        );
        addTearDown(() async => await tempDir.delete(recursive: true));
        await File(
          '${tempDir.path}/Cargo.toml',
        ).writeAsString('[package]\nname = "demo"\n');

        final executor = BuildWebExecutor(
          runCommandFn: makeRunCommandStub(
            commands: commands,
            handlers: {
              'which:wasm-pack': (_) async => success(),
              'which:wasm-bindgen': (_) async => success(),
              'cargo:read-manifest': (_) async => success(
                stdout: '{"targets":[{"kind":["cdylib"],"name":"demo_crate"}]}',
              ),
              'wasm-pack:build': (_) async => success(),
              'wasm-bindgen:${tempDir.path}/target/wasm32-unknown-unknown/release/demo_crate.wasm':
                  (_) async => success(),
              'dart:compile': (_) async => success(),
            },
          ),
          bailFn: throwBail,
          commandWhich: 'which',
        );

        await executor.execute(
          BuildWebArgs(
            output: '${tempDir.path}/web',
            release: true,
            verbose: true,
            rustCrateDir: tempDir.path,
            cargoBuildArgs: ['--features', 'demo'],
            wasmBindgenArgs: ['--weak-refs'],
            wasmPackRustupToolchain: 'nightly-2025-02-01',
            wasmPackRustflags: null,
            dartCompileJsEntrypoint: 'web/main.dart',
          ),
        );

        expect(
          commands
              .map((e) => '${e.command}:${e.arguments.firstOrNull ?? ''}')
              .toList(),
          containsAll([
            'which:wasm-pack',
            'which:wasm-bindgen',
            'cargo:read-manifest',
            'wasm-pack:build',
            'wasm-bindgen:${tempDir.path}/target/wasm32-unknown-unknown/release/demo_crate.wasm',
            'dart:compile',
          ]),
        );

        final wasmPack = commands.firstWhere((e) => e.command == 'wasm-pack');
        expect(wasmPack.arguments, containsAll(['--features', 'demo']));
        expect(wasmPack.env?['RUSTUP_TOOLCHAIN'], 'nightly-2025-02-01');
        expect(
          wasmPack.env?['RUSTFLAGS'],
          '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
        );

        final dartCompile = commands.firstWhere((e) => e.command == 'dart');
        expect(
          dartCompile.arguments,
          containsAll(['--verbose', 'web/main.dart']),
        );
      },
    );

    test('execute installs missing wasm-pack before continuing', () async {
      final commands = <RecordedCommand>[];
      var wasmPackInstalled = false;
      final tempDir = await Directory.systemTemp.createTemp(
        'build_web_install',
      );
      addTearDown(() async => await tempDir.delete(recursive: true));
      await File(
        '${tempDir.path}/Cargo.toml',
      ).writeAsString('[package]\nname = "demo"\n');

      final executor = BuildWebExecutor(
        runCommandFn: makeRunCommandStub(
          commands: commands,
          handlers: {
            'which:wasm-pack': (_) {
              if (wasmPackInstalled) return Future.value(success());
              throw ProcessException('which', ['wasm-pack'], 'missing', 1);
            },
            'cargo:install': (_) async {
              wasmPackInstalled = true;
              return success();
            },
            'cargo:read-manifest': (_) async => success(
              stdout: '{"targets":[{"kind":["cdylib"],"name":"demo_crate"}]}',
            ),
            'wasm-pack:build': (_) async => success(),
          },
        ),
        bailFn: throwBail,
        commandWhich: 'which',
      );

      await executor.execute(
        BuildWebArgs(
          output: '${tempDir.path}/web',
          release: false,
          verbose: false,
          rustCrateDir: tempDir.path,
          cargoBuildArgs: const [],
          wasmBindgenArgs: const [],
          wasmPackRustupToolchain: null,
          wasmPackRustflags:
              '-C target-feature=+atomics,+bulk-memory,+mutable-globals -C opt-level=z',
          dartCompileJsEntrypoint: null,
        ),
      );

      final installCommands = commands
          .where((e) => e.command == 'cargo')
          .toList();
      expect(
        installCommands.any(
          (e) => e.arguments.join(' ') == 'install wasm-pack',
        ),
        isTrue,
      );
    });

    test(
      'sanityChecks bails when crate directory is missing Cargo.toml',
      () async {
        final tempDir = await Directory.systemTemp.createTemp(
          'build_web_missing_crate',
        );
        addTearDown(() async => await tempDir.delete(recursive: true));

        final executor = BuildWebExecutor(
          runCommandFn: makeRunCommandStub(
            commands: [],
            handlers: {'which:wasm-pack': (_) async => success()},
          ),
          bailFn: throwBail,
          commandWhich: 'which',
        );

        await expectLater(
          () => executor.sanityChecks(
            BuildWebArgs(
              output: '${tempDir.path}/web',
              release: false,
              verbose: false,
              rustCrateDir: tempDir.path,
              cargoBuildArgs: const [],
              wasmBindgenArgs: const [],
              wasmPackRustupToolchain: null,
              wasmPackRustflags: null,
              dartCompileJsEntrypoint: null,
            ),
          ),
          throwsA(
            isA<BailException>().having(
              (e) => e.message,
              'message',
              contains('not a crate directory'),
            ),
          ),
        );
      },
    );

    test(
      'ensurePackageInstalled bails when install still does not provide binary',
      () async {
        final executor = BuildWebExecutor(
          runCommandFn: makeRunCommandStub(
            commands: [],
            handlers: {
              'which:wasm-bindgen': (_) => throw ProcessException(
                'which',
                ['wasm-bindgen'],
                'missing',
                1,
              ),
              'cargo:install': (_) async => success(),
            },
          ),
          bailFn: throwBail,
          commandWhich: 'which',
        );

        await expectLater(
          () => executor.ensurePackageInstalled(
            binaryName: 'wasm-bindgen',
            install: () async => await executor.runCommandFn('cargo', [
              'install',
              '-f',
              'wasm-bindgen-cli',
            ]),
            hint: 'install wasm-bindgen',
          ),
          throwsA(
            isA<BailException>().having(
              (e) => e.message,
              'message',
              'install wasm-bindgen',
            ),
          ),
        );
      },
    );

    test('getRustCrateName bails when crate name is empty', () async {
      final executor = BuildWebExecutor(
        runCommandFn: makeRunCommandStub(
          commands: [],
          handlers: {
            'cargo:read-manifest': (_) async =>
                success(stdout: '{"targets":[{"kind":["cdylib"],"name":""}]}'),
          },
        ),
        bailFn: throwBail,
      );

      await expectLater(
        () => executor.getRustCrateName(rustCrateDir: '.'),
        throwsA(
          isA<BailException>().having(
            (e) => e.message,
            'message',
            'Crate name cannot be empty.',
          ),
        ),
      );
    });

    test('computeRustflags keeps override and default branch behavior', () {
      expect(
        computeRustflags(argsOverride: null),
        '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
      );
      expect(
        computeRustflags(
          argsOverride:
              '-C target-feature=+atomics,+bulk-memory,+mutable-globals -C opt-level=z',
        ),
        '-C target-feature=+atomics,+bulk-memory,+mutable-globals -C opt-level=z',
      );
      expect(
        computeRustflags(argsOverride: '-C opt-level=z'),
        '-C opt-level=z',
      );
    });
  });
}

class RecordedCommand {
  final String command;
  final List<String> arguments;
  final String? pwd;
  final Map<String, String>? env;

  const RecordedCommand({
    required this.command,
    required this.arguments,
    required this.pwd,
    required this.env,
  });
}

typedef CommandHandler =
    Future<RunCommandOutput> Function(RecordedCommand command);

BuildWebRunCommand makeRunCommandStub({
  required List<RecordedCommand> commands,
  required Map<String, CommandHandler> handlers,
}) {
  return (
    String command,
    List<String> arguments, {
    String? pwd,
    Map<String, String>? env,
    bool shell = true,
    bool silent = false,
    bool? checkExitCode,
    bool printCommandInStderr = false,
  }) async {
    final recorded = RecordedCommand(
      command: command,
      arguments: List<String>.from(arguments),
      pwd: pwd,
      env: env == null ? null : Map<String, String>.from(env),
    );
    commands.add(recorded);

    final key = '$command:${arguments.firstOrNull ?? ''}';
    final handler = handlers[key];
    if (handler == null) {
      throw StateError('Missing handler for $key');
    }
    return handler(recorded);
  };
}

RunCommandOutput success({
  String stdout = '',
  String stderr = '',
  int exitCode = 0,
}) {
  return RunCommandOutput(stdout: stdout, stderr: stderr, exitCode: exitCode);
}

Never throwBail(String? message) => throw BailException(message);

class BailException implements Exception {
  final String? message;

  const BailException(this.message);
}

extension on List<String> {
  String? get firstOrNull => this.isEmpty ? null : first;
}
