import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/direct_sources.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';

class RustGenerator extends BaseGenerator {
  RustGenerator({required super.baseDir});

  @override
  Future<void> executeFormat() => executeRustFormat(workingDirectory: baseDir.toFilePath());

  @override
  Set<String> get duplicatorBlacklistNames => {'mod.rs'};

  @override
  Map<String, String> generateDirectSources() => generateRustDirectSources();

  @override
  String get extension => 'rs';
}
