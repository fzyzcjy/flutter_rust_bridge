// ignore_for_file: avoid_print

import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:io/io.dart';
import 'package:path/path.dart' as path;
import 'package:toml/toml.dart';

part 'build.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand(
      'build-flutter',
      buildFlutter,
      _$populateBuildFlutterConfigParser,
      _$parseBuildFlutterConfigResult,
    ),
    SimpleConfigCommand(
      'ohos-device-smoke',
      ohosDeviceSmoke,
      _$populateOhosDeviceSmokeConfigParser,
      _$parseOhosDeviceSmokeConfigResult,
    ),
  ];
}

// We do not test web, since it is already tested when building the demo on website
enum BuildTarget { windows, macos, linux, androidAab, androidApk, ios, ohos }

@CliOptions()
class BuildFlutterConfig {
  @CliOption(
    defaultsTo: 'frb_example/flutter_via_create',
    convert: convertConfigPackage,
  )
  final String package;
  final BuildTarget target;

  const BuildFlutterConfig({required this.package, required this.target});
}

@CliOptions()
class OhosDeviceSmokeConfig {
  final String hap;
  final String bundle;

  @CliOption(defaultsTo: 'EntryAbility')
  final String ability;

  @CliOption(defaultsTo: 'FRB_OHOS_SMOKE_RESULT=PASS')
  final String expectedLog;

  @CliOption(defaultsTo: 30)
  final int timeoutSeconds;

  final String? deviceId;

  @CliOption(defaultsTo: 'target/ohos_device_smoke')
  final String artifactDir;

  const OhosDeviceSmokeConfig({
    required this.hap,
    required this.bundle,
    required this.ability,
    required this.expectedLog,
    required this.timeoutSeconds,
    required this.deviceId,
    required this.artifactDir,
  });
}

// ref: https://docs.flutter.dev/deployment
Future<void> buildFlutter(BuildFlutterConfig config) async {
  final outputDir = '${exec.pwd}target/build_flutter_output';
  Directory(outputDir).createSync(recursive: true);
  void copyArtifacts(List<String> paths) {
    for (final path in paths) {
      copyPath('${exec.pwd}${config.package}/$path', outputDir);
    }
  }

  switch (config.target) {
    case BuildTarget.windows:
      // https://docs.flutter.dev/deployment/windows
      // https://docs.flutter.dev/platform-integration/windows/building#compiling-with-visual-studio
      await exec(
        'flutter build windows --verbose',
        relativePwd: config.package,
      );
      copyArtifacts(['build/windows/x64/runner/Release']);

    case BuildTarget.macos:
      // https://docs.flutter.dev/deployment/macos
      await exec('flutter build macos --verbose', relativePwd: config.package);
      copyArtifacts(['build/macos/Build/Products/Release']);

    case BuildTarget.linux:
      // https://docs.flutter.dev/deployment/linux
      // https://stackoverflow.com/questions/73278689/how-to-run-a-standalone-linux-app-built-with-flutter
      await exec('flutter build linux --verbose', relativePwd: config.package);
      copyArtifacts([
        linuxBuildBundlePathForTesting(
          machineArchitecture: currentMachineArchitectureForTesting(),
        ),
      ]);

    case BuildTarget.androidAab:
      // https://docs.flutter.dev/deployment/android
      await exec(
        'flutter build appbundle --verbose',
        relativePwd: config.package,
      );
      copyArtifacts(['build/app/outputs/bundle/release']);

    case BuildTarget.androidApk:
      // https://docs.flutter.dev/deployment/android
      await exec('flutter build apk --verbose', relativePwd: config.package);
      copyArtifacts(['build/app/outputs/apk/release']);

    case BuildTarget.ios:
      // https://docs.flutter.dev/deployment/ios
      await exec(
        'flutter build ipa --no-codesign --verbose',
        relativePwd: config.package,
      );
      copyArtifacts(['build/ios/archive']);

    case BuildTarget.ohos:
      await _runOhosBuildPreflight();
      final hapBackup = stashOhosHapOutputForTesting(
        Directory('${exec.pwd}${config.package}/build/ohos/hap'),
      );
      try {
        await exec(
          'flutter build hap --no-codesign --release --target-platform ohos-arm64 --verbose',
          relativePwd: config.package,
        );
        await _verifyOhosHapContainsRustLibrary(config.package);
      } catch (_) {
        restoreOhosHapOutputForTesting(hapBackup);
        rethrow;
      }
      hapBackup.backup?.deleteSync(recursive: true);
      copyArtifacts(['build/ohos/hap']);
  }
}

