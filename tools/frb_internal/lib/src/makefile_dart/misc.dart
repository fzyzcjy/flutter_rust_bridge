import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart' as generator;
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('generate-test', generator.generate),
  ];
}

const kRustPackages = <String>[
  'frb_rust',
  'frb_codegen',
  'frb_example/dart_minimal/rust',
  'frb_example/pure_dart/rust',
  // TODO `with_flutter` example
];

const kDartPackages = <String>[
  'frb_dart',
  'frb_example/dart_minimal',
  'frb_example/pure_dart',
  // TODO `with_flutter` example
  'frb_utils',
  'tools/frb_internal',
];
