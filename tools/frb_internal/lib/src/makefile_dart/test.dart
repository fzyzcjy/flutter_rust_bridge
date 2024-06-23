// ignore_for_file: avoid_print

import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/post_release.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester.dart'
    as dart_sanitizer_tester;
import 'package:flutter_rust_bridge_internal/src/utils/codecov_transformer.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:meta/meta.dart';
import 'package:retry/retry.dart';
import 'package:toml/toml.dart';
import 'package:yaml/yaml.dart';

part 'test.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('test-mimic-quickstart', testMimicQuickstart),
    SimpleCommand('test-upgrade', testUpgrade),
    SimpleConfigCommand('test-rust', testRust, _$populateTestRustConfigParser,
        _$parseTestRustConfigResult),
    SimpleConfigCommand(
        'test-rust-package',
        testRustPackage,
        _$populateTestRustPackageConfigParser,
        _$parseTestRustPackageConfigResult),
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
class TestRustPackageConfig {
  final String package;
  final bool updateGoldens;
  final bool coverage;

  const TestRustPackageConfig({
    required this.package,
    required this.updateGoldens,
    required this.coverage,
  });
}

@CliOptions()
class TestDartConfig {
  @CliOption(convert: convertConfigPackage)
  final String package;

  const TestDartConfig({required this.package});
}

@CliOptions()
class TestDartNativeConfig {
  @CliOption(convert: convertConfigPackage)
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
  @CliOption(convert: convertConfigPackage)
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
  @CliOption(convert: convertConfigPackage)
  final String package;

  const TestFlutterConfig({this.flutterTestArgs, required this.package});
}

@CliOptions()
class TestFlutterWebConfig {
  @CliOption(convert: convertConfigPackage)
  final String package;
  final bool coverage;

  const TestFlutterWebConfig({required this.package, required this.coverage});
}

Future<void> testMimicQuickstart() async =>
    await const MimicQuickstartTester(postRelease: false).test();

class MimicQuickstartTester {
  final bool postRelease;
  final bool coverage;

  const MimicQuickstartTester({
    required this.postRelease,
    this.coverage = false,
  });

  Future<void> test() async {
    _prepareDir();

    await _quickstartStepCreate();

    // Since we will run after modify and generate, there is no need to run (again) here.
    // await _quickstartStepRun();

    await _quickstartStepModify();

    await _quickstartStepGenerate();

    // Run again after modification
    await _quickstartStepRun();
  }

  static void _prepareDir() {
    Directory('${exec.pwd}frb_example').createSync(recursive: true);
    final targetDir = Directory(_baseDir);
    if (targetDir.existsSync()) targetDir.deleteSync(recursive: true);
  }

  static const _kMimicQuickstartPackageName = 'my_app';

  static String get _baseDir =>
      '${exec.pwd}frb_example/$_kMimicQuickstartPackageName/';

  Future<void> _quickstartStepCreate() async {
    await executeFrbCodegen(
      'create $_kMimicQuickstartPackageName ${postRelease ? "" : "--local"}',
      relativePwd: 'frb_example',
      coverage: coverage,
      postRelease: postRelease,
      coverageName: 'MimicQuickstartStepCreate',
    );

    // avoid workspace issue (only exist in our setup, not in real user's)
    simpleReplaceFile(
      '${exec.pwd}frb_example/$_kMimicQuickstartPackageName/rust/Cargo.toml',
      '[lib]',
      '[workspace]\n\n[lib]',
    );
  }

  Future<void> _quickstartStepRun() async {
    final String deviceId;
    if (Platform.isWindows) {
      deviceId = 'windows';
    } else if (Platform.isMacOS) {
      deviceId = 'macos';
    } else if (Platform.isLinux) {
      deviceId = 'linux';
    } else {
      throw UnimplementedError();
    }

    await flutterIntegrationTestRaw(
      flutterTestArgs: '--device-id $deviceId',
      relativePwd: 'frb_example/$_kMimicQuickstartPackageName',
    );
  }