Future<void> ohosDeviceSmoke(OhosDeviceSmokeConfig config) async {
  if (config.timeoutSeconds <= 0) {
    throw ArgumentError.value(
      config.timeoutSeconds,
      'timeoutSeconds',
      'must be greater than zero',
    );
  }
  if (config.expectedLog.trim().isEmpty) {
    throw ArgumentError.value(
      config.expectedLog,
      'expectedLog',
      'must not be empty',
    );
  }

  final hapFile = File(path.absolute(config.hap));
  if (!hapFile.existsSync()) {
    throw StateError('Signed OHOS HAP does not exist: ${hapFile.path}');
  }

  final targetsResult = await _runHdc(['list', 'targets']);
  _ensureProcessSucceeded(targetsResult, operation: '`hdc list targets`');
  final deviceId = resolveOhosDeviceIdForTesting(
    targetsResult.stdout as String,
    requestedDeviceId: config.deviceId,
  );
  final hdcPrefix = ['-t', deviceId];

  final bundlesResult = await _runHdc([
    ...hdcPrefix,
    'shell',
    'bm',
    'dump',
    '-a',
  ]);
  _ensureProcessSucceeded(bundlesResult, operation: 'query installed bundles');
  if (ohosBundleAppearsInstalledForTesting(
    bundlesResult.stdout as String,
    bundle: config.bundle,
  )) {
    throw StateError(
      'Refusing to replace the already-installed bundle `${config.bundle}` '
      'on OHOS device $deviceId. Use a dedicated smoke-test bundle name.',
    );
  }

  final artifactDirectory = Directory(path.absolute(config.artifactDir));
  artifactDirectory.createSync(recursive: true);
  final safeDeviceId = deviceId.replaceAll(RegExp(r'[^A-Za-z0-9_.-]'), '_');
  final logFile = File(
    path.join(artifactDirectory.path, 'hilog-$safeDeviceId.log'),
  );

  var installedByThisRun = false;
  String? successMessage;
  Object? operationError;
  StackTrace? operationStackTrace;
  try {
    final installResult = await _runHdc([
      ...hdcPrefix,
      'install',
      '-r',
      hapFile.path,
    ]);
    _ensureProcessSucceeded(installResult, operation: 'install signed HAP');
    final installOutput = '${installResult.stdout}\n${installResult.stderr}';
    if (!ohosHdcInstallSucceededForTesting(installOutput)) {
      throw StateError('OHOS HAP installation failed:\n$installOutput');
    }
    installedByThisRun = true;

    final launchResult = await _runHdc([
      ...hdcPrefix,
      'shell',
      'aa',
      'start',
      '-a',
      config.ability,
      '-b',
      config.bundle,
    ]);
    _ensureProcessSucceeded(launchResult, operation: 'start OHOS ability');
    final launchOutput = '${launchResult.stdout}\n${launchResult.stderr}';
    if (!ohosHdcAbilityStartSucceededForTesting(launchOutput)) {
      throw StateError('OHOS ability launch failed:\n$launchOutput');
    }

    final deadline = DateTime.now().add(
      Duration(seconds: config.timeoutSeconds),
    );
    String? processId;
    var latestLogs = '';
    while (DateTime.now().isBefore(deadline)) {
      final pidResult = await _runHdc([
        ...hdcPrefix,
        'shell',
        'pidof',
        config.bundle,
      ]);
      final candidate = (pidResult.stdout as String).trim();
      if (pidResult.exitCode == 0 && RegExp(r'^\d+$').hasMatch(candidate)) {
        processId = candidate;
      }

      if (processId != null) {
        final logsResult = await _runHdc([
          ...hdcPrefix,
          'shell',
          'hilog',
          '-x',
          '-P',
          processId,
        ]);
        latestLogs = '${logsResult.stdout}\n${logsResult.stderr}';
        logFile.writeAsStringSync(latestLogs);
        if (ohosDeviceSmokeLogPassedForTesting(
          latestLogs,
          expectedLog: config.expectedLog,
        )) {
          successMessage =
              'OHOS device smoke passed: device=$deviceId, pid=$processId, '
              'marker=${config.expectedLog}, logs=${logFile.path}';
          break;
        }
      }

      await Future<void>.delayed(const Duration(seconds: 1));
    }

    if (successMessage == null) {
      throw StateError(
        'Timed out after ${config.timeoutSeconds}s waiting for OHOS smoke '
        'marker `${config.expectedLog}` on device $deviceId. '
        'Device logs: ${logFile.path}\n$latestLogs',
      );
    }
  } catch (error, stackTrace) {
    operationError = error;
    operationStackTrace = stackTrace;
  }

  Object? cleanupError;
  if (installedByThisRun) {
    try {
      final uninstallResult = await _runHdc([
        ...hdcPrefix,
        'uninstall',
        config.bundle,
      ]);
      final uninstallOutput =
          '${uninstallResult.stdout}\n${uninstallResult.stderr}';
      if (uninstallResult.exitCode != 0 ||
          !uninstallOutput.contains('uninstall bundle successfully')) {
        cleanupError = StateError(
          'Failed to clean up OHOS smoke bundle '
          '`${config.bundle}` from device $deviceId:\n$uninstallOutput',
        );
      }
    } catch (error) {
      cleanupError = error;
    }
  }

  if (operationError != null) {
    if (cleanupError != null) {
      Error.throwWithStackTrace(
        StateError(
          '$operationError\nAdditionally, OHOS smoke cleanup failed: '
          '$cleanupError',
        ),
        operationStackTrace!,
      );
    }
    Error.throwWithStackTrace(operationError, operationStackTrace!);
  }
  if (cleanupError != null) throw cleanupError;
  if (successMessage == null) {
    throw StateError('OHOS smoke finished without a result');
  }
  print(successMessage);
}

