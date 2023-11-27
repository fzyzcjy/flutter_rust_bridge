import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart'
    as frb_example_pure_dart_generator;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'test.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('test-rust', testRust, _$populateTestConfigParser, _$parseTestConfigResult),
    SimpleConfigCommand(
        'test-dart-native', testDartNative, _$populateTestDartConfigParser, _$parseTestDartConfigResult),
    SimpleConfigCommand('test-dart-web', testDartWeb, _$populateTestDartConfigParser, _$parseTestDartConfigResult),
  ];
}

@CliOptions()
class TestConfig {
  const TestConfig();
}

@CliOptions()
class TestDartConfig {
  final String package;

  const TestDartConfig({required this.package});
}

Future<void> testRust(TestConfig config) async {
  TODO;
  '''
    just install_expand
    just _test_rust_single frb_codegen --features uuid,chrono
    just _test_rust_single frb_rust
    just _test_rust_single frb_macros
    just _test_rust_single {{dir_example_pure_dart}}/rust
    just _test_rust_single {{dir_example_with_flutter}}/rust
    just _test_rust_single {{dir_example_pure_dart_multi}}/rust
    just _test_rust_single {{dir_example_pure_dart_multi}}/rust --features c-output
    just _test_rust_single {{dir_example_pure_dart_multi}}/rust --features c-output,extra-c-output-path

_test_rust_single directory *args:
    cd {{directory}} && cargo build {{args}}
    cd {{directory}} && cargo test {{args}}
  ''';
}

Future<void> testDartNative(TestDartConfig config) async {
  TODO;
}

Future<void> testDartWeb(TestDartConfig config) async {
  TODO;
}