  Future<void> _quickstartStepModify() async {
    const kExtraRustSrc =
        '''pub fn hello(a: String) -> String { a.repeat(2) }''';
    const kExtraDartTestPrelude = '''
    import 'package:$_kMimicQuickstartPackageName/src/rust/api/simple.dart';
    ''';
    const kExtraDartTestBody = '''
  testWidgets('Can call the new function', (WidgetTester tester) async {
    var result = await hello(a: "Hi");
    expect(result, 'HiHi');
  });
  ''';

    final pathRustSrc =
        '${exec.pwd}frb_example/$_kMimicQuickstartPackageName/rust/src/api/simple.rs';
    final pathDartTest =
        '${exec.pwd}frb_example/$_kMimicQuickstartPackageName/integration_test/simple_test.dart';

    simpleActFile(pathRustSrc, (x) => x + kExtraRustSrc);
    simpleActFile(pathDartTest, (x) => kExtraDartTestPrelude + x);
    simpleReplaceFile(
        pathDartTest, 'testWidgets(', '$kExtraDartTestBody\ntestWidgets(');

    for (final path in [pathRustSrc, pathDartTest]) {
      print('path=$path content=${File(path).readAsStringSync()}');
    }
  }

  Future<void> _quickstartStepGenerate() async {
    await executeFrbCodegen(
      'generate',
      relativePwd: 'frb_example/$_kMimicQuickstartPackageName',
      coverage: coverage,
      coverageName: 'MimicQuickstartStepGenerate',
      postRelease: postRelease,
    );
  }
}

Future<void> testUpgrade() async {
  void checkVersion({required String expectVersion}) {
    print('checkVersion expectVersion=$expectVersion');

    final baseDir = MimicQuickstartTester._baseDir;

    final pubspecYaml =
        loadYaml(File('${baseDir}pubspec.yaml').readAsStringSync());
    final dartVersion = pubspecYaml['dependencies']['flutter_rust_bridge'];
    if (dartVersion != expectVersion) {
      throw Exception(
          'checkVersion failed. dartVersion=$dartVersion expectVersion=$expectVersion');
    }

    final cargoToml =
        TomlDocument.parse(File('${baseDir}rust/Cargo.toml').readAsStringSync())
            .toMap();
    final rustVersion = cargoToml['dependencies']['flutter_rust_bridge'];
    if (rustVersion != '=$expectVersion') {
      throw Exception('rustVersion=$rustVersion expectVersion=$expectVersion');
    }
  }

  // This old-version can be bumped if needed
  const oldVersion = '2.0.0-dev.20';
  final newVersion = getFrbDartVersion();
  if (oldVersion == newVersion) {
    throw Exception('This test requires oldVersion!=newVersion');
  }

  await quickstartStepInstall(CodegenInstallMode.cargoInstall,
      versionConstraint: oldVersion);

  MimicQuickstartTester._prepareDir();

  await const MimicQuickstartTester(postRelease: true)._quickstartStepCreate();
  checkVersion(expectVersion: oldVersion);

  await const MimicQuickstartTester(postRelease: false, coverage: true)
      ._quickstartStepGenerate();
  checkVersion(expectVersion: newVersion);
}

Future<void> testRust(TestRustConfig config) async {
  for (final package in kRustPackages) {
    await testRustPackage(TestRustPackageConfig(
      package: package,
      updateGoldens: config.updateGoldens,
      coverage: config.coverage,
    ));
  }
}

Future<void> testRustPackage(TestRustPackageConfig config) async {
  await runPubGetIfNotRunYet('frb_example/dart_minimal');
  await runPubGetIfNotRunYet('frb_example/pure_dart');

  await exec('cargo build', relativePwd: config.package);

  final effectiveEnableCoverage = config.coverage &&
      const ['frb_codegen', 'frb_rust'].contains(config.package);

  final outputCodecovPath =
      '${getCoverageDir('test_rust_package_${config.package.replaceAll("/", "_")}')}/codecov.json';
  await exec(
      'cargo ${effectiveEnableCoverage ? "llvm-cov --codecov --output-path $outputCodecovPath" : "test"}',
      relativePwd: config.package,
      extraEnv: {
        'FRB_SKIP_GENERATE_FRB_EXAMPLE_TEST': '1',
        if (config.updateGoldens) 'UPDATE_GOLDENS': '1',
        ...kEnvEnableRustBacktrace,
      });

  if (effectiveEnableCoverage) transformCodecovReport(outputCodecovPath);
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
    reportPath: '${getCoverageDir('rust')}/codecov.json',
    (rustEnvMap) async {
      await runPubGetIfNotRunYet(config.package);

      final dartMode = kDartModeOfPackage[config.package]!;

      var extraFlags = '';
      if (dartMode == DartMode.dart) {
        extraFlags += '--enable-experiment=native-assets ';
      }
      if (const {
        'frb_example/pure_dart',
        'frb_example/pure_dart_pde',
      }.contains(config.package)) {
        extraFlags += '--enable-vm-service ';
      }

      // extra check for e.g. #1807
      await wrapMaybeSetExitIfChangedRaw(true, () async {
        await exec(
          '${dartMode.name} $extraFlags test ${config.coverage ? ' --coverage="coverage"' : ""}',
          relativePwd: config.package,
          extraEnv: {
            // Deliberately do not provide backtrace env to see whether the test_utils work
            // ...kEnvEnableRustBacktrace,
            ...rustEnvMap,
          },
        );
      });
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
      'cargo llvm-cov report --codecov '
      '--output-path $reportPath '
      "--ignore-filename-regex '.*/frb_example/.*' "
      '$cargoLlvmCovCommonArgs',
      relativePwd: relativeRustPwd,
      extraEnv: envMap);
  transformCodecovReport(reportPath);

  return ans;
}