Future<ProcessResult> _runHdc(List<String> arguments) async {
  try {
    return await Process.run('hdc', arguments);
  } on ProcessException catch (error) {
    throw StateError(
      'Cannot execute `hdc`: ${error.message}. Install the HarmonyOS '
      'command-line tools and add `hdc` to PATH.',
    );
  }
}

void _ensureProcessSucceeded(
  ProcessResult result, {
  required String operation,
}) {
  if (result.exitCode != 0) {
    throw StateError(
      'Failed to $operation (exit code ${result.exitCode}):\n'
      '${result.stdout}\n${result.stderr}',
    );
  }
}

String resolveOhosDeviceIdForTesting(
  String targetsOutput, {
  required String? requestedDeviceId,
}) {
  final targets = targetsOutput
      .split(RegExp(r'\r?\n'))
      .map((line) => line.trim().split(RegExp(r'\s+')).first)
      .where((target) => target.isNotEmpty && target != '[Empty]')
      .toSet()
      .toList();

  if (requestedDeviceId != null && requestedDeviceId.trim().isNotEmpty) {
    if (!targets.contains(requestedDeviceId)) {
      throw StateError(
        'Requested OHOS device `$requestedDeviceId` is not connected. '
        'Connected targets: ${targets.join(', ')}',
      );
    }
    return requestedDeviceId;
  }
  if (targets.isEmpty) {
    throw StateError(
      'No OHOS device is connected. Enable developer mode and verify '
      '`hdc list targets`.',
    );
  }
  if (targets.length > 1) {
    throw StateError(
      'Multiple OHOS devices are connected (${targets.join(', ')}). '
      'Pass `--device-id` explicitly.',
    );
  }
  return targets.single;
}

