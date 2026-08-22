import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/ohos_build.dart';
import 'package:test/test.dart';

void main() {
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
