import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart' as generator;

class ServeWebCommand extends Command<void> {
  @override
  String get name => 'serve-web';

  @override
  String get description => '';

  @override
  Future<void> run() async {
    TODO;
  }
}