bool ohosBundleAppearsInstalledForTesting(
  String bundlesOutput, {
  required String bundle,
}) => RegExp(
  '(^|[^A-Za-z0-9_.-])${RegExp.escape(bundle)}([^A-Za-z0-9_.-]|\$)',
  multiLine: true,
).hasMatch(bundlesOutput);

bool ohosHdcInstallSucceededForTesting(String output) =>
    output.contains('install bundle successfully');

bool ohosHdcAbilityStartSucceededForTesting(String output) =>
    output.toLowerCase().contains('start ability successfully');

bool ohosDeviceSmokeLogPassedForTesting(
  String logs, {
  required String expectedLog,
}) {
  final marker = expectedLog.trim();
  if (marker.isEmpty) return false;

  final lines = logs.split(RegExp(r'\r?\n'));
  final processStartIndex = lines.lastIndexWhere(
    (line) => line.contains('APPSPAWN: AppSpawnChild'),
  );
  if (processStartIndex < 0) return false;

  final markerAtEndOfLine = RegExp('${RegExp.escape(marker)}\\s*\$');
  return lines.skip(processStartIndex + 1).any(markerAtEndOfLine.hasMatch);
}

Future<void> _runOhosBuildPreflight() async {
  final sdkHome = Platform.environment['OHOS_SDK_HOME'];
  final errors = validateOhosSdkHomeForTesting(
    sdkHome: sdkHome,
    isWindows: Platform.isWindows,
    pathExists: (candidate) =>
        FileSystemEntity.typeSync(candidate) != FileSystemEntityType.notFound,
  );

  try {
    final result = await Process.run('flutter', ['create', '--help']);
    final help = '${result.stdout}\n${result.stderr}';
    if (result.exitCode != 0) {
      errors.add(
        '`flutter create --help` failed. Install or select the OHOS-enabled '
        'Flutter fork before building.',
      );
    } else if (!ohosFlutterCreateHelpSupportsPlatformForTesting(help)) {
      errors.add(
        'The active Flutter SDK does not advertise `ohos` in '
        '`flutter create --help`. Select the OHOS-enabled Flutter fork.',
      );
    }
  } on ProcessException catch (error) {
    errors.add(
      'Cannot execute `flutter`: ${error.message}. Add the OHOS-enabled '
      'Flutter SDK to PATH.',
    );
  }

  if (await _findOhosArchiveLister() == null) {
    errors.add(
      'Cannot inspect HAP archives because neither `jar` nor `unzip` is '
      'available. Install a full JDK 17 or add `unzip` to PATH.',
    );
  }

  if (errors.isNotEmpty) {
    throw StateError(
      'OHOS build environment preflight failed:\n'
      '${errors.map((error) => '- $error').join('\n')}\n'
      'See website/docs/guides/miscellaneous/harmony-os.md for setup steps.',
    );
  }

  print(
    'OHOS build environment preflight passed: '
    'SDK=$sdkHome, Flutter target=ohos-arm64, '
    'Rust target=aarch64-unknown-linux-ohos, HAP ABI=arm64-v8a',
  );
}

