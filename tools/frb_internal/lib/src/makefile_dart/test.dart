// ignore_for_file: avoid_print

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
    SimpleConfigCommand(
        'test-dart-native',
        testDartNative,
        _$populateTestDartNativeConfigParser,
        _$parseTestDartNativeConfigResult),
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
    SimpleConfigCommand(
        'test-flutter-web',
        testFlutterWeb,
        _$populateTestFlutterWebConfigParser,
        _$parseTestFlutterWebConfigResult),
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

  const TestDartConfig({required this.package});
}

@CliOptions()
class TestDartNativeConfig {
  final String package;
  final bool coverage;

  const TestDartNativeConfig({required this.package, required this.coverage});
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

@CliOptions()
class TestFlutterWebConfig {
  final String package;
  final bool coverage;

  const TestFlutterWebConfig({required this.package, required this.coverage});
}

Future<void> testRust(TestRustConfig config) async {
  for (final package in kRustPackages) {
    await testRustPackage(config, package);
  }
}

Future<void> testRustPackage(TestRustConfig config, String package) async {
  await runPubGetIfNotRunYet('frb_example/dart_minimal');
  await runPubGetIfNotRunYet('frb_example/pure_dart');

  await exec('cargo build', relativePwd: package);

  final effectiveEnableCoverage = config.coverage && package == 'frb_codegen';

  await exec(
      'cargo ${effectiveEnableCoverage ? "llvm-cov --lcov --output-path lcov.info" : "test"}',
      relativePwd: package,
      extraEnv: {
        // If we are doing codecov, then we need to enable all tests;
        // otherwise, we can rely on other CIs to check code generation
        // and skip this heavy test.
        if (!effectiveEnableCoverage) 'FRB_SKIP_GENERATE_FRB_EXAMPLE_TEST': '1',
        if (config.updateGoldens) 'UPDATE_GOLDENS': '1',
        ...kEnvEnableRustBacktrace,
      });
}

Future<void> testDartNative(TestDartNativeConfig config) async {
  final enableRustCoverage = config.coverage &&
      !const [
        'frb_dart',
        'frb_utils',
        'tools/frb_internal',
      ].contains(config.package);

  await withLlvmCovReport(
    relativeRustPwd: '${config.package}/rust',
    enable: enableRustCoverage,
    reportPath: '${config.package}/coverage/rust_lcov.info',
    (rustEnvMap) async {
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
        '${dartMode.name} $extraFlags test ${config.coverage ? ' --coverage="coverage"' : ""}',
        relativePwd: config.package,
        extraEnv: {
          ...kEnvEnableRustBacktrace,
          ...rustEnvMap,
        },
      );
    },
  );

  if (config.coverage) {
    await _formatDartCoverage(package: config.package);
  }
}

// Follow steps in https://github.com/taiki-e/cargo-llvm-cov#get-coverage-of-external-tests
Future<T> withLlvmCovReport<T>(
  Future<T> Function(Map<String, String> envMap) inner, {
  required bool enable,
  required String relativeRustPwd,
  required String reportPath,
}) async {
  if (!enable) {
    return await inner({});
  }

  // `--release`, since our dart tests by default build rust release libs
  const cargoLlvmCovCommonArgs = '--release';

  final rawEnvs = (await exec('cargo llvm-cov show-env $cargoLlvmCovCommonArgs',
          relativePwd: relativeRustPwd))
      .stdout;
  final envMap = Map.fromEntries(rawEnvs.trim().split('\n').map((line) {
    final m = RegExp(r"""^(\w+)=['"]?(.+?)['"]?$""").firstMatch(line)!;
    return MapEntry(m.group(1)!, m.group(2)!);
  }));
  print('envMap=$envMap');

  await exec('cargo llvm-cov clean --workspace $cargoLlvmCovCommonArgs',
      relativePwd: relativeRustPwd, extraEnv: envMap);

  final ans = await inner(envMap);

  await exec(
      'cargo llvm-cov report --lcov '
      '--output-path ${exec.pwd}/$reportPath '
      "--ignore-filename-regex '.*/frb_example/.*' "
      '$cargoLlvmCovCommonArgs',
      relativePwd: relativeRustPwd,
      extraEnv: envMap);

  return ans;
}

// ref: https://github.com/rrousselGit/riverpod/blob/67d26d2a47a7351d6676012c44eb53dd6ff79787/scripts/coverage.sh#L10
// ref: https://github.com/mobxjs/mobx.dart/blob/52515a1d82f15a2b2eb48822d030647e217134cc/tool/coverage.sh#L12
Future<void> _formatDartCoverage({required String package}) async {
  await exec('dart pub global activate coverage');

  final reportOn = '${exec.pwd}/frb_dart';
  await exec(
    'format_coverage --lcov --in=coverage --out=coverage/lcov.txt --packages=.dart_tool/package_config.json --report-on=$reportOn',
    relativePwd: package,
  );
}

Future<void> testDartWeb(TestDartConfig config) async {
  await runPubGetIfNotRunYet(config.package);

  final package = config.package;
  if (package == 'frb_dart') {
    await exec('dart test -p chrome',
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
      'flutter test integration_test/simple_test.dart --verbose --reporter=expanded ${config.flutterTestArgs ?? ""}',
      relativePwd: config.package);
}

Future<void> testFlutterWeb(TestFlutterWebConfig config) async {
  await _runFlutterDoctor();
  await runPubGetIfNotRunYet(config.package);

  await executeFrbCodegen('build-web',
      relativePwd: config.package, coverage: config.coverage);

  await exec(
      'flutter drive '
      '--driver=test_driver/integration_test.dart '
      '--target=integration_test/simple_test.dart '
      '-d web-server '
      '--verbose',
      relativePwd: config.package);
}

Future<void> _runFlutterDoctor() async => await exec('flutter doctor -v');

const kEnvEnableRustBacktrace = {'RUST_BACKTRACE': 'full'};
