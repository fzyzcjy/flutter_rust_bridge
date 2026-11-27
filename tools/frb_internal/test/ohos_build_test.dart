import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/ohos_build.dart';
import 'package:test/test.dart';

void main() {
  test('OHOS target mapping selects Flutter, Rust, and HAP architectures', () {
    expect(resolveOhosTargetForTesting(null).hapAbi, 'arm64-v8a');
    expect(
      resolveOhosTargetForTesting('ohos-x64').rustTarget,
      'x86_64-unknown-linux-ohos',
    );
    expect(resolveOhosTargetForTesting('ohos-x64').hapAbi, 'x86_64');
    expect(() => resolveOhosTargetForTesting('ohos-arm'), throwsArgumentError);
    expect(
      () => resolveOhosTargetForTesting('ohos-invalid'),
      throwsArgumentError,
    );
  });

  test('OHOS build mode defaults to release and accepts supported modes', () {
    expect(resolveOhosBuildModeForTesting(null), 'release');
    expect(resolveOhosBuildModeForTesting('debug'), 'debug');
    expect(resolveOhosBuildModeForTesting('profile'), 'profile');
    expect(resolveOhosBuildModeForTesting(' release '), 'release');
    expect(
      () => resolveOhosBuildModeForTesting('staging'),
      throwsArgumentError,
    );
  });

  test('OHOS HAP validation searches Flutter and Hvigor output locations', () {
    final directories = ohosHapOutputDirectoriesForTesting('/tmp/example');
    expect(directories.first.path, endsWith('/build/ohos/hap'));
    expect(directories.last.path, endsWith('/ohos/entry/build'));
  });

  test('OHOS HAP output is normalized for artifact copying', () {
    final temporaryDirectory = Directory.systemTemp.createTempSync(
      'frb_ohos_hap_normalize_test_',
    );
    try {
      final entryBuild = Directory(
        '${temporaryDirectory.path}/ohos/entry/build/outputs',
      )..createSync(recursive: true);
      File('${entryBuild.path}/entry-default.hap').writeAsStringSync('hap');

      final canonical = normalizeOhosHapOutputForTesting(
        temporaryDirectory.path,
      );

      expect(canonical.path, endsWith('/build/ohos/hap'));
      expect(
        File('${canonical.path}/entry-default.hap').readAsStringSync(),
        'hap',
      );
    } finally {
      temporaryDirectory.deleteSync(recursive: true);
    }
  });

  test('OHOS Rust library name follows Cargo crate naming rules', () {
    expect(
      ohosRustLibraryNameForTesting('''
[package]
name = "rust-lib-example"
version = "0.1.0"
'''),
      'librust_lib_example.so',
    );
    expect(
      ohosRustLibraryNameForTesting('''
[package]
name = "rust-lib-example"
version = "0.1.0"

[lib]
name = "custom_bridge"
crate-type = ["cdylib"]
'''),
      'libcustom_bridge.so',
    );
  });

  test('OHOS HAP validation resolves app and plugin Rust manifests', () {
    expect(
      ohosRustCargoTomlPathForTesting(
        packageDir: '/workspace/app',
        fileExists: (candidate) =>
            candidate == '/workspace/app/rust/Cargo.toml',
      ),
      '/workspace/app/rust/Cargo.toml',
    );
    expect(
      ohosRustCargoTomlPathForTesting(
        packageDir: '/workspace/plugin/example',
        fileExists: (candidate) =>
            candidate == '/workspace/plugin/rust/Cargo.toml',
      ),
      '/workspace/plugin/rust/Cargo.toml',
    );
  });

  test('OHOS HAP validation rejects any archive missing the Rust library', () {
    expect(
      () => validateOhosHapRustLibrariesForTesting({
        '/build/entry.hap': ['libs/arm64-v8a/librust_lib_example.so'],
        '/build/feature.hap': ['libs/arm64-v8a/libother.so'],
      }, expectedLibrary: 'librust_lib_example.so'),
      throwsA(
        isA<StateError>().having(
          (error) => error.message,
          'message',
          contains('/build/feature.hap'),
        ),
      ),
    );
  });

  test('OHOS HAP backup restores the previous output after failure', () {
    final temporaryDirectory = Directory.systemTemp.createTempSync(
      'frb_ohos_hap_backup_test_',
    );
    try {
      final output = Directory('${temporaryDirectory.path}/hap')..createSync();
      File('${output.path}/previous.hap').writeAsStringSync('previous');

      final backup = stashOhosHapOutputForTesting(output);
      expect(output.existsSync(), isFalse);
      expect(backup.backup!.existsSync(), isTrue);

      output.createSync();
      File('${output.path}/failed.hap').writeAsStringSync('failed');
      restoreOhosHapOutputForTesting(backup);

      expect(
        File('${output.path}/previous.hap').readAsStringSync(),
        'previous',
      );
      expect(File('${output.path}/failed.hap').existsSync(), isFalse);
      expect(backup.backup!.existsSync(), isFalse);
    } finally {
      temporaryDirectory.deleteSync(recursive: true);
    }
  });

  test('OHOS HAP backup removes failed first-build output', () {
    final temporaryDirectory = Directory.systemTemp.createTempSync(
      'frb_ohos_hap_first_build_test_',
    );
    try {
      final output = Directory('${temporaryDirectory.path}/hap');
      final backup = stashOhosHapOutputForTesting(output);
      expect(backup.backup, isNull);

      output.createSync();
      File('${output.path}/failed.hap').writeAsStringSync('failed');
      restoreOhosHapOutputForTesting(backup);

      expect(output.existsSync(), isFalse);
    } finally {
      temporaryDirectory.deleteSync(recursive: true);
    }
  });

  test('OHOS HAP backup covers all supported output locations', () {
    final temporaryDirectory = Directory.systemTemp.createTempSync(
      'frb_ohos_hap_multi_backup_test_',
    );
    try {
      final firstOutput = Directory('${temporaryDirectory.path}/flutter/hap')
        ..createSync(recursive: true);
      File('${firstOutput.path}/previous.hap').writeAsStringSync('previous');
      final secondOutput = Directory('${temporaryDirectory.path}/entry/build')
        ..createSync(recursive: true);
      File('${secondOutput.path}/previous.hap').writeAsStringSync('previous');

      final backup = stashOhosHapOutputsForTesting([firstOutput, secondOutput]);
      expect(firstOutput.existsSync(), isFalse);
      expect(secondOutput.existsSync(), isFalse);

      firstOutput.createSync(recursive: true);
      secondOutput.createSync(recursive: true);
      File('${firstOutput.path}/failed.hap').writeAsStringSync('failed');
      File('${secondOutput.path}/failed.hap').writeAsStringSync('failed');
      restoreOhosHapOutputsForTesting(backup);

      expect(
        File('${firstOutput.path}/previous.hap').readAsStringSync(),
        'previous',
      );
      expect(
        File('${secondOutput.path}/previous.hap').readAsStringSync(),
        'previous',
      );
      expect(File('${firstOutput.path}/failed.hap').existsSync(), isFalse);
      expect(File('${secondOutput.path}/failed.hap').existsSync(), isFalse);
      deleteOhosHapBackupsForTesting(backup);
      expect(
        backup.entries.every(
          (entry) => entry.backup == null || !entry.backup!.existsSync(),
        ),
        isTrue,
      );
    } finally {
      temporaryDirectory.deleteSync(recursive: true);
    }
  });

  test('OHOS SDK preflight accepts a complete native SDK directory', () {
    const sdkHome = '/opt/ohos/18/native';
    final existingPaths = {
      sdkHome,
      '$sdkHome/llvm/bin/clang',
      '$sdkHome/llvm/bin/llvm-ar',
      '$sdkHome/sysroot',
    };

    expect(
      validateOhosSdkHomeForTesting(
        sdkHome: sdkHome,
        isWindows: false,
        pathExists: existingPaths.contains,
      ),
      isEmpty,
    );
  });

  test('OHOS SDK preflight reports missing native toolchain components', () {
    const sdkHome = '/opt/ohos/18/native';

    expect(
      validateOhosSdkHomeForTesting(
        sdkHome: sdkHome,
        isWindows: false,
        pathExists: (candidate) => candidate == sdkHome,
      ),
      containsAll([
        contains('llvm/bin/clang'),
        contains('llvm/bin/llvm-ar'),
        contains('sysroot'),
      ]),
    );
  });

  test('OHOS SDK preflight rejects missing and unsafe SDK paths', () {
    expect(
      validateOhosSdkHomeForTesting(
        sdkHome: null,
        isWindows: false,
        pathExists: (_) => false,
      ).single,
      contains('is not set'),
    );

    final errors = validateOhosSdkHomeForTesting(
      sdkHome: '/opt/鸿蒙 sdk/native',
      isWindows: false,
      pathExists: (_) => true,
    );
    expect(errors, contains(contains('contains whitespace')));
    expect(errors, contains(contains('contains non-ASCII')));
  });

  test('OHOS Flutter preflight only accepts ohos in platforms help', () {
    expect(
      ohosFlutterCreateHelpSupportsPlatformForTesting('''
--platforms          The platforms supported by this project.
                     [android, ios, ohos]
--project-name       The project name.
'''),
      isTrue,
    );
    expect(
      ohosFlutterCreateHelpSupportsPlatformForTesting('''
--platforms          The platforms supported by this project.
                     [android, ios]
--description        Mentions OHOS elsewhere.
'''),
      isFalse,
    );
  });
}
