import 'dart:io';

Future<void> executeProcess(String executable, List<String> arguments, {String? workingDirectory}) async {
  print('executeProcess: $executable $arguments $workingDirectory');
  final process = await Process.start(executable, arguments, workingDirectory: workingDirectory);
  process.stdout.listen((e) => print(String.fromCharCodes(e)));
  process.stderr.listen((e) => print('[STDERR] ${String.fromCharCodes(e)}'));
  final exitCode = await process.exitCode;
  if (exitCode != 0) throw Exception('Process execution failed (exitCode=$exitCode)');
}

Future<void> executeDartFormat({required String workingDirectory}) async =>
    executeProcess('dart', ['format'], workingDirectory: workingDirectory);

Future<void> executeRustFormat({required String workingDirectory}) async =>
    executeProcess('cargo', ['+nightly', 'fmt'], workingDirectory: workingDirectory);
