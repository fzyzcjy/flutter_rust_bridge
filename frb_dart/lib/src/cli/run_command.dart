import 'dart:convert';
import 'dart:io';

// final arrow = stdout.supportsAnsiEscapes ? Colorize('>').green().bold().toString() : '>'; // #1262
const _kArrow = '>';

/// {@macro flutter_rust_bridge.internal}
Future<String> runCommand(
  String command,
  List<String> arguments, {
  String? pwd,
  Map<String, String>? env,
  bool shell = true,
  bool silent = false,
}) async {
  // ignore: avoid_print
  print('$_kArrow $command ${arguments.join(' ')} (pwd: $pwd, env: $env)');
  final process = await Process.start(
    command,
    arguments,
    runInShell: shell,
    workingDirectory: pwd,
    environment: env,
  );

  final ret = <String>[];
  final err = <String>[];

  process.stdout.transform(utf8.decoder).listen((line) {
    if (!silent) stdout.write(line);
    ret.add(line);
  });

  process.stderr.transform(utf8.decoder).listen((line) {
    if (!silent) stderr.write(line);
    err.add(line);
  });

  final exitCode = await process.exitCode;
  if (exitCode != 0) {
    throw ProcessException(command, arguments, err.join(''), exitCode);
  }

  return ret.join('');
}
