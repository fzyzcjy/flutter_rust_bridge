import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart' as generator;

class GenerateTestCommand extends Command<void> {
  @override
  String get name => 'generate-test';

  @override
  String get description => '';

  @override
  Future<void> run() async {
    await generator.generate();
  }
}
