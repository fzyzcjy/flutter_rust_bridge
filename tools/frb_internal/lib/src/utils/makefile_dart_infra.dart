import 'dart:io';
import 'dart:math';

import 'package:args/args.dart';
import 'package:args/command_runner.dart';

// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:path/path.dart' as path;

class SimpleExecutor {
  final Map<String, String>? env;
  final String? pwd;

  const SimpleExecutor({this.env, this.pwd});

  Future<RunCommandOutput> call(
    String cmd, {
    String? relativePwd,
    Map<String, String>? extraEnv,
    bool? checkExitCode,
    bool silent = false,
    Duration? timeout,
    bool inheritStdio = false,
  }) async {
    final String command;
    final List<String> args;
    if (Platform.isWindows) {
      command = 'powershell';
      args = ['-noprofile', '-command', '& $cmd'];
    } else {
      command = '/bin/sh';
      args = ['-c', cmd];
    }
    final effectivePwd = '${pwd ?? ""}${relativePwd ?? ""}';
    if (inheritStdio) {
      assert(!silent && timeout == null);
      return await _runCommandInheritingStdio(
        command,
        args,
        env: {...?env, ...?extraEnv},
        pwd: effectivePwd,
        checkExitCode: checkExitCode ?? true,
      );
    }
    return await runCommand(
      command,
      args,
      env: {...?env, ...?extraEnv},
      pwd: effectivePwd,
      checkExitCode: checkExitCode,
      silent: silent,
      timeout: timeout,
    );
  }
}

/// Runs a command with this process's own stdio instead of pipes, so its output
/// is not captured (the returned stdout/stderr are empty).
///
/// On Windows, Dart creates the child ends of captured stdio pipes with
/// FILE_FLAG_OVERLAPPED. Rust's std assumes inherited stdio is synchronous and
/// aborts with "operation failed to complete synchronously" when concurrent
/// writes to such a pipe leave one pending, which can crash Rust test binaries.
Future<RunCommandOutput> _runCommandInheritingStdio(
  String command,
  List<String> arguments, {
  required Map<String, String> env,
  required String pwd,
  required bool checkExitCode,
}) async {
  stdout.writeln(
    '\x1B[1m> $command ${arguments.join(' ')}\x1B[0m '
    '(pwd: $pwd, env: $env, inheritStdio: true)',
  );
  final process = await Process.start(
    command,
    arguments,
    workingDirectory: pwd.isEmpty ? null : pwd,
    environment: env,
    mode: ProcessStartMode.inheritStdio,
  );
  final exitCode = await process.exitCode;
  if (checkExitCode && exitCode != 0) {
    throw ProcessException(
      command,
      arguments,
      'Bad exit code ($exitCode).',
      exitCode,
    );
  }
  return RunCommandOutput(stdout: '', stderr: '', exitCode: exitCode);
}

extension ExtCommandRunner<T> on CommandRunner<T> {
  void addCommands(List<Command<T>> commands) {
    for (final command in commands) {
      addCommand(command);
    }
  }
}

class SimpleCommand extends Command<void> {
  @override
  final String name;
  @override
  final String description;
  final Future<void> Function() executor;

  SimpleCommand(this.name, this.executor, {this.description = ''});

  @override
  Future<void> run() async => await executor();
}

class SimpleConfigCommand<T> extends Command<void> {
  @override
  final String name;
  @override
  final String description;
  final Future<void> Function(T config) executor;

  final void Function(ArgParser) populateConfigParser;
  final T Function(ArgResults) parseConfigResult;

  SimpleConfigCommand(
    this.name,
    this.executor,
    this.populateConfigParser,
    this.parseConfigResult, {
    this.description = '',
  }) {
    populateConfigParser(argParser);
  }

  @override
  Future<void> run() async => await executor(parseConfigResult(argResults!));
}

String randomTempDirName() {
  final timeStr = DateTime.now()
      .toIso8601String()
      .replaceAll(".", "")
      .replaceAll(":", "");
  final randomStr = Random().nextInt(1000000000);
  return 'temp_${timeStr}_$randomStr';
}

String randomTempDir() =>
    path.join(Directory.systemTemp.path, randomTempDirName());
