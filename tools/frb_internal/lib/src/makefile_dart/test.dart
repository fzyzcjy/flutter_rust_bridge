// ignore_for_file: avoid_print

import 'dart:async';
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
    SimpleConfigCommand(
      'test-rust',
      testRust,
      _$populateTestRustConfigParser,
      _$parseTestRustConfigResult,
    ),
    SimpleConfigCommand(
      'test-rust-package',
      testRustPackage,
      _$populateTestRustPackageConfigParser,
      _$parseTestRustPackageConfigResult,
    ),
    SimpleConfigCommand(
      'test-dart-native',
      testDartNative,
      _$populateTestDartNativeConfigParser,
      _$parseTestDartNativeConfigResult,
    ),
    SimpleConfigCommand(
      'test-dart-web',
      testDartWeb,
      _$populateTestDartConfigParser,
      _$parseTestDartConfigResult,
    ),
    SimpleConfigCommand(
      'test-dart-valgrind',
      testDartValgrind,
      _$populateTestDartConfigParser,
      _$parseTestDartConfigResult,
    ),
    SimpleConfigCommand(
      'test-dart-sanitizer',
      testDartSanitizer,
      _$populateTestDartSanitizerConfigParser,
      _$parseTestDartSanitizerConfigResult,
    ),
    SimpleConfigCommand(
      'test-flutter-native',
      testFlutterNative,
      _$populateTestFlutterConfigParser,
      _$parseTestFlutterConfigResult,
    ),
    SimpleConfigCommand(
      'test-flutter-web',
      testFlutterWeb,
      _$populateTestFlutterWebConfigParser,
      _$parseTestFlutterWebConfigResult,
    ),
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
  @CliOption(defaultsTo: false)
  final bool checkClean;

  const TestDartNativeConfig({
    required this.package,
    required this.coverage,
    required this.checkClean,
  });
}

