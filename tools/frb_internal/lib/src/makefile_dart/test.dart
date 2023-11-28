import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'test.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('test-rust', testRust, _$populateTestConfigParser,
        _$parseTestConfigResult),
    SimpleConfigCommand('test-dart-native', testDartNative,
        _$populateTestDartConfigParser, _$parseTestDartConfigResult),
    SimpleConfigCommand('test-dart-web', testDartWeb,
        _$populateTestDartConfigParser, _$parseTestDartConfigResult),
    SimpleConfigCommand('test-dart-valgrind', testDartValgrind,
        _$populateTestDartConfigParser, _$parseTestDartConfigResult),
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
  await exec('cargo build', relativePwd: package);
  await exec('cargo test', relativePwd: package, extraEnv: {
    // Because we have another CI to run the codegen and check outputs
    'FRB_SKIP_GENERATE_FRB_EXAMPLE_TEST': '1',
  });
}

Future<void> testDartNative(TestDartConfig config) async {
  await runDartPubGetIfNotRunYet(config.package);

  final dartMode = kDartModeOfPackage[config.package]!;

  var extraFlags = '';
  if (dartMode == DartMode.dart) {
    extraFlags += '--enable-experiment=native-assets ';
  }
  if (config.package == 'frb_example/pure_dart') {
    extraFlags += '--enable-vm-service ';
  }

  await exec('${dartMode.name} $extraFlags test', relativePwd: config.package);
}

Future<void> testDartWeb(TestDartConfig config) async {
  await runDartPubGetIfNotRunYet(config.package);

  final package = config.package;
  if (package == 'frb_dart') {
    await exec('dart test -p chrome', relativePwd: package);
  } else {
    await exec(
        'dart run flutter_rust_bridge_utils test-web --entrypoint ../$package/test/dart_web_test_entrypoint.dart',
        relativePwd: 'frb_utils');
  }
}

/// ref https://github.com/dart-lang/sdk/blob/master/runtime/tools/valgrind.py
Future<void> testDartValgrind(TestDartConfig config) async {
  await exec('sudo apt install -y valgrind');
  await runDartPubGetIfNotRunYet(config.package);

  const valgrindCommand = 'valgrind '
      '--error-exitcode=1 '
      '--leak-check=full '
      '--trace-children=yes '
      // Used for implicit null checks.
      '--ignore-ranges=0x000-0xFFF '
      // Valgrind crashes with the default level (2).
      '--vex-iropt-level=1';

  await exec('$valgrindCommand $TODO');

  throw Exception('implement the checks');
}
