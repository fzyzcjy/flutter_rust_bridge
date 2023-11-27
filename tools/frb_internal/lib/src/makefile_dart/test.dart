import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
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
  await exec('cd ${config.package} && ${kDartModeOfPackage[config.package]!.name} test');
}

Future<void> testDartWeb(TestDartConfig config) async {
  final package = config.package;
  if (package == 'frb_dart') {
    await exec('cd $package && dart test -p chrome');
  } else {
    TODO;
    '''
  dart_test_web_integration features="":
    cd {{dir_example_pure_dart}}/dart && dart run \
      ../../../frb_dart/bin/serve.dart \
      -c ../rust --dart-input lib/main.web.dart --root web/ --run-tests \
      --features={{features}}
  ''';
  }
}