enum Sanitizer { asan, msan, lsan, tsan }

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
    const kExtraDartTestPrelude =
        '''
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
      pathDartTest,
      'testWidgets(',
      '$kExtraDartTestBody\ntestWidgets(',
    );

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

    final pubspecYaml = loadYaml(
      File('${baseDir}pubspec.yaml').readAsStringSync(),
    );
    final dartVersion = pubspecYaml['dependencies']['flutter_rust_bridge'];
    if (dartVersion != expectVersion) {
      throw Exception(
        'checkVersion failed. dartVersion=$dartVersion expectVersion=$expectVersion',
      );
    }

    final cargoToml = TomlDocument.parse(
      File('${baseDir}rust/Cargo.toml').readAsStringSync(),
    ).toMap();
    final rustVersion = cargoToml['dependencies']['flutter_rust_bridge'];
    if (rustVersion != '=$expectVersion') {
      throw Exception('rustVersion=$rustVersion expectVersion=$expectVersion');
    }
  }

  // This old-version can be bumped if needed
  const oldVersion = '2.11.0';
  final newVersion = getFrbDartVersion();
  if (oldVersion == newVersion) {
    throw Exception('This test requires oldVersion!=newVersion');
  }

  await quickstartStepInstall(
    CodegenInstallMode.cargoInstall,
    versionConstraint: oldVersion,
  );

  MimicQuickstartTester._prepareDir();

  await const MimicQuickstartTester(postRelease: true)._quickstartStepCreate();
  checkVersion(expectVersion: oldVersion);

  await const MimicQuickstartTester(
    postRelease: false,
    coverage: true,
  )._quickstartStepGenerate();
  checkVersion(expectVersion: newVersion);
}

Future<void> testRust(TestRustConfig config) async {
  for (final package in kRustPackages) {
    await testRustPackage(
      TestRustPackageConfig(
        package: package,
        updateGoldens: config.updateGoldens,
        coverage: config.coverage,
      ),
    );
  }
}

Future<void> testRustPackage(TestRustPackageConfig config) async {
  await runPubGetIfNotRunYet('frb_example/dart_minimal');
  await runPubGetIfNotRunYet('frb_example/pure_dart');
  print("testRustPackage ${config.package}");
  final feature = getRustFeaturesOfPackage(config.package);
  await exec(
    'cargo build ${feature != null ? "--features $feature" : ""}',
    relativePwd: config.package,
  );

  final effectiveEnableCoverage =
      config.coverage &&
      const ['frb_codegen', 'frb_rust'].contains(config.package);

  final outputCodecovPath =
      '${getCoverageDir('test_rust_package_${config.package.replaceAll("/", "_")}')}/codecov.json';
  await exec(
    'cargo ${effectiveEnableCoverage ? "llvm-cov --codecov --output-path $outputCodecovPath" : "test"} ${feature != null ? "--features $feature" : ""}',
    relativePwd: config.package,
    extraEnv: {
      'FRB_SKIP_GENERATE_FRB_EXAMPLE_TEST': '1',
      if (config.updateGoldens) 'UPDATE_GOLDENS': '1',
      ...kEnvEnableRustBacktrace,
    },
  );

  if (effectiveEnableCoverage) transformCodecovReport(outputCodecovPath);
}

Future<void> testDartNative(TestDartNativeConfig config) async {
  final enableRustCoverage =
      config.coverage &&
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
      // native-assets experiment was stabilized in Dart 3.10, so we no longer need the flag
      // VM service is automatically enabled in Dart 3.10+ when running tests

      // extra check for e.g. #1807
      await wrapMaybeSetExitIfChangedRaw(config.checkClean, () async {
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

  final rawEnvs = (await exec(
    'cargo llvm-cov show-env $cargoLlvmCovCommonArgs',
    relativePwd: relativeRustPwd,
  )).stdout;
  final envMap = Map.fromEntries(
    rawEnvs.trim().split('\n').map((line) {
      final m = RegExp(r"""^(\w+)=['"]?(.+?)['"]?$""").firstMatch(line)!;
      return MapEntry(m.group(1)!, m.group(2)!);
    }),
  );
  // For Dart 3.10+ native-assets hooks, pass RUSTFLAGS via NIX_FRB_RUSTFLAGS
  // because hooks run in a semi-hermetic environment
  if (envMap.containsKey('RUSTFLAGS')) {
    envMap['NIX_FRB_RUSTFLAGS'] = envMap['RUSTFLAGS']!;
  }
  print('envMap=$envMap');

  await exec(
    'cargo llvm-cov clean --workspace $cargoLlvmCovCommonArgs',
    relativePwd: relativeRustPwd,
    extraEnv: envMap,
  );

  final ans = await inner(envMap);

  await exec(
    'cargo llvm-cov report --codecov '
    '--output-path $reportPath '
    "--ignore-filename-regex '.*/frb_example/.*' "
    '$cargoLlvmCovCommonArgs',
    relativePwd: relativeRustPwd,
    extraEnv: envMap,
  );
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
    final features = getRustFeaturesOfPackage(config.package);
    await exec(
      'dart run flutter_rust_bridge_utils test-web --entrypoint ../$package/test/dart_web_test_entrypoint.dart ${features != null ? "--rust-features $features" : ""}',
      relativePwd: 'frb_utils',
      // extraEnv: kEnvEnableRustBacktrace,
    );
  }
}

/// ref https://github.com/dart-lang/sdk/blob/master/runtime/tools/valgrind.py
Future<void> testDartValgrind(TestDartConfig config) async {
  await exec('sudo apt install -y valgrind');
  await runPubGetIfNotRunYet(config.package);
  await exec(_dartValgrindCompileCommand(), relativePwd: config.package);

  const valgrindCommand =
      'valgrind '
      '--error-exitcode=1 '
      '--leak-check=full '
      '--trace-children=yes '
      // Used for implicit null checks.
      '--ignore-ranges=0x000-0xFFF '
      // Valgrind crashes with the default level (2).
      '--vex-iropt-level=1';

  final output = await exec(
    '$valgrindCommand ${_dartValgrindOutputExecutablePath()}',
    relativePwd: config.package,
    checkExitCode: false,
    extraEnv: kEnvEnableRustBacktrace,
  );

  checkValgrindOutput(output.stdout);
}

String _dartValgrindCompileCommand() {
  return 'dart build cli '
      '-t test/dart_valgrind_test_entrypoint.dart '
      '-o ${_dartValgrindOutputDirectory()}';
}

@visibleForTesting
String dartValgrindCompileCommandForTesting() => _dartValgrindCompileCommand();

String _dartValgrindOutputDirectory() => 'build/valgrind_test_output/';

@visibleForTesting
String dartValgrindOutputDirectoryForTesting() =>
    _dartValgrindOutputDirectory();

String _dartValgrindOutputExecutablePath() {
  return 'build/valgrind_test_output/bundle/bin/dart_valgrind_test_entrypoint';
}

@visibleForTesting
String dartValgrindOutputExecutablePathForTesting() =>
    _dartValgrindOutputExecutablePath();

@visibleForTesting
void checkValgrindOutput(String output) {
  const kDartAllTestsPassedStr = 'All tests passed!';
  if (!output.contains(kDartAllTestsPassedStr)) {
    throw Exception(
      'valgrind_util does not find "$kDartAllTestsPassedStr", thus dart test seems failed',
    );
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
    flutterTestArgs: config.flutterTestArgs ?? '',
  );

  if (config.package == 'frb_example/flutter_via_create') {
    await _runFlutterViaCreateQuickstartSmokeTest(
      package: config.package,
      target: quickstartSmokeNativeTargetForTesting(
        flutterTestArgs: config.flutterTestArgs,
      ),
      deviceId: quickstartSmokeDeviceIdForTesting(
        flutterTestArgs: config.flutterTestArgs,
      ),
    );
  }
}

Future<void> flutterIntegrationTestRaw({
  String flutterTestArgs = '',
  required String relativePwd,
}) async {
  const timeout = Duration(minutes: 20);
  await retry(
    () async => await exec(
      'flutter test integration_test/simple_test.dart --verbose --reporter=expanded $flutterTestArgs',
      relativePwd: relativePwd,
      timeout: timeout,
    ),
    maxAttempts: 3,
    onRetry: (e) => print(
      'ERROR when executing flutterIntegrationTestRaw, thus retry (exception=$e)',
    ),
  );
}

Future<void> testFlutterWeb(TestFlutterWebConfig config) async {
  await _runFlutterDoctor();
  await runPubGetIfNotRunYet(config.package);
  await _installDartCoverage();

  final buildWebPackage = resolveBuildWebPackage(config.package);
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
    relativePwd: config.package,
  );

  if (config.package == 'frb_example/flutter_via_create') {
    await _runFlutterViaCreateQuickstartSmokeTest(
      package: config.package,
      target: QuickstartSmokeTarget.web,
      deviceId: 'chrome',
    );
  }

  if (config.coverage) {
    await _formatDartCoverage(package: buildWebPackage);
  }
}

@visibleForTesting
String resolveBuildWebPackage(String package) =>
    kBuildWebPackageReplacer[package] ?? package;

enum QuickstartSmokeTarget { web, desktop, android, ios }

@visibleForTesting
QuickstartSmokeTarget quickstartSmokeNativeTargetForTesting({
  required String? flutterTestArgs,
}) {
  final deviceId = quickstartSmokeDeviceIdForTesting(
    flutterTestArgs: flutterTestArgs,
  );
  if (deviceId == 'emulator-5554') return QuickstartSmokeTarget.android;
  if (Platform.isMacOS && deviceId != null && deviceId != 'macos') {
    return QuickstartSmokeTarget.ios;
  }
  return QuickstartSmokeTarget.desktop;
}

@visibleForTesting
String? quickstartSmokeDeviceIdForTesting({required String? flutterTestArgs}) {
  final args = flutterTestArgs ?? '';
  final match = RegExp(r'--device-id\s+([^\s]+)').firstMatch(args);
  return match?.group(1);
}

Future<void> _runFlutterViaCreateQuickstartSmokeTest({
  required String package,
  required QuickstartSmokeTarget target,
  required String? deviceId,
}) async {
  const artifactDir = 'target/quickstart_smoke';
  final targetName = target.name;
  final screenshotPath = '$artifactDir/quickstart-$targetName.png';
  final preprocessedScreenshotPath =
      '$artifactDir/quickstart-$targetName-processed.png';
  final ocrOutputPath = '$artifactDir/quickstart-$targetName.txt';
  final logPath = '$artifactDir/quickstart-$targetName.log';
  final absolutePackage = Directory(package).absolute;
  final absoluteArtifactDir = Directory('${absolutePackage.path}/$artifactDir');
  absoluteArtifactDir.createSync(recursive: true);
  for (final relativePath in [
    screenshotPath,
    preprocessedScreenshotPath,
    ocrOutputPath,
    logPath,
  ]) {
    final file = File('${absolutePackage.path}/$relativePath');
    if (file.existsSync()) file.deleteSync();
  }

  final logFile = File('${absolutePackage.path}/$logPath');
  final logSink = logFile.openWrite();
  final flutterRunArgs = [
    'run',
    '-d',
    deviceId ?? quickstartSmokeDefaultDeviceIdForTesting(target),
    if (target == QuickstartSmokeTarget.web) ...[
      '--web-header=Cross-Origin-Opener-Policy=same-origin',
      '--web-header=Cross-Origin-Embedder-Policy=require-corp',
    ],
  ];
  final environment = <String, String>{
    if (Platform.isLinux) 'DISPLAY': Platform.environment['DISPLAY'] ?? ':99',
  };
  final chromeWrapper = await _createChromeWrapperIfNeeded(
    absoluteArtifactDir.path,
  );
  if (chromeWrapper != null) {
    environment['CHROME_EXECUTABLE'] = chromeWrapper.path;
  }

  final process = await Process.start(
    'flutter',
    flutterRunArgs,
    workingDirectory: absolutePackage.path,
    environment: environment,
    runInShell: Platform.isWindows,
  );
  int? observedExitCode;
  final exitCodeFuture = process.exitCode;
  unawaited(exitCodeFuture.then((exitCode) => observedExitCode = exitCode));
  final outputBuffer = StringBuffer();
  process.stdout.transform(systemEncoding.decoder).listen((text) {
    outputBuffer.write(text);
    logSink.write(text);
  });
  process.stderr.transform(systemEncoding.decoder).listen((text) {
    outputBuffer.write(text);
    logSink.write(text);
  });

  final deadline = DateTime.now().add(const Duration(seconds: 75));
  const failurePatterns = [
    'DataCloneError',
    'Failed to execute \'postMessage\' on \'Worker\'',
    'fail to create WorkerPool',
    'Failed to initialize',
    'WebAssembly.instantiate',
  ];
  while (DateTime.now().isBefore(deadline)) {
    await Future<void>.delayed(const Duration(seconds: 1));
    final output = outputBuffer.toString();
    if (failurePatterns.any(output.contains)) break;
    if (observedExitCode != null) break;
  }

  final screenshotFile = File('${absolutePackage.path}/$screenshotPath');
  await _captureQuickstartSmokeScreenshot(
    target: target,
    screenshotFile: screenshotFile,
  );
  final preprocessedScreenshotFile = File(
    '${absolutePackage.path}/$preprocessedScreenshotPath',
  );
  await _preprocessQuickstartSmokeScreenshot(
    screenshotFile: screenshotFile,
    preprocessedScreenshotFile: preprocessedScreenshotFile,
  );
  final ocrOutputFile = File('${absolutePackage.path}/$ocrOutputPath');
  await _runQuickstartSmokeOcr(
    preprocessedScreenshotFile: preprocessedScreenshotFile,
    ocrOutputFile: ocrOutputFile,
  );

  final killedBySmoke = observedExitCode == null;
  if (killedBySmoke) {
    process.kill();
  }
  final exitCode = await exitCodeFuture;
  await logSink.close();
  if (chromeWrapper != null && chromeWrapper.existsSync()) {
    chromeWrapper.deleteSync();
  }

  final combinedOutput = outputBuffer.toString();
  print('\n===== flutter run log ($targetName) =====');
  print(logFile.readAsStringSync());
  if (ocrOutputFile.existsSync()) {
    print('\n===== screenshot OCR ($targetName) =====');
    print(ocrOutputFile.readAsStringSync());
  }

  for (final pattern in failurePatterns) {
    if (combinedOutput.contains(pattern)) {
      throw Exception(
        'flutter_via_create $targetName quickstart smoke test failed '
        'with `$pattern`',
      );
    }
  }

  if (!screenshotFile.existsSync() || screenshotFile.lengthSync() == 0) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test failed to capture '
      'a screenshot at `$screenshotPath`',
    );
  }

  if (!ocrOutputFile.existsSync()) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test did not produce OCR '
      'output at `$ocrOutputPath`',
    );
  }
  checkQuickstartSmokeOcrTextForTesting(ocrOutputFile.readAsStringSync());

  if (!killedBySmoke && exitCode != 0) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test failed with '
      'unexpected exit code $exitCode',
    );
  }
}

@visibleForTesting
String quickstartSmokeDefaultDeviceIdForTesting(QuickstartSmokeTarget target) {
  if (target == QuickstartSmokeTarget.web) return 'chrome';
  if (target == QuickstartSmokeTarget.android) return 'emulator-5554';
  if (target == QuickstartSmokeTarget.desktop) {
    if (Platform.isWindows) return 'windows';
    if (Platform.isMacOS) return 'macos';
    if (Platform.isLinux) return 'linux';
  }
  throw Exception('No default quickstart smoke device for `$target`');
}

Future<File?> _createChromeWrapperIfNeeded(String artifactDir) async {
  if (!Platform.isLinux || Platform.environment['USER'] != 'root') {
    return null;
  }
  final file = File('$artifactDir/chrome-no-sandbox-wrapper');
  await file.writeAsString(
    '#!/bin/sh\nexec google-chrome --no-sandbox "\$@"\n',
  );
  await Process.run('chmod', ['+x', file.path]);
  return file;
}

Future<void> _captureQuickstartSmokeScreenshot({
  required QuickstartSmokeTarget target,
  required File screenshotFile,
}) async {
  final result = switch (target) {
    QuickstartSmokeTarget.android => await Process.run(
      'adb',
      ['exec-out', 'screencap', '-p'],
      stdoutEncoding: null,
      stderrEncoding: systemEncoding,
    ),
    QuickstartSmokeTarget.ios => await Process.run('xcrun', [
      'simctl',
      'io',
      'booted',
      'screenshot',
      screenshotFile.path,
    ], stderrEncoding: systemEncoding),
    _ when Platform.isMacOS => await Process.run('screencapture', [
      '-x',
      screenshotFile.path,
    ], stderrEncoding: systemEncoding),
    _ when Platform.isWindows => await Process.run('powershell', [
      '-NoProfile',
      '-Command',
      r'''
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$bitmap = New-Object System.Drawing.Bitmap $bounds.Width, $bounds.Height
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $bounds.Size)
$bitmap.Save($args[0], [System.Drawing.Imaging.ImageFormat]::Png)
$graphics.Dispose()
$bitmap.Dispose()
''',
      screenshotFile.path,
    ], stderrEncoding: systemEncoding),
    _ => await Process.run('import', [
      '-window',
      'root',
      screenshotFile.path,
    ], stderrEncoding: systemEncoding),
  };
  if (target == QuickstartSmokeTarget.android && result.exitCode == 0) {
    screenshotFile.writeAsBytesSync(result.stdout as List<int>);
  }
  if (result.exitCode != 0) {
    throw Exception(
      'Failed to capture quickstart screenshot (exitCode=${result.exitCode}, '
      'stderr=${result.stderr})',
    );
  }
}

Future<void> _preprocessQuickstartSmokeScreenshot({
  required File screenshotFile,
  required File preprocessedScreenshotFile,
}) async {
  final result = await Process.run(Platform.isWindows ? 'magick' : 'convert', [
    screenshotFile.path,
    '-resize',
    '300%',
    '-colorspace',
    'Gray',
    '-normalize',
    preprocessedScreenshotFile.path,
  ], stderrEncoding: systemEncoding);
  if (result.exitCode != 0 || !preprocessedScreenshotFile.existsSync()) {
    screenshotFile.copySync(preprocessedScreenshotFile.path);
  }
}

Future<void> _runQuickstartSmokeOcr({
  required File preprocessedScreenshotFile,
  required File ocrOutputFile,
}) async {
  final outputBase = ocrOutputFile.path.replaceAll(RegExp(r'\.txt$'), '');
  final result = await Process.run('tesseract', [
    preprocessedScreenshotFile.path,
    outputBase,
  ], stderrEncoding: systemEncoding);
  if (result.exitCode != 0) {
    throw Exception(
      'Failed to OCR quickstart screenshot (exitCode=${result.exitCode}, '
      'stderr=${result.stderr})',
    );
  }
}

@visibleForTesting
String normalizeQuickstartSmokeOcrTextForTesting(String text) => text
    .toLowerCase()
    .replaceAll(RegExp(r'[^a-z0-9]+'), ' ')
    .replaceAll(RegExp(r'\s+'), ' ')
    .trim();

@visibleForTesting
void checkQuickstartSmokeOcrTextForTesting(String text) {
  final normalized = normalizeQuickstartSmokeOcrTextForTesting(text);
  final hasExpectedText =
      normalized.contains('hello') && normalized.contains('tom');
  if (!hasExpectedText) {
    throw Exception(
      'flutter_via_create web quickstart smoke test screenshot OCR did not '
      'contain expected text `Hello, Tom` (normalized OCR: `$normalized`)',
    );
  }
}

Future<void> _runFlutterDoctor() async => await exec('flutter doctor -v');

const kEnvEnableRustBacktrace = {'RUST_BACKTRACE': 'full'};