// ref: https://github.com/rrousselGit/riverpod/blob/67d26d2a47a7351d6676012c44eb53dd6ff79787/scripts/coverage.sh#L10
// ref: https://github.com/mobxjs/mobx.dart/blob/52515a1d82f15a2b2eb48822d030647e217134cc/tool/coverage.sh#L12
Future<void> _formatDartCoverage({required String package}) async {
  await _installDartCoverage();

  final reportOn = '${exec.pwd}/frb_dart';
  await exec(
    'format_coverage --check-ignore --lcov --in=coverage --out=${getCoverageDir('dart')}/lcov.info --packages=.dart_tool/package_config.json --report-on=$reportOn',
    relativePwd: package,
  );
}

Future<void> _installDartCoverage() async {
  await exec('dart pub global activate coverage');
}

String getCoverageDir(String lang) {
  final ans = '${exec.pwd}target/coverage/$lang';
  Directory(ans).createSync(recursive: true);
  return ans;
}

Future<void> testDartWeb(TestDartConfig config) async {
  await runPubGetIfNotRunYet(config.package);

  final package = config.package;
  if (package == 'frb_dart') {
    await exec(
      'dart test -p chrome',
      relativePwd: package,
      // extraEnv: kEnvEnableRustBacktrace,
    );
  } else {
    await exec(
      'dart run flutter_rust_bridge_utils test-web --entrypoint ../$package/test/dart_web_test_entrypoint.dart',
      relativePwd: 'frb_utils',
      // extraEnv: kEnvEnableRustBacktrace,
    );
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

  await flutterIntegrationTestRaw(
      relativePwd: config.package,
      flutterTestArgs: config.flutterTestArgs ?? '');
}

Future<void> flutterIntegrationTestRaw({
  String flutterTestArgs = '',
  required String relativePwd,
}) async {
  await retry(
    () async => await exec(
        'flutter test integration_test/simple_test.dart --verbose --reporter=expanded $flutterTestArgs',
        relativePwd: relativePwd),
    maxAttempts: 3,
    onRetry: (e) => print(
        'ERROR when executing flutterIntegrationTestRaw, thus retry (exception=$e)'),
  );
}

Future<void> testFlutterWeb(TestFlutterWebConfig config) async {
  await _runFlutterDoctor();
  await runPubGetIfNotRunYet(config.package);
  await _installDartCoverage();

  final buildWebPackage =
      kBuildWebPackageReplacer[config.package] ?? config.package;
  await executeFrbCodegen(
    'build-web --dart-coverage',
    relativePwd: buildWebPackage,
    coverage: config.coverage,
    coverageName: 'TestFlutterWeb',
  );

  await exec(
      'flutter drive '
      '--driver=test_driver/integration_test.dart '
      '--target=integration_test/simple_test.dart '
      '-d web-server '
      '--verbose',
      relativePwd: config.package);

  if (config.coverage) {
    await _formatDartCoverage(package: config.package);
  }
}

Future<void> _runFlutterDoctor() async => await exec('flutter doctor -v');

const kEnvEnableRustBacktrace = {'RUST_BACKTRACE': 'full'};
