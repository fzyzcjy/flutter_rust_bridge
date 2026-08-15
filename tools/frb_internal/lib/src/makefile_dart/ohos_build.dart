// ignore_for_file: avoid_print

import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/ohos_hap.dart';
import 'package:path/path.dart' as path;
import 'package:toml/toml.dart';

Future<void> buildOhos(String package) async {
  await _runOhosBuildPreflight();
  final hapBackup = stashOhosHapOutputForTesting(
    Directory('${exec.pwd}$package/build/ohos/hap'),
  );
  try {
    await exec(
      'flutter build hap --no-codesign --release --target-platform ohos-arm64 --verbose',
      relativePwd: package,
    );
    await _verifyOhosHapContainsRustLibrary(package);
  } catch (_) {
    restoreOhosHapOutputForTesting(hapBackup);
    rethrow;
  }
  hapBackup.backup?.deleteSync(recursive: true);
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

  if (!await hasOhosArchiveLister()) {
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

  final entriesByHap = <String, Iterable<String>>{};
  for (final hapFile in hapFiles) {
    entriesByHap[hapFile.path] = await listOhosHapEntries(hapFile.path);
  }
  validateOhosHapRustLibrariesForTesting(
    entriesByHap,
    expectedLibrary: expectedLibrary,
  );
}

void validateOhosHapRustLibrariesForTesting(
  Map<String, Iterable<String>> entriesByHap, {
  required String expectedLibrary,
}) {
  final invalidHaps = entriesByHap.entries
      .where(
        (entry) => !ohosHapContainsRustLibraryForTesting(
          entry.value,
          expectedLibrary: expectedLibrary,
        ),
      )
      .map((entry) => entry.key)
      .toList();
  if (invalidHaps.isNotEmpty) {
    throw StateError(
      'OHOS HAPs do not contain $expectedLibrary for arm64-v8a: '
      '${invalidHaps.join(', ')}',
    );
  }
}

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
