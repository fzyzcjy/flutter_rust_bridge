import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/direct_sources.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:flutter_rust_bridge_internal/src/utils/generator_utils.dart';

Future<void> generateDart({required Uri dartRoot}) async {
  writeCodeFiles(dartRoot, generateDartDirectSources());
  await executeDartFormat(workingDirectory: dartRoot.toFilePath());
}
