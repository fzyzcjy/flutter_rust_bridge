import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';

class RustGenerator extends BaseGenerator {
  RustGenerator({required super.baseDir});

  @override
  Future<void> executeFormat() => executeRustFormat(workingDirectory: baseDir.toFilePath());
}
