import 'dart:async';
import 'dart:convert';
import 'dart:io';

/// {@macro flutter_rust_bridge.internal}
class RunCommandOutput {
  /// {@macro flutter_rust_bridge.internal}
  final String stdout;

  /// {@macro flutter_rust_bridge.internal}
  final String stderr;

  /// {@macro flutter_rust_bridge.internal}
  final int exitCode;

  /// {@macro flutter_rust_bridge.internal}
  const RunCommandOutput({
    required this.stdout,
    required this.stderr,
    required this.exitCode,
  });
}

/// {@macro flutter_rust_bridge.internal}
Future<RunCommandOutput> runCommand(
  String command,
  List<String> arguments, {
  String? pwd,
  Map<String, String>? env,
  bool shell = true,
  bool silent = false,
  bool? checkExitCode,
  bool printCommandInStderr = false,
  List<String> removedParentEnvKeys = const [],
  Duration? timeout,
}) async {
  final processEnvironment = removedParentEnvKeys.isEmpty
      ? env
      : {
          for (final entry in Platform.environment.entries)
            if (!removedParentEnvKeys.contains(entry.key))
              entry.key: entry.value,
          ...?env,
        };
  final displayEnvironment = removedParentEnvKeys.isEmpty
      ? env
      : {...?env, 'removedParentEnvKeys': removedParentEnvKeys.join(',')};

  // ignore: avoid_print
  (printCommandInStderr ? stderr : stdout).writeAndFlush(
    '\x1B[1m> $command ${arguments.join(' ')}\x1B[0m (pwd: $pwd, env: $displayEnvironment)\n',
  );

  final process = await Process.start(
    command,
    arguments,
    runInShell: shell,
    workingDirectory: pwd,
    environment: processEnvironment,
    includeParentEnvironment: removedParentEnvKeys.isEmpty,
  );

  final stdoutText = <String>[];
  final stderrText = <String>[];

  process.stdout.transform(utf8.decoder).listen((line) {
    if (!silent) stdout.writeAndFlush(line);
    stdoutText.add(line);
  });

  process.stderr.transform(utf8.decoder).listen((line) {
    if (!silent) stderr.writeAndFlush(line);
    stderrText.add(line);
  });

  final int exitCode;
  try {
    exitCode = timeout == null
        ? await process.exitCode
        : await process.exitCode.timeout(timeout);
  } on TimeoutException {
    await _killProcessTree(process.pid);
    throw TimeoutException(
      'Command timed out after $timeout: $command ${arguments.join(" ")}',
      timeout,
    );
  }
  if ((checkExitCode ?? true) && (exitCode != 0)) {
    const envKey = 'FRB_DART_RUN_COMMAND_STDERR';
    final enableStderr = Platform.environment[envKey] == '1';
    final info = enableStderr
        ? 'stderr=${stderrText.join("")}'
        : 'If you want to see extra information, set $envKey=1';
    throw ProcessException(
      command,
      arguments,
      'Bad exit code ($exitCode). $info',
      exitCode,
    );
  }

  return RunCommandOutput(
    stdout: stdoutText.join(''),
    stderr: stderrText.join(''),
    exitCode: exitCode,
  );
}

Future<void> _killProcessTree(int pid) async {
  // ignore: avoid_print
  print('Killing process tree rooted at PID $pid');

  if (Platform.isWindows) {
    try {
      await Process.run('taskkill', ['/F', '/T', '/PID', '$pid']);
    } on ProcessException {
      Process.killPid(pid, ProcessSignal.sigkill);
    }
    return;
  }

  final pids = await _processTreeIds(pid);
  var didSignal = false;
  for (final pid in pids.reversed) {
    didSignal |= Process.killPid(pid);
  }
  if (!didSignal) {
    return;
  }

  await Future<void>.delayed(const Duration(milliseconds: 500));
  for (final pid in pids.reversed) {
    Process.killPid(pid, ProcessSignal.sigkill);
  }
}

Future<List<int>> _processTreeIds(int pid) async {
  final pids = [pid];
  for (final childPid in await _childProcessIds(pid)) {
    pids.addAll(await _processTreeIds(childPid));
  }
  return pids;
}

Future<List<int>> _childProcessIds(int pid) async {
  final ProcessResult result;
  try {
    result = await Process.run('ps', ['-eo', 'pid=,ppid=']);
  } on ProcessException {
    return const [];
  }

  if (result.exitCode != 0) return const [];

  return (result.stdout as String)
      .split('\n')
      .map((line) => line.trim().split(RegExp(r'\s+')))
      .where((parts) => parts.length == 2 && parts[1] == '$pid')
      .map((parts) => int.tryParse(parts[0]))
      .nonNulls
      .toList();
}

extension on IOSink {
  void writeAndFlush(String message) {
    write(message);
    flush();
  }
}
