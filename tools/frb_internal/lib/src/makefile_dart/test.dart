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
  for (final package in kRustPackages) {
    await testRustPackage(package);
  }
}

Future<void> testRustPackage(String package) async {
  await exec('cd $package && cargo build');
  await exec('cd $package && cargo test');
}

Future<void> testDartNative(TestDartConfig config) async {
  '''
    just install_ffigen_dependency
    just dart_pub_get dart_only
    just dart_test_simple pure_dart
    just dart_test_simple pure_dart_multi
    
    dart_test_simple name:
    just _dart_test_raw {{name}} ""

  _dart_test_raw name script_prefix:
    cd frb_example/{{name}}/rust && cargo build --verbose
    # need to be AOT, since prod environment is AOT, and JIT+valgrind will have strange problems
    cd frb_example/{{name}}/dart && dart compile exe bin/{{name}}.dart -o main.exe
    cd frb_example/{{name}}/dart && \
        {{script_prefix}} ./main.exe \
        "../../../target/debug/libflutter_rust_bridge_example_{{name}}.{{library_file_ext}}" --chain-stack-traces
  ''';
  TODO;
}

Future<void> testDartWeb(TestDartConfig config) async {
  '''
  dart_test_web_unit:
    cd frb_dart && dart pub get
    cd frb_dart && dart test test/*.dart
    cd frb_dart && dart test -p chrome test/*.dart

  dart_test_web_integration features="":
    just dart_pub_get dart_only
    cd {{dir_example_pure_dart}}/dart && dart run \
      ../../../frb_dart/bin/serve.dart \
      -c ../rust --dart-input lib/main.web.dart --root web/ --run-tests \
      --features={{features}}
  ''';
  TODO;
}
