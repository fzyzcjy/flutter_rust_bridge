import 'dart:io';

// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';

const kPinnedRustfmtNightly = 'nightly-2025-02-01';

Future<void> executeDartFormat({required String pwd}) async {
  final entries = ['build.dart', 'bin', 'benchmark', 'lib', 'test']
      .where(
        (entry) =>
            File('$pwd/$entry').existsSync() ||
            Directory('$pwd/$entry').existsSync(),
      )
      .toList();
  if (entries.isEmpty) return;
  if (File('$pwd/pubspec.yaml').existsSync()) {
    await runCommand('dart', ['pub', 'get'], pwd: pwd);
  }
  await runCommand('dart', ['format', ...entries], pwd: pwd);
}

Future<void> executeRustFormat({required String pwd}) async =>
    runCommand('cargo', ['+$kPinnedRustfmtNightly', 'fmt'], pwd: pwd);
