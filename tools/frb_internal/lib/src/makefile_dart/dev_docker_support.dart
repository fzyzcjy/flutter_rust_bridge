import 'dart:io';

// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:path/path.dart' as path;

typedef DevDockerWorkflowCommandRunner =
    Future<RunCommandOutput> Function(
      String command, {
      String? relativePwd,
      Map<String, String>? extraEnv,
      bool? checkExitCode,
    });

String shellEscape(String value) {
  return "'${value.replaceAll("'", r"'\''")}'";
}

void appendTextToFile({required String outputPath, required String text}) {
  final outputFile = File(outputPath);
  outputFile.parent.createSync(recursive: true);
  outputFile.writeAsStringSync(text, mode: FileMode.append);
}

String resolveRepoPath({
  required String rawPath,
  required String repoRootPath,
}) {
  if (path.isAbsolute(rawPath)) {
    return path.normalize(rawPath);
  }

  return path.normalize(path.join(repoRootPath, rawPath));
}
