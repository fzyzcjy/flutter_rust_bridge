import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/build.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/build_cli.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate_from_scratch.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/lint.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/post_release.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/quickstart_smoke.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/released_version.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester/dart_sdk.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester/runner.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester/sanitizer.dart';
import 'package:test/test.dart';

void main() {
  test('known thread suppressions are limited to their TSAN entrypoints', () {
    for (final package in [
      'frb_example/pure_dart_pde',
      'frb_example/pure_dart',
      'frb_example/deliberate_bad',
      'frb_example/dart_minimal',
    ]) {
      for (final sanitizer in Sanitizer.values) {
        final environment = sanitizerEntrypointEnvironmentForTesting(
          package: package,
          sanitizer: sanitizer,
          environment: const {'TSAN_OPTIONS': 'halt_on_error=1'},
        );
        expect(
          environment,
          (package == 'frb_example/pure_dart_pde' ||
                      package == 'frb_example/pure_dart') &&
                  sanitizer == Sanitizer.tsan
              ? {
                  'TSAN_OPTIONS':
                      'halt_on_error=1:report_thread_leaks=1:'
                      'print_suppressions=1:'
                      'suppressions=../../tools/dart_tsan_${package.endsWith('_pde') ? 'pde' : 'pure'}.supp',
                }
              : <String, String>{},
        );
      }
    }
  });

  test('PDE thread suppression accepts only one known creation path', () {
    const rule =
        'thread:frb_example_pure_dart_pde::api::async_spawn::'
        'simple_use_async_spawn_blocking::';
    expect(
      File('../../tools/dart_tsan_pde.supp').readAsStringSync(),
      '$rule\n',
    );
    const report =
        'ThreadSanitizer: Matched 1 suppressions (pid=123):\n'
        '1 $rule\n';
    expect(() => checkPdeThreadLeakSuppressionForTesting(''), returnsNormally);
    expect(
      () => checkPdeThreadLeakSuppressionForTesting(report),
      returnsNormally,
    );
    for (final unexpected in [
      report.replaceAll('Matched 1', 'Matched 2'),
      report.replaceAll('1 thread:', '2 thread:'),
      report.replaceAll('thread:', 'race:'),
      report.replaceAll('simple_use_async_spawn_blocking::', 'different_api::'),
      '$report$report',
      'ThreadSanitizer: Matched malformed summary\n',
    ]) {
      expect(
        () => checkPdeThreadLeakSuppressionForTesting(unexpected),
        throwsException,
      );
    }
  });

  test('pure thread suppression accepts only its one known creation path', () {
    const rule =
        'thread:frb_example_pure_dart::api::async_spawn::'
        'simple_use_async_spawn_blocking::';
    expect(
      File('../../tools/dart_tsan_pure.supp').readAsStringSync(),
      '$rule\n',
    );
    const report =
        'ThreadSanitizer: Matched 1 suppressions (pid=123):\n'
        '1 $rule\n';
    expect(() => checkPureThreadLeakSuppressionForTesting(''), returnsNormally);
    expect(
      () => checkPureThreadLeakSuppressionForTesting(report),
      returnsNormally,
    );
    for (final unexpected in [
      report.replaceAll('Matched 1', 'Matched 2'),
      report.replaceAll('1 thread:', '2 thread:'),
      report.replaceAll('thread:', 'race:'),
      report.replaceAll('simple_use_async_spawn_blocking::', 'different_api::'),
      report.replaceAll('pure_dart::', 'pure_dart_pde::'),
      '$report$report',
      'ThreadSanitizer: Matched malformed summary\n',
    ]) {
      expect(
        () => checkPureThreadLeakSuppressionForTesting(unexpected),
        throwsException,
      );
    }
    expect(
      () => checkPdeThreadLeakSuppressionForTesting(report),
      throwsException,
    );
  });

  test('dart valgrind command uses the Dart AOT suppression file', () {
    expect(
      dartValgrindCommandForTesting(),
      contains('--suppressions=../../tools/dart_valgrind.supp'),
    );
    expect(File('../../tools/dart_valgrind.supp').existsSync(), true);
  });

  test('dart valgrind command treats only checked leak kinds as errors', () {
    expect(dartValgrindCommandForTesting(), contains('--error-exitcode=1'));
    expect(
      dartValgrindCommandForTesting(),
      contains('--errors-for-leak-kinds=definite,indirect'),
    );
  });

  test('Valgrind does not suppress FRB serializer or CST allocations', () {
    final suppressions = File('../../tools/dart_valgrind.supp')
        .readAsStringSync();
    expect(suppressions, isNot(contains('frbgen_')));
    expect(suppressions, isNot(contains('frb_rust_vec_u8_new')));
  });

  test('dart valgrind compile command uses dart build output directory', () {
    expect(
      dartValgrindCompileCommandForTesting(),
      'dart build cli -t test/dart_valgrind_test_entrypoint.dart -o build/valgrind_test_output/',
    );
  });

  test('dart valgrind output directory matches build output directory', () {
    expect(
      dartValgrindOutputDirectoryForTesting(),
      'build/valgrind_test_output/',
    );
  });

  test('dart valgrind executable path points to cli bundle binary', () {
    expect(
      dartValgrindOutputExecutablePathForTesting(),
      'build/valgrind_test_output/bundle/bin/dart_valgrind_test_entrypoint',
    );
  });

  test('sanitized Dart release defaults to checked-in artifact tag', () {
    expect(
      sanitizedDartReleaseName(environment: {}),
      kDefaultSanitizedDartReleaseName,
    );
  });

  test('sanitized Dart release can be overridden by environment', () {
    expect(
      sanitizedDartReleaseName(
        environment: {'FRB_SANITIZED_DART_RELEASE_NAME': ' Build_test '},
      ),
      'Build_test',
    );
  });

  test('sanitized Dart cache path remains outside the repository', () {
    expect(
      sanitizedDartCacheRelativePathForTesting(
        repoRootPath:
            '/home/runner/work/flutter_rust_bridge/flutter_rust_bridge',
        cacheRootPath: '/tmp/frb_sanitized_dart/release',
      ),
      '../../../../../tmp/frb_sanitized_dart/release',
    );
  });

  test('sanitized Dart version check is skipped without main Dart env', () {
    checkSanitizedDartVersionForTesting(
      versionOutput: 'Dart SDK version: 3.11.0 (stable)',
      environment: {},
    );
  });

  test('sanitized Dart version check accepts matching main Dart env', () {
    checkSanitizedDartVersionForTesting(
      versionOutput: 'Dart SDK version: 3.11.0 (stable)',
      environment: {'FRB_MAIN_DART_VERSION': '3.11.0'},
    );
  });

  test('sanitized Dart version check rejects stale artifact version', () {
    expect(
      () => checkSanitizedDartVersionForTesting(
        versionOutput: 'Dart SDK version: 3.10.0 (stable)',
        environment: {'FRB_MAIN_DART_VERSION': '3.11.0'},
      ),
      throwsA(
        isA<Exception>().having(
          (error) => error.toString(),
          'message',
          contains('Build a new sanitized Dart artifact'),
        ),
      ),
    );
  });

  test('ASAN rustflags keep production runtime semantics', () {
    expect(
      sanitizerRustflagsForTesting(Sanitizer.asan),
      '-Zsanitizer=address -Zmerge-functions=disabled -Cdebuginfo=1',
    );
  });

  test('sanitizer rustflags keep full MSAN instrumentation', () {
    expect(
      sanitizerRustflagsForTesting(Sanitizer.msan),
      '-Zsanitizer=memory -Zmerge-functions=disabled -Cdebuginfo=1',
    );
  });

  test('LSAN rustflags keep production runtime semantics', () {
    expect(
      sanitizerRustflagsForTesting(Sanitizer.lsan),
      '-Zsanitizer=leak -Zmerge-functions=disabled -Cdebuginfo=1',
    );
  });

  test('TSAN rustflags preserve production synchronization semantics', () {
    expect(
      sanitizerRustflagsForTesting(Sanitizer.tsan),
      '-Zsanitizer=thread -Zmerge-functions=disabled -Cdebuginfo=1',
    );
  });

  test(
    'sanitizer failure matching requires the complete normalized report',
    () {
      const knownReport =
          '==123==ERROR: LeakSanitizer: detected memory leaks\n'
          'Direct leak of 16 byte(s) in 1 object(s) allocated from:\n'
          '    #0 0x1234  (/tmp/sdk/out/dartvm+0x217a63f) '
          '(BuildId: abcdef)\n'
          '    #1 0xabcd known_runtime_function '
          '(libfrb_example_pure_dart.so+0x11f0446) (BuildId: 012345)\n'
          'SUMMARY: LeakSanitizer: 16 byte(s) leaked in 1 allocation(s).\n';
      final normalizedKnownReport = normalizeSanitizerReportForTesting(
        knownReport,
      );

      expect(
        isOnlyAllowedSanitizerFailureForTesting(
          exitCode: 23,
          stderr: knownReport,
          expectedExitCode: 23,
          expectedNormalizedReports: [normalizedKnownReport],
        ),
        isTrue,
      );
      expect(
        isOnlyAllowedSanitizerFailureForTesting(
          exitCode: 23,
          stderr:
              '==123==ERROR: LeakSanitizer: detected memory leaks\n'
              'SUMMARY: LeakSanitizer: 16 byte(s) leaked in 1 allocation(s).\n',
          expectedExitCode: 23,
          expectedNormalizedReports: [normalizedKnownReport],
        ),
        isFalse,
      );
      final unrelatedStackReport = knownReport.replaceFirst(
        'known_runtime_function',
        'new_regression_function',
      );
      expect(
        isOnlyAllowedSanitizerFailureForTesting(
          exitCode: 23,
          stderr: unrelatedStackReport,
          expectedExitCode: 23,
          expectedNormalizedReports: [normalizedKnownReport],
        ),
        isFalse,
      );
      final unstableValuesChanged = knownReport
          .replaceAll('0x1234', '0x9999')
          .replaceAll('0xabcd', '0x8888')
          .replaceAll('==123==', '==987==')
          .replaceAll('/tmp/sdk/out/dartvm', '/other/sdk/dartvm')
          .replaceAll('abcdef', 'fedcba')
          .replaceAll('012345', '543210');
      expect(
        normalizeSanitizerReportForTesting(unstableValuesChanged),
        normalizedKnownReport,
      );
      expect(
        isOnlyAllowedSanitizerFailureForTesting(
          exitCode: 23,
          stderr: knownReport,
          expectedExitCode: 23,
          expectedNormalizedReports: const [],
        ),
        isFalse,
      );
    },
  );

  test('linux build bundle path follows the current machine architecture', () {
    expect(
      linuxBuildBundlePathForTesting(machineArchitecture: 'x86_64'),
      'build/linux/x64/release/bundle',
    );
    expect(
      linuxBuildBundlePathForTesting(machineArchitecture: 'amd64'),
      'build/linux/x64/release/bundle',
    );
    expect(
      linuxBuildBundlePathForTesting(machineArchitecture: 'aarch64'),
      'build/linux/arm64/release/bundle',
    );
    expect(
      linuxBuildBundlePathForTesting(machineArchitecture: 'arm64'),
      'build/linux/arm64/release/bundle',
    );
    expect(
      linuxBuildBundlePathForTesting(machineArchitecture: 'riscv64'),
      'build/linux/riscv64/release/bundle',
    );
  });

  test('GitHub release create command does not label prerelease versions', () {
    expect(
      githubReleaseCreateCommand(
        version: '2.13.0-beta.1',
        notesFile: 'temp.txt',
      ),
      'gh release create v2.13.0-beta.1 --notes-file temp.txt --title v2.13.0-beta.1',
    );
  });

  test(
    'GitHub release create command keeps stable versions latest-neutral',
    () {
      expect(
        githubReleaseCreateCommand(version: '2.13.0', notesFile: 'temp.txt'),
        'gh release create v2.13.0 --notes-file temp.txt --title v2.13.0',
      );
    },
  );

  test('release Cargo lock template path exists', () {
    expect(File(releaseCargoLockTemplatePathForTesting()).existsSync(), true);
  });

  test('release publishes every shared Dart package', () {
    expect(kDartPublishedPackages.map(dartPublishCommand), [
      'cd frb_dart && flutter pub publish --force --server=https://pub.dartlang.org',
      'cd frb_hooks && dart pub publish --force --server=https://pub.dartlang.org',
    ]);
  });

  test('release guard rejects uninitialized submodules', () {
    expect(
      () => verifyReleaseSubmodules(
        submoduleStatus: '-6f7144d frb_codegen/assets/integration_template/cargokit/app/rust_builder/cargokit',
      ),
      throwsA(
        isA<Exception>().having(
          (error) => error.toString(),
          'message',
          contains('git submodule update --init --recursive'),
        ),
      ),
    );
  });

  test('release guard accepts initialized submodules', () {
    expect(
      () => verifyReleaseSubmodules(
        submoduleStatus:
            ' 6f7144d frb_codegen/assets/integration_template/cargokit/app/rust_builder/cargokit (heads/main)\n'
            ' 6f7144d frb_codegen/assets/integration_template/cargokit/plugin/cargokit (heads/main)',
      ),
      returnsNormally,
    );
  });

  test('release guard reports all uninitialized submodules', () {
    expect(
      uninitializedSubmodulePathsForTesting(
        '-6f7144d frb_codegen/assets/integration_template/cargokit/app/rust_builder/cargokit\n'
        ' 6f7144d frb_codegen/assets/integration_template/cargokit/plugin/cargokit\n'
        '-1234567 unrelated/submodule',
      ),
      [
        'frb_codegen/assets/integration_template/cargokit/app/rust_builder/cargokit',
        'unrelated/submodule',
      ],
    );
  });

  test(
    'pure dart generator resolves package from repo root instead of cwd',
    () {
      expect(
        pureDartUriForTesting(repoRootPath: '/workspace/flutter_rust_bridge/')
            .toFilePath(),
        '/workspace/flutter_rust_bridge/frb_example/pure_dart/',
      );
    },
  );

  test('lint ffigen normalization ignores formatting-only differences', () {
    expect(
      normalizeFfigenLintText('''
void fn(
  int value,
) {
  return inner(
    value,
  );
}
      '''),
      normalizeFfigenLintText('''
void fn(int value) {
  return inner(value);
}
      '''),
    );
  });

  test('lint ffigen normalization canonicalizes ffi void function syntax', () {
    expect(
      normalizeFfigenLintText('''
late final ptr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>();
      '''),
      normalizeFfigenLintText('''
late final ptr = _lookup<ffi.NativeFunction<ffi.VoidFunction(ffi.Pointer<ffi.Void>)>>();
      '''),
    );
  });

  test('lint ffigen normalization canonicalizes callback function spacing', () {
    expect(
      normalizeFfigenLintText('''
late final callback = ptr.asFunction<void Function(ffi.Pointer<ffi.Void>)>();
      '''),
      normalizeFfigenLintText('''
late final callback = ptr.asFunction<voidFunction(ffi.Pointer<ffi.Void>)>();
      '''),
    );
  });

  test('integrate Cargo.lock source of truth keeps local crate after flutter_rust_bridge', () {
    for (final (package, crateName) in [
      (
        'frb_example/flutter_via_create/rust/Cargo.lock',
        'rust_lib_flutter_via_create',
      ),
      (
        'frb_example/flutter_via_integrate/rust/Cargo.lock',
        'rust_lib_flutter_via_integrate',
      ),
      (
        'frb_example/flutter_via_create_native_assets/rust/Cargo.lock',
        'rust_lib_flutter_via_create_native_assets',
      ),
      (
        'frb_example/flutter_via_integrate_native_assets/rust/Cargo.lock',
        'rust_lib_flutter_via_integrate_native_assets',
      ),
    ]) {
      final content = File('../../$package').readAsStringSync();
      final localCrateIndex = content.indexOf('name = "$crateName"');
      final frbIndex = content.indexOf('name = "flutter_rust_bridge"');

      expect(localCrateIndex, greaterThanOrEqualTo(0), reason: package);
      expect(frbIndex, greaterThanOrEqualTo(0), reason: package);
      expect(localCrateIndex, greaterThan(frbIndex), reason: package);
    }
  });

  test('resolveBuildWebPackage uses replacement package for flutter package example', () {
    expect(
      resolveBuildWebPackage('frb_example/flutter_package/example'),
      'frb_example/flutter_package',
    );
    expect(
      resolveBuildWebPackage(
        'frb_example/flutter_package_native_assets/example',
      ),
      'frb_example/flutter_package_native_assets',
    );
  });

  test('resolveBuildWebPackage keeps package when no replacement exists', () {
    expect(
      resolveBuildWebPackage('frb_example/gallery'),
      'frb_example/gallery',
    );
  });

  test('book help normalization removes trailing line whitespace', () {
    expect(
      normalizeBookHelpForTesting(
        'line with spaces   \n'
        '          \n'
        'plain\n',
      ),
      '''
line with spaces

plain
''',
    );
  });

  test('build_cli state is restored when internal generation fails', () async {
    final tempDir = await Directory.systemTemp.createTemp('frb_build_cli_');
    addTearDown(() => tempDir.delete(recursive: true));
    final pubspecs = [
      File('${tempDir.path}/frb_dart/pubspec.yaml'),
      File('${tempDir.path}/frb_utils/pubspec.yaml'),
    ];
    const disabledContents = '''
dev_dependencies:
  # Temporarily remove before https://github.com/kevmoo/build_cli/issues/168 is fixed
  # build_cli: ^2.2.5
''';
    for (final pubspec in pubspecs) {
      await pubspec.parent.create(recursive: true);
      await pubspec.writeAsString(disabledContents);
    }

    await expectLater(
      withBuildCliEnabled(
        repoRootPath: tempDir.path,
        action: () async {
          for (final pubspec in pubspecs) {
            expect(
              await pubspec.readAsString(),
              contains('\n  build_cli: ^2.2.5'),
            );
          }
          throw StateError('generation failed');
        },
      ),
      throwsStateError,
    );
    for (final pubspec in pubspecs) {
      expect(await pubspec.readAsString(), disabledContents);
    }
  });

  test('build_cli validates every pubspec before changing files', () async {
    final tempDir = await Directory.systemTemp.createTemp('frb_build_cli_');
    addTearDown(() => tempDir.delete(recursive: true));
    final firstPubspec = File('${tempDir.path}/frb_dart/pubspec.yaml');
    final secondPubspec = File('${tempDir.path}/frb_utils/pubspec.yaml');
    await firstPubspec.parent.create(recursive: true);
    await secondPubspec.parent.create(recursive: true);
    const disabledContents = '''
dev_dependencies:
  # Temporarily remove before https://github.com/kevmoo/build_cli/issues/168 is fixed
  # build_cli: ^2.2.5
''';
    await firstPubspec.writeAsString(disabledContents);
    await secondPubspec.writeAsString('dev_dependencies:\n');

    await expectLater(
      withBuildCliEnabled(repoRootPath: tempDir.path, action: () async {}),
      throwsStateError,
    );
    expect(await firstPubspec.readAsString(), disabledContents);
  });

  test('from-scratch selection keeps every tracked generated output', () {
    expect(
      selectTrackedGeneratedFilesForFromScratchForTesting([
        'frb_example/example/frb_generated.h',
        'frb_example/example/lib/src/rust/frb_generated.dart',
        'frb_example/example/lib/src/rust/api/model.dart',
        'frb_example/example/lib/src/rust/api/model.freezed.dart',
        'frb_example/example/lib/src/rust/third_party/binding.dart',
        'frb_example/example/lib/unrelated_model.g.dart',
        'frb_example/rust_ui/src/frb_generated.rs',
        'frb_rust/src/internal_generated/mod.rs',
        'frb_dart/lib/src/ffigen_generated/multi_package.dart',
        'frb_dart/lib/src/cli/build_web/entrypoint.g.dart',
        'frb_codegen/assets/integration_template/shared/lib/src/rust/frb_generated.dart',
        'frb_codegen/assets/integration_template/shared/lib/model.g.dart',
        'frb_example/example/lib/model.dart',
      ]),
      [
        'frb_example/example/frb_generated.h',
        'frb_example/example/lib/src/rust/frb_generated.dart',
        'frb_example/example/lib/src/rust/api/model.dart',
        'frb_example/example/lib/src/rust/api/model.freezed.dart',
        'frb_example/example/lib/src/rust/third_party/binding.dart',
        'frb_example/example/lib/unrelated_model.g.dart',
        'frb_example/rust_ui/src/frb_generated.rs',
        'frb_rust/src/internal_generated/mod.rs',
        'frb_dart/lib/src/ffigen_generated/multi_package.dart',
        'frb_dart/lib/src/cli/build_web/entrypoint.g.dart',
      ],
    );
  });

  test('from-scratch restoration check reports every missing output', () async {
    final tempDir = await Directory.systemTemp.createTemp(
      'frb_generated_restore_',
    );
    try {
      final restoredFile = File('${tempDir.path}/restored.g.dart');
      await restoredFile.writeAsString('restored');

      expect(
        () => verifyGeneratedFilesRestoredForTesting(
          repoRoot: tempDir.path,
          expectedGeneratedFiles: [
            'restored.g.dart',
            'missing.freezed.dart',
            'rust/src/frb_generated.rs',
          ],
        ),
        throwsA(
          isA<StateError>()
              .having(
                (error) => error.message,
                'message',
                contains('missing.freezed.dart'),
              )
              .having(
                (error) => error.message,
                'message',
                contains('rust/src/frb_generated.rs'),
              ),
        ),
      );
    } finally {
      await tempDir.delete(recursive: true);
    }
  });

  test('pub get guard refreshes stale package config roots', () async {
    final tempDir = await Directory.systemTemp.createTemp('frb_pub_get_guard_');
    try {
      final packageConfig = File(
        '${tempDir.path}/package/.dart_tool/package_config.json',
      );
      await packageConfig.parent.create(recursive: true);
      await packageConfig.writeAsString('''
{
  "configVersion": 2,
  "packages": [
    {
      "name": "missing_lints",
      "rootUri": "file://${tempDir.path}/missing_lints",
      "packageUri": "lib/"
    }
  ]
}
''');

      expect(await shouldRunPubGetForTesting('${tempDir.path}/package'), true);

      await Directory('${tempDir.path}/missing_lints').create();
      expect(await shouldRunPubGetForTesting('${tempDir.path}/package'), false);
    } finally {
      await tempDir.delete(recursive: true);
    }
  });

  test('quickstart smoke OCR normalization ignores punctuation', () {
    expect(
      normalizeQuickstartSmokeOcrTextForTesting(
        'Action: Call Rust `greet("Tom")`\nResult: `Hello, Tom!`',
      ),
      'action call rust greet tom result hello tom',
    );
  });

  test('quickstart smoke OCR accepts hello tom text', () {
    expect(
      () => checkQuickstartSmokeOcrTextForTesting('Result: `Hello, Tom!`'),
      returnsNormally,
    );
  });

  test('quickstart smoke resolves package from repo root instead of cwd', () {
    expect(
      quickstartSmokePackagePathForTesting(
        'frb_example/flutter_via_create',
        repoRootPath: '/workspace/flutter_rust_bridge/',
      ),
      '/workspace/flutter_rust_bridge/frb_example/flutter_via_create',
    );
  });

  test(
    'quickstart smoke waits for Flutter run readiness before screenshot',
    () {
      expect(
        quickstartSmokeFlutterRunIsReadyForTesting(
          'Debug service listening on ws://127.0.0.1:1234/ws',
        ),
        true,
      );
      expect(
        quickstartSmokeFlutterRunIsReadyForTesting('Flutter run key commands.'),
        true,
      );
    },
  );

  test('quickstart smoke gives iOS cold builds more readiness time', () {
    expect(
      quickstartSmokeFlutterRunReadyTimeoutForTesting(
        QuickstartSmokeTarget.ios,
      ),
      const Duration(minutes: 10),
    );
  });

  test('quickstart smoke disables DDS for Android runs', () {
    expect(
      quickstartSmokeFlutterRunArgsForTesting(
        target: QuickstartSmokeTarget.android,
        deviceId: 'emulator-5554',
      ),
      ['run', '-d', 'emulator-5554', '--no-dds'],
    );
  });

  test(
    'quickstart smoke gives macOS desktop cold builds more readiness time',
    () {
      expect(
        quickstartSmokeFlutterRunReadyTimeoutForTesting(
          QuickstartSmokeTarget.desktop,
          isMacOS: true,
        ),
        const Duration(minutes: 10),
      );
      expect(
        quickstartSmokeFlutterRunReadyTimeoutForTesting(
          QuickstartSmokeTarget.desktop,
          isMacOS: false,
        ),
        const Duration(minutes: 5),
      );
    },
  );

  test('quickstart smoke does not capture while Flutter is still building', () {
    expect(
      quickstartSmokeFlutterRunIsReadyForTesting(
        'Running Gradle task \'assembleDebug\'...',
      ),
      false,
    );
    expect(
      quickstartSmokeFlutterRunIsReadyForTesting(
        'Building Windows application...',
      ),
      false,
    );
  });

  test('quickstart smoke detects web worker startup failures', () {
    expect(
      quickstartSmokeOutputFailurePatternForTesting(
        'DataCloneError: Failed to execute postMessage',
      ),
      'DataCloneError',
    );
  });

  test('quickstart smoke ignores unrelated Android graphics warnings', () {
    expect(
      quickstartSmokeOutputFailurePatternForTesting(
        'W/HWUI: Failed to initialize 101010-2 format, error = EGL_SUCCESS',
      ),
      isNull,
    );
  });

  test('quickstart smoke OCR rejects unrelated text', () {
    expect(
      () => checkQuickstartSmokeOcrTextForTesting(
        'Failed to initialize the application',
      ),
      throwsA(
        isA<Exception>().having(
          (exception) => exception.toString(),
          'message',
          contains('Hello, Tom'),
        ),
      ),
    );
  });

  group('git clean check', () {
    test('classifies git diff exit codes', () {
      expect(classifyGitDiffExitCodeForTesting(0), 'clean');
      expect(classifyGitDiffExitCodeForTesting(1), 'dirty');
      expect(classifyGitDiffExitCodeForTesting(128), 'unavailable');
    });

    test('detects CI from common environment variables', () {
      expect(isCiForTesting({'GITHUB_ACTIONS': 'true'}), true);
      expect(isCiForTesting({'CI': 'true'}), true);
      expect(isCiForTesting({'CI': '1'}), true);
      expect(isCiForTesting({'CI': 'false'}), false);
      expect(isCiForTesting({'CI': '0'}), false);
      expect(isCiForTesting({}), false);
    });

    test('decides clean git diff should continue silently', () {
      expect(
        () => handleGitDiffResultForTesting(
          exitCode: 0,
          isBefore: false,
          isCi: false,
        ),
        returnsNormally,
      );
    });

    test('warns when working tree is already dirty before command', () {
      expect(
        () => handleGitDiffResultForTesting(
          exitCode: 1,
          isBefore: true,
          isCi: false,
        ),
        prints(contains('working tree is already dirty')),
      );
    });

    test('fails when working tree changed after command', () {
      expect(
        () => handleGitDiffResultForTesting(
          exitCode: 1,
          isBefore: false,
          isCi: false,
        ),
        throwsA(
          isA<Exception>().having(
            (exception) => exception.toString(),
            'message',
            contains('Working tree changed'),
          ),
        ),
      );
    });

    test('warns when git metadata is unavailable outside CI', () {
      expect(
        () => handleGitDiffResultForTesting(
          exitCode: 128,
          isBefore: false,
          isCi: false,
        ),
        prints(contains('git metadata is unavailable')),
      );
    });

    test('fails when git metadata is unavailable in CI', () {
      expect(
        () => handleGitDiffResultForTesting(
          exitCode: 128,
          isBefore: false,
          isCi: true,
        ),
        throwsA(
          isA<Exception>().having(
            (exception) => exception.toString(),
            'message',
            contains('Failed to check working tree cleanliness'),
          ),
        ),
      );
    });
  });

  group('release version check', () {
    test('parses crates.io package metadata', () {
      expect(
        parseCratesIoReleasedVersion({
          'crate': {'max_version': '2.12.0'},
        }),
        '2.12.0',
      );
    });

    test('parses pub.dev package metadata', () {
      expect(
        parsePubDevReleasedVersion({
          'latest': {'version': '2.12.0'},
        }),
        '2.12.0',
      );
    });

    test('finds pub.dev prerelease target version outside latest', () {
      expect(
        parsePubDevReleasedVersion({
          'latest': {'version': '2.12.0'},
          'versions': [
            {'version': '2.12.0'},
            {'version': '2.13.0-beta.1'},
          ],
        }, targetVersion: '2.13.0-beta.1'),
        '2.13.0-beta.1',
      );
    });

    test('summarizes whether every package is published', () {
      final output = buildReleasePackageStatusOutput([
        const ReleasePackageStatus(
          registry: 'crates.io',
          name: 'flutter_rust_bridge',
          manifestVersion: '2.12.0',
          releasedVersion: '2.12.0',
        ),
        const ReleasePackageStatus(
          registry: 'pub.dev',
          name: 'flutter_rust_bridge',
          manifestVersion: '2.12.0',
          releasedVersion: '2.11.1',
        ),
      ]);

      expect(output['allReleased'], false);
      expect(output['packages'], [
        {
          'registry': 'crates.io',
          'name': 'flutter_rust_bridge',
          'manifestVersion': '2.12.0',
          'releasedVersion': '2.12.0',
          'isReleased': true,
        },
        {
          'registry': 'pub.dev',
          'name': 'flutter_rust_bridge',
          'manifestVersion': '2.12.0',
          'releasedVersion': '2.11.1',
          'isReleased': false,
        },
      ]);
    });

    test('uses explicit target version for every published package', () async {
      final statuses = await fetchReleasePackageStatuses(
        targetVersion: '9.9.9',
        fetcher: (uri) async {
          if (uri.host == 'crates.io') {
            return {
              'crate': {'max_version': '9.9.9'},
            };
          }
          return {
            'latest': {'version': '9.9.9'},
            'versions': [
              {'version': '9.9.9'},
            ],
          };
        },
      );

      expect(
        statuses.map((status) => status.manifestVersion),
        everyElement('9.9.9'),
      );
      expect(statuses.map((status) => (status.registry, status.name)), [
        ('crates.io', 'flutter_rust_bridge_codegen'),
        ('crates.io', 'flutter_rust_bridge_macros'),
        ('crates.io', 'flutter_rust_bridge'),
        ('pub.dev', 'flutter_rust_bridge'),
        ('pub.dev', 'flutter_rust_bridge_hooks'),
      ]);
      expect(statuses.map((status) => status.isReleased), everyElement(true));
    });

    test(
      'reports hooks as unreleased when its target version is absent',
      () async {
        final statuses = await fetchReleasePackageStatuses(
          targetVersion: '9.9.9',
          fetcher: (uri) async {
            if (uri.host == 'crates.io') {
              return {
                'crate': {'max_version': '9.9.9'},
              };
            }
            if (uri.path.endsWith('/flutter_rust_bridge_hooks')) {
              return {'versions': <Map<String, String>>[]};
            }
            return {
              'versions': [
                {'version': '9.9.9'},
              ],
            };
          },
        );

        final output = buildReleasePackageStatusOutput(statuses);
        final hooksStatus = statuses.singleWhere(
          (status) => status.name == 'flutter_rust_bridge_hooks',
        );
        expect(output['allReleased'], false);
        expect(hooksStatus.releasedVersion, isNull);
        expect(hooksStatus.isReleased, false);
      },
    );

    test(
      'reports hooks as unreleased when only another version exists',
      () async {
        final statuses = await fetchReleasePackageStatuses(
          targetVersion: '9.9.9',
          fetcher: (uri) async {
            if (uri.host == 'crates.io') {
              return {
                'crate': {'max_version': '9.9.9'},
              };
            }
            final version = uri.path.endsWith('/flutter_rust_bridge_hooks')
                ? '9.9.8'
                : '9.9.9';
            return {
              'latest': {'version': version},
              'versions': [
                {'version': version},
              ],
            };
          },
        );

        final output = buildReleasePackageStatusOutput(statuses);
        final hooksStatus = statuses.singleWhere(
          (status) => status.name == 'flutter_rust_bridge_hooks',
        );
        expect(output['allReleased'], false);
        expect(hooksStatus.releasedVersion, '9.9.8');
        expect(hooksStatus.isReleased, false);
      },
    );

    test(
      'uses each local Dart manifest version as its pub.dev target',
      () async {
        final rustVersion = getWorkspaceRustVersion();

        final statuses = await fetchReleasePackageStatuses(
          dartPackageManifestFetcher: (package) => switch (package) {
            'frb_dart' =>
              '''
name: flutter_rust_bridge
version: 9.9.9
''',
            'frb_hooks' =>
              '''
name: flutter_rust_bridge_hooks
version: 9.9.8
''',
            _ => throw StateError('Unexpected Dart package: $package'),
          },
          fetcher: (uri) async {
            if (uri.host == 'crates.io') {
              return {
                'crate': {'max_version': rustVersion},
              };
            }
            final version = uri.path.endsWith('/flutter_rust_bridge_hooks')
                ? '9.9.8'
                : '9.9.9';
            return {
              'versions': [
                {'version': version},
              ],
            };
          },
        );

        final pubDevStatuses = statuses.where(
          (status) => status.registry == 'pub.dev',
        );
        expect(pubDevStatuses, hasLength(2));
        expect(
          pubDevStatuses.map((status) => (status.name, status.manifestVersion)),
          [
            ('flutter_rust_bridge', '9.9.9'),
            ('flutter_rust_bridge_hooks', '9.9.8'),
          ],
        );
        expect(
          pubDevStatuses.map((status) => status.isReleased),
          everyElement(true),
        );
      },
    );
  });

  group('post-release config', () {
    test('uses stable constraint without fetching crates.io', () async {
      final requirement = await resolveCodegenVersionRequirement(
        ReleaseChannel.stable,
        fetcher: (_) => throw StateError('should not fetch'),
      );

      expect(requirement, '^2.0.0');
    });

    test('uses latest unstable exact constraint from crates.io', () async {
      final requirement = await resolveCodegenVersionRequirement(
        ReleaseChannel.unstable,
        fetcher: (_) async => {
          'crate': {'max_stable_version': '2.12.0'},
          'versions': [
            {'num': '2.14.0-beta.1', 'yanked': true},
            {'num': '2.13.0-alpha.1', 'yanked': false},
            {'num': '2.13.0-beta.1', 'yanked': false},
            {'num': '2.12.0', 'yanked': false},
          ],
        },
      );

      expect(requirement, '=2.13.0-beta.1');
    });

    test('skips unstable channel when only old prereleases exist', () async {
      final requirement = await resolveCodegenVersionRequirement(
        ReleaseChannel.unstable,
        fetcher: (_) async => {
          'crate': {'max_stable_version': '2.12.0'},
          'versions': [
            {'num': '2.12.0', 'yanked': false},
            {'num': '2.0.0-dev.42', 'yanked': false},
          ],
        },
      );

      expect(requirement, isNull);
    });

    test('parses release channel from CLI arguments', () {
      final config = parsePostReleaseConfig([
        '--codegen-install-mode',
        'cargo-install',
        '--release-channel',
        'unstable',
        '--integration-backend',
        'native-assets',
      ]);

      expect(config.codegenInstallMode, CodegenInstallMode.cargoInstall);
      expect(config.releaseChannel, ReleaseChannel.unstable);
      expect(config.integrationBackend, IntegrateExampleBackend.nativeAssets);
    });

    test('defaults post-release backend to cargokit', () {
      final config = parsePostReleaseConfig([
        '--codegen-install-mode',
        'cargo-install',
        '--release-channel',
        'unstable',
      ]);

      expect(config.integrationBackend, IntegrateExampleBackend.cargokit);
    });

    test('parses mimic quickstart backend from CLI arguments', () {
      final config = parseTestMimicQuickstartConfig([
        '--integration-backend',
        'native-assets',
      ]);

      expect(config.integrationBackend, IntegrateExampleBackend.nativeAssets);
    });

    test('defaults mimic quickstart backend to cargokit', () {
      final config = parseTestMimicQuickstartConfig([]);

      expect(config.integrationBackend, IntegrateExampleBackend.cargokit);
    });
  });

  group('test checkValgrindOutput', () {
    test('accepts a clean run', () {
      checkValgrindOutput(
        stdout: '00:00 +1: All tests passed!',
        stderr: '''
==3667== LEAK SUMMARY:
==3667==    definitely lost: 0 bytes in 0 blocks
==3667==    indirectly lost: 0 bytes in 0 blocks
==3667==      possibly lost: 1,216 bytes in 4 blocks
==3667==    still reachable: 16,530 bytes in 202 blocks
==3667==         suppressed: 0 bytes in 0 blocks
==3667== Reachable blocks (those to which a pointer was found) are not shown.
==3667== To see them, rerun with: --leak-check=full --show-leak-kinds=all
    ''',
        exitCode: 0,
      );
    });

    test('rejects an error reported only on stderr', () {
      expect(
        () => checkValgrindOutput(
          stdout: '00:00 +1: All tests passed!',
          stderr: '''
==3667== LEAK SUMMARY:
==3667==    definitely lost: 4 bytes in 0 blocks
==3667==    indirectly lost: 0 bytes in 0 blocks
==3667==      possibly lost: 1,216 bytes in 4 blocks
==3667==    still reachable: 16,530 bytes in 202 blocks
==3667==         suppressed: 0 bytes in 0 blocks
==3667== Reachable blocks (those to which a pointer was found) are not shown.
==3667== To see them, rerun with: --leak-check=full --show-leak-kinds=all
    ''',
          exitCode: 0,
        ),
        throwsA(isA<Exception>()),
      );
    });

    test('rejects an error reported only on stdout', () {
      expect(
        () => checkValgrindOutput(
          stdout: '''
00:00 +1: All tests passed!
==3667== LEAK SUMMARY:
==3667==    definitely lost: 4 bytes in 0 blocks
==3667==    indirectly lost: 0 bytes in 0 blocks
    ''',
          stderr: '',
          exitCode: 0,
        ),
        throwsA(isA<Exception>()),
      );
    });

    test('rejects indirectly lost bytes', () {
      expect(
        () => checkValgrindOutput(
          stdout: '00:00 +1: All tests passed!',
          stderr: '''
==3667== LEAK SUMMARY:
==3667==    definitely lost: 0 bytes in 0 blocks
==3667==    indirectly lost: 4 bytes in 0 blocks
==3667==      possibly lost: 1,216 bytes in 4 blocks
==3667==    still reachable: 16,530 bytes in 202 blocks
==3667==         suppressed: 0 bytes in 0 blocks
==3667== Reachable blocks (those to which a pointer was found) are not shown.
==3667== To see them, rerun with: --leak-check=full --show-leak-kinds=all
    ''',
          exitCode: 0,
        ),
        throwsA(isA<Exception>()),
      );
    });

    test('rejects a nonzero exit code', () {
      expect(
        () => checkValgrindOutput(
          stdout: '00:00 +1: All tests passed!',
          stderr: '',
          exitCode: 1,
        ),
        throwsA(isA<Exception>()),
      );
    });

    test('rejects output without the test success marker', () {
      expect(
        () => checkValgrindOutput(
          stdout: '',
          stderr: '''
==3667== LEAK SUMMARY:
==3667==    definitely lost: 0 bytes in 0 blocks
==3667==    indirectly lost: 0 bytes in 0 blocks
==3667==      possibly lost: 1,216 bytes in 4 blocks
==3667==    still reachable: 16,530 bytes in 202 blocks
==3667==         suppressed: 0 bytes in 0 blocks
==3667== Reachable blocks (those to which a pointer was found) are not shown.
==3667== To see them, rerun with: --leak-check=full --show-leak-kinds=all
    ''',
          exitCode: 0,
        ),
        throwsA(isA<Exception>()),
      );
    });
  });
}
