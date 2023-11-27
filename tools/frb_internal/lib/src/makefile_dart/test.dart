import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart'
    as frb_example_pure_dart_generator;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'test.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('test', test, _$populateTestConfigParser, _$parseTestConfigResult),
    SimpleConfigCommand('test-rust', testRust, _$populateTestConfigParser, _$parseTestConfigResult),
    SimpleConfigCommand('test-dart-native', testDartNative, _$populateTestConfigParser, _$parseTestConfigResult),
    SimpleConfigCommand('test-dart-web', testDartWeb, _$populateTestConfigParser, _$parseTestConfigResult),
  ];
}

@CliOptions()
class TestConfig {
  const TestConfig();
}

Future<void> test(TestConfig config) async {
  await testRust(config);
  await testDartNative(config);
  await testDartWeb(config);
}

Future<void> testRust(TestConfig config) async {
  TODO;
}

Future<void> testDartNative(TestConfig config) async {
  TODO;
}

Future<void> testDartWeb(TestConfig config) async {
  TODO;
}
