import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester.dart'
    as dart_sanitizer_tester;
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:meta/meta.dart';

part 'test.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('test-rust', testRust, _$populateTestRustConfigParser,
        _$parseTestRustConfigResult),
    SimpleConfigCommand('test-dart-native', testDartNative,
        _$populateTestDartConfigParser, _$parseTestDartConfigResult),
    SimpleConfigCommand('test-dart-web', testDartWeb,
        _$populateTestDartConfigParser, _$parseTestDartConfigResult),
    SimpleConfigCommand('test-dart-valgrind', testDartValgrind,
        _$populateTestDartConfigParser, _$parseTestDartConfigResult),
    SimpleConfigCommand(
        'test-dart-sanitizer',
        testDartSanitizer,
        _$populateTestDartSanitizerConfigParser,
        _$parseTestDartSanitizerConfigResult),
    SimpleConfigCommand('test-flutter-native', testFlutterNative,
        _$populateTestFlutterConfigParser, _$parseTestFlutterConfigResult),
    SimpleConfigCommand('test-flutter-web', testFlutterWeb,
        _$populateTestDartConfigParser, _$parseTestDartConfigResult),
  ];
}

@CliOptions()
class TestConfig {
  const TestConfig();
}

@CliOptions()
class TestRustConfig {
  final bool updateGoldens;
  final bool coverage;

  const TestRustConfig({required this.updateGoldens, required this.coverage});
}

@CliOptions()
class TestDartConfig {
  final String package;
  final bool coverage;

  const TestDartConfig({required this.package, required this.coverage});
}

enum Sanitizer {
  asan,
  msan,
  lsan,
  tsan,
}

@CliOptions()
class TestDartSanitizerConfig {
  final String package;
  final bool useLocalSanitizedDartBinary;
  final Sanitizer sanitizer;

  const TestDartSanitizerConfig({
    required this.package,
    required this.useLocalSanitizedDartBinary,
    required this.sanitizer,
  });
}

@CliOptions()
class TestFlutterConfig {
  final String? flutterTestArgs;
  final String package;

  const TestFlutterConfig({this.flutterTestArgs, required this.package});
}

Future<void> testRust(TestRustConfig config) async {
  for (final package in kRustPackages) {
    await testRustPackage(config, package);
  }
}

Future<void> testRustPackage(TestRustConfig config, String package) async {
  await exec('cargo build', relativePwd: package);

  // TODO test coverage
  await exec('cargo test', relativePwd: package, extraEnv: {
    // Because we have another CI to run the codegen and check outputs
    'FRB_SKIP_GENERATE_FRB_EXAMPLE_TEST': '1',
    if (config.updateGoldens) 'UPDATE_GOLDENS': '1',
    ...kEnvEnableRustBacktrace,
  });
}

Future<void> testDartNative(TestDartConfig config) async {
  await runPubGetIfNotRunYet(config.package);

  final dartMode = kDartModeOfPackage[config.package]!;

  var extraFlags = '';
  if (dartMode == DartMode.dart) {
    extraFlags += '--enable-experiment=native-assets ';
  }
  if (config.package == 'frb_example/pure_dart') {
    extraFlags += '--enable-vm-service ';
  }

  await exec(
      '${dartMode.name} $extraFlags test ${config.coverage ? " --coverage" : ""}',
      relativePwd: config.package,
      extraEnv: kEnvEnableRustBacktrace);
}

Future<void> testDartWeb(TestDartConfig config) async {
  await runPubGetIfNotRunYet(config.package);

  final package = config.package;
  if (package == 'frb_dart') {
    await exec('dart test -p chrome ${config.coverage ? " --coverage" : ""}',
        relativePwd: package, extraEnv: kEnvEnableRustBacktrace);
  } else {
    await exec(
        'dart run flutter_rust_bridge_utils test-web --entrypoint ../$package/test/dart_web_test_entrypoint.dart',
        relativePwd: 'frb_utils',
        extraEnv: kEnvEnableRustBacktrace);
  }
}

/// ref https://github.com/dart-lang/sdk/blob/master/runtime/tools/valgrind.py
Future<void> testDartValgrind(TestDartConfig config) async {
  await exec('sudo apt install -y valgrind');
  await runPubGetIfNotRunYet(config.package);

  await exec(
      'dart --enable-experiment=native-assets build '
      'test/dart_valgrind_test_entrypoint.dart -o build/valgrind_test_output/',
      relativePwd: config.package);

  const valgrindCommand = 'valgrind '
      '--error-exitcode=1 '
      '--leak-check=full '
      '--trace-children=yes '
      // Used for implicit null checks.
      '--ignore-ranges=0x000-0xFFF '
      // Valgrind crashes with the default level (2).
      '--vex-iropt-level=1';

  final output = await exec(
    '$valgrindCommand build/valgrind_test_output/dart_valgrind_test_entrypoint.exe',
    relativePwd: config.package,
    checkExitCode: false,
    extraEnv: kEnvEnableRustBacktrace,
  );

  checkValgrindOutput(output.stdout);
}

@visibleForTesting
void checkValgrindOutput(String output) {
  const kDartAllTestsPassedStr = 'All tests passed!';
  if (!output.contains(kDartAllTestsPassedStr)) {
    throw Exception(
        'valgrind_util does not find "$kDartAllTestsPassedStr", thus dart test seems failed');
  }

  const re = r'(?:definitely|indirectly) lost: (\d+) bytes';
  final matches = RegExp(re).allMatches(output).toList();
  if (![0, 2, 3].contains(matches.length)) {
    throw Exception('Invalid number of matches for `$re` (matches=$matches)');
  }

  for (final match in matches) {
    final lostBytes = int.parse(match.group(1)!);
    if (lostBytes != 0) {
      throw Exception(
        'There are some lost bytes, so the check fails. '
        'This may or may not be a problem. '
        'If you can confirm the lost bytes are reasonable, just change the checker script and let the check pass. '
        'line=${match.group(0)}',
      );
    }
  }
}

Future<void> testDartSanitizer(TestDartSanitizerConfig config) async =>
    await dart_sanitizer_tester.run(config);

Future<void> testFlutterNative(TestFlutterConfig config) async {
  await _runFlutterDoctor();
  await runPubGetIfNotRunYet(config.package);

  await exec(
      'flutter test integration_test/simple_test.dart --verbose ${config.flutterTestArgs ?? ""}',
      relativePwd: config.package);
}

Future<void> testFlutterWeb(TestDartConfig config) async {
  await _runFlutterDoctor();
  await runPubGetIfNotRunYet(config.package);

  await executeFrbCodegen(cmd: 'build-web', relativePwd: config.package);

  await exec(
      'flutter drive '
      '--driver=test_driver/integration_test.dart '
      '--target=integration_test/simple_test.dart '
      '-d web-server '
      '--verbose',
      relativePwd: config.package);
}

Future<void> _runFlutterDoctor() async => await exec('flutter doctor -v');

const kEnvEnableRustBacktrace = {'RUST_BACKTRACE': '1'};
