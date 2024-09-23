import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_utils/src/dart_web_test_utils/run_test.dart';

part 'test_web_command.g.dart';

class TestWebCommand extends _$TestWebConfigCommand<void> {
  @override
  String get name => 'test-web';

  @override
  String get description => '';

  @override
  Future<void> run() async {
    await executeTestWeb(_options);
  }
}

@CliOptions(createCommand: true)
class TestWebConfig {
  @CliOption(help: 'Dart file to be tested')
  late String entrypoint;

  @CliOption(help: 'Make the browser headless', defaultsTo: true)
  late bool headless;

  @CliOption(help: 'Rust feature flags to set during build')
  late List<String> rustFeatures;
}
