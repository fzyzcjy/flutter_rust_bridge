// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';

Future<void> executeDartFormat({required String pwd}) async =>
    runCommand('dart', ['format', '.'], pwd: pwd);

Future<void> executeRustFormat({required String pwd}) async =>
    runCommand('cargo', ['+nightly', 'fmt'], pwd: pwd);