List<String> validateOhosSdkHomeForTesting({
  required String? sdkHome,
  required bool isWindows,
  required bool Function(String candidate) pathExists,
}) {
  if (sdkHome == null || sdkHome.trim().isEmpty) {
    return [
      '`OHOS_SDK_HOME` is not set. Point it to the OpenHarmony SDK '
          '`native` directory (the directory containing `llvm` and `sysroot`).',
    ];
  }

  final errors = <String>[];
  if (RegExp(r'\s').hasMatch(sdkHome)) {
    errors.add(
      '`OHOS_SDK_HOME` contains whitespace: $sdkHome. Move the SDK to a '
      'path without spaces or other whitespace.',
    );
  }
  if (sdkHome.runes.any((character) => character > 0x7f)) {
    errors.add(
      '`OHOS_SDK_HOME` contains non-ASCII characters: $sdkHome. Move the SDK '
      'to a plain ASCII path.',
    );
  }
  if (!pathExists(sdkHome)) {
    errors.add(
      '`OHOS_SDK_HOME` does not exist: $sdkHome. Point it to the installed '
      'SDK `native` directory.',
    );
    return errors;
  }

  final pathContext = isWindows ? path.windows : path.posix;
  final executableSuffix = isWindows ? '.exe' : '';
  final requiredComponents = [
    pathContext.join('llvm', 'bin', 'clang$executableSuffix'),
    pathContext.join('llvm', 'bin', 'llvm-ar$executableSuffix'),
    'sysroot',
  ];
  for (final component in requiredComponents) {
    final candidate = pathContext.join(sdkHome, component);
    if (!pathExists(candidate)) {
      errors.add(
        '`OHOS_SDK_HOME` is missing `$component`: $candidate. Ensure the '
        'OpenHarmony native SDK component is installed and select its '
        '`native` directory.',
      );
    }
  }

  return errors;
}

bool ohosFlutterCreateHelpSupportsPlatformForTesting(String help) {
  var inPlatformsOption = false;
  for (final line in help.split('\n')) {
    final trimmed = line.trimLeft();
    final startsNewOption =
        trimmed.startsWith('--') ||
        trimmed.startsWith('-') && trimmed.contains('--');
    if (inPlatformsOption &&
        startsNewOption &&
        !trimmed.contains('--platforms')) {
      return false;
    }
    if (trimmed.contains('--platforms')) {
      inPlatformsOption = true;
    }
    if (inPlatformsOption &&
        line
            .split(RegExp(r'[^A-Za-z0-9_]+'))
            .any((token) => token.toLowerCase() == 'ohos')) {
      return true;
    }
  }
  return false;
}

({Directory output, Directory? backup}) stashOhosHapOutputForTesting(
  Directory output,
) {
  if (!output.existsSync()) return (output: output, backup: null);
  final backup = Directory(
    '${output.path}.frb_backup_${pid}_${DateTime.now().microsecondsSinceEpoch}',
  );
  output.renameSync(backup.path);
  return (output: output, backup: backup);
}

void restoreOhosHapOutputForTesting(
  ({Directory output, Directory? backup}) state,
) {
  if (state.output.existsSync()) {
    state.output.deleteSync(recursive: true);
  }
  if (state.backup != null) {
    state.backup!.renameSync(state.output.path);
  }
}

Future<void> _verifyOhosHapContainsRustLibrary(String package) async {
  final packageDir = '${exec.pwd}$package';
  final cargoTomlPath = ohosRustCargoTomlPathForTesting(
    packageDir: packageDir,
    fileExists: (candidate) => File(candidate).existsSync(),
  );
  final cargoToml = File(cargoTomlPath).readAsStringSync();
  final expectedLibrary = ohosRustLibraryNameForTesting(cargoToml);
  final hapDirectory = Directory('$packageDir/build/ohos/hap');
  final hapFiles = hapDirectory.existsSync()
      ? hapDirectory
            .listSync(recursive: true)
            .whereType<File>()
            .where((file) => file.path.endsWith('.hap'))
            .toList()
      : <File>[];

  if (hapFiles.isEmpty) {
    throw StateError(
      'OHOS build produced no HAP files in ${hapDirectory.path}',
    );
  }

  for (final hapFile in hapFiles) {
    final entries = await _listOhosHapEntries(hapFile.path);
    if (ohosHapContainsRustLibraryForTesting(
      entries,
      expectedLibrary: expectedLibrary,
    )) {
      return;
    }
  }

  throw StateError(
    'OHOS HAP does not contain $expectedLibrary for arm64-v8a: '
    '${hapFiles.map((file) => file.path).join(', ')}',
  );
}

