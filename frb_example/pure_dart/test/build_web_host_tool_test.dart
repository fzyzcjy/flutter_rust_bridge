@TestOn('vm')
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:test/test.dart';

void main() {
  test('build-web permits installing a host tool in the wasm-pack environment',
      () async {
    final temp = await Directory.systemTemp.createTemp('frb_host_tool_');
    addTearDown(() => temp.delete(recursive: true));
    final source = Directory('${temp.path}/source');
    await Directory('${source.path}/src').create(recursive: true);
    await File('${source.path}/Cargo.toml').writeAsString('''
[package]
name = "frb-host-tool"
version = "0.1.0"
edition = "2021"

[workspace]
''');
    await File('${source.path}/src/main.rs').writeAsString(
      'fn main() { println!("host-tool-ready"); }',
    );

    // Use an already-installed toolchain: an uninstalled one (e.g. a pinned
    // nightly) makes rustup download it inside the 1-minute install timeout.
    final activeToolchainResult =
        await Process.run('rustup', ['show', 'active-toolchain']);
    expect(activeToolchainResult.exitCode, 0);
    final activeToolchain = (activeToolchainResult.stdout as String)
        .trim()
        .split(RegExp(r'\s+'))
        .first;

    var installed = false;
    await executeBuildWeb(
      BuildWebArgs(
        output: 'web',
        release: false,
        verbose: false,
        rustCrateDir: 'rust',
        cargoBuildArgs: [],
        wasmBindgenArgs: [],
        wasmPackRustupToolchain: activeToolchain,
        wasmPackRustflags: null,
        dartCompileJsEntrypoint: null,
        dartCompileWasmEntrypoint: null,
      ),
      runCommandImpl: (
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
        if (command == 'wasm-pack') {
          final result = await runCommand(
            'cargo',
            [
              'install',
              '--offline',
              '--path',
              source.path,
              '--root',
              temp.path
            ],
            pwd: source.path,
            env: env,
            removedParentEnvKeys: removedParentEnvKeys,
            timeout: const Duration(minutes: 1),
          );
          installed = true;
          return result;
        }
        if (arguments.contains('wasm-pack')) {
          return const RunCommandOutput(stdout: '', stderr: '', exitCode: 0);
        }
        return runCommand(command, arguments, pwd: pwd);
      },
    );

    expect(installed, isTrue);
    final executable =
        Platform.isWindows ? 'frb-host-tool.exe' : 'frb-host-tool';
    final output = await runCommand('${temp.path}/bin/$executable', []);
    expect(output.stdout.trim(), 'host-tool-ready');
  }, timeout: const Timeout(Duration(minutes: 2)));
}
