import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/direct_sources.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/duplicator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';

Future<void> generateDart({required Uri dartRoot}) async {
  writeCodeFiles(dartRoot, generateDartDirectSources());
  generateDartDuplicates(dartRoot: dartRoot);
  await executeDartFormat(workingDirectory: dartRoot.toFilePath());
}