Future<({String executable, List<String> arguments})?>
_findOhosArchiveLister() async {
  for (final candidate in ohosArchiveListerCandidatesForTesting()) {
    try {
      final result = await Process.run(
        candidate.executable,
        candidate.arguments,
      );
      if (result.exitCode == 0) return candidate;
    } on ProcessException {
      // Try the next supported archive lister.
    }
  }
  return null;
}

Future<List<String>> _listOhosHapEntries(String hapPath) async {
  final lister = await _findOhosArchiveLister();
  if (lister == null) {
    throw StateError(
      'Cannot inspect OHOS HAP $hapPath because neither `jar` nor `unzip` '
      'is available. Install a full JDK 17 or add `unzip` to PATH.',
    );
  }
  final arguments = switch (lister.executable) {
    'jar' => ['tf', hapPath],
    'unzip' => ['-Z1', hapPath],
    _ => throw StateError(
      'Unsupported OHOS HAP archive lister: ${lister.executable}',
    ),
  };
  final result = await Process.run(lister.executable, arguments);
  if (result.exitCode != 0) {
    throw StateError(
      'Failed to inspect OHOS HAP $hapPath with ${lister.executable}: '
      '${result.stderr}',
    );
  }
  return (result.stdout as String).split(RegExp(r'\r?\n'));
}

List<({String executable, List<String> arguments})>
ohosArchiveListerCandidatesForTesting() => const [
  (executable: 'jar', arguments: ['--version']),
  (executable: 'unzip', arguments: ['-v']),
];

String ohosRustCargoTomlPathForTesting({
  required String packageDir,
  required bool Function(String candidate) fileExists,
}) {
  final candidates = [
    path.join(packageDir, 'rust', 'Cargo.toml'),
    path.join(path.dirname(packageDir), 'rust', 'Cargo.toml'),
  ];
  for (final candidate in candidates) {
    if (fileExists(candidate)) return candidate;
  }
  throw StateError(
    'Cannot locate Rust Cargo.toml for OHOS HAP validation. Checked: '
    '${candidates.join(', ')}',
  );
}

String ohosRustLibraryNameForTesting(String cargoToml) {
  final manifest = TomlDocument.parse(cargoToml).toMap();
  final library = manifest['lib'];
  final explicitLibraryName = library is Map ? library['name'] : null;
  if (explicitLibraryName is String && explicitLibraryName.isNotEmpty) {
    return 'lib${explicitLibraryName.replaceAll('-', '_')}.so';
  }

  final package = manifest['package'];
  final crateName = package is Map ? package['name'] : null;
  if (crateName is! String || crateName.isEmpty) {
    throw StateError('Rust Cargo.toml does not define package.name');
  }
  return 'lib${crateName.replaceAll('-', '_')}.so';
}

bool ohosHapContainsRustLibraryForTesting(
  Iterable<String> entries, {
  required String expectedLibrary,
}) => entries.any(
  (entry) =>
      entry.trim().replaceAll('\\', '/') == 'libs/arm64-v8a/$expectedLibrary',
);

String linuxBuildBundlePathForTesting({required String machineArchitecture}) =>
    'build/linux/${_linuxFlutterArchitecture(machineArchitecture)}/release/bundle';

String currentMachineArchitectureForTesting() {
  try {
    final result = Process.runSync('uname', ['-m']);
    if (result.exitCode != 0) {
      throw StateError(
        'Failed to detect machine architecture: ${result.stderr}',
      );
    }

    return (result.stdout as String).trim();
  } on ProcessException catch (error) {
    throw StateError(
      'Failed to run "uname" to detect machine architecture: ${error.message}',
    );
  }
}

String _linuxFlutterArchitecture(String machineArchitecture) =>
    switch (machineArchitecture) {
      'x86_64' || 'amd64' => 'x64',
      'aarch64' || 'arm64' => 'arm64',
      'riscv64' => 'riscv64',
      _ => throw UnsupportedError(
        'Unsupported Linux machine architecture: $machineArchitecture',
      ),
    };
