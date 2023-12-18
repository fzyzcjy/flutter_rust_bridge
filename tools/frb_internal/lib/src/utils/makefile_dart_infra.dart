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
    return await runCommand(
      command,
      args,
      env: {...?env, ...?extraEnv},
      pwd: '${pwd ?? ""}${relativePwd ?? ""}',
      checkExitCode: checkExitCode,
    );
  }
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

  SimpleCommand(
    this.name,
    this.executor, {
    this.description = '',
  });

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
  final timeStr =
      DateTime.now().toIso8601String().replaceAll(".", "").replaceAll(":", "");
  final randomStr = Random().nextInt(1000000000);
  return 'temp_${timeStr}_$randomStr';
}

String randomTempDir() =>
    path.join(Directory.systemTemp.path, randomTempDirName());
