import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/direct_sources.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/duplicator.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:flutter_rust_bridge_internal/src/utils/generator_utils.dart';

Future<void> generateRust({required Uri rustRoot}) async {
  writeCodeFiles(rustRoot, generateRustDirectSources());
  generateRustDuplicates(dartRoot: dartRoot);
  await executeRustFormat(workingDirectory: rustRoot.toFilePath());
}
