import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/direct_sources.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';

class DartGenerator extends BaseGenerator {
  DartGenerator({required super.baseDir});

  @override
  Future<void> executeFormat() => executeDartFormat(workingDirectory: baseDir.toFilePath());

  @override
  Set<String> get duplicatorBlacklistNames => {};

  @override
  Map<String, String> generateDirectSources() => generateDartDirectSources();

  @override
  String get extension => 'dart';
}
