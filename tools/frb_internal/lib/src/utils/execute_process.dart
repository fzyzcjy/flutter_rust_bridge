// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';

const kPinnedRustfmtNightly = 'nightly-2025-02-01';

Future<void> executeDartFormat({required String pwd}) async =>
    runCommand('dart', ['format', '.'], pwd: pwd);

Future<void> executeRustFormat({required String pwd}) async =>
    runCommand('cargo', ['+$kPinnedRustfmtNightly', 'fmt'], pwd: pwd);
