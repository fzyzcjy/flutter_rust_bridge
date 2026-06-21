import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/build.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/lint.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/post_release.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/quickstart_smoke.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/released_version.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester.dart';
import 'package:test/test.dart';

void main() {
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

  test('sanitizer env keeps TSAN on the normal thread pool', () async {
    final runtimeEnv = await sanitizerRuntimeEnvForTesting(Sanitizer.tsan);

    expect(runtimeEnv, isNot(contains('FRB_SYNC_THREAD_POOL')));
    expect(runtimeEnv['TSAN_OPTIONS'], contains('report_thread_leaks=0'));
    expect(runtimeEnv['TSAN_OPTIONS'], contains('suppressions='));
  });

  test('sanitizer rustflags disable ASAN use-after-scope false positives', () {
    expect(
      sanitizerRustflagsForTesting(
        Sanitizer.asan,
        package: 'frb_example/dart_minimal',
      ),
      contains('-Cllvm-args=-asan-use-after-scope=0'),
    );
  });

  test('sanitizer rustflags keep ASAN stack sentinel coverage', () {
    expect(
      sanitizerRustflagsForTesting(
        Sanitizer.asan,
        package: 'frb_example/pure_dart',
      ),
      contains('-Cllvm-args=-asan-stack=0'),
    );
    expect(
      sanitizerRustflagsForTesting(
        Sanitizer.asan,
        package: 'frb_example/deliberate_bad',
      ),
      isNot(contains('-Cllvm-args=-asan-stack=0')),
    );
  });

  test('sanitizer rustflags normalize Rust package working directories', () {
    expect(
      packageForRustflagsForTesting('frb_example/pure_dart_pde/rust'),
      'frb_example/pure_dart_pde',
    );
  });

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

  test(
    'pure dart generator resolves package from repo root instead of cwd',
    () {
      expect(
        pureDartUriForTesting(
          repoRootPath: '/workspace/flutter_rust_bridge/',
        ).toFilePath(),
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

  test(
    'integrate Cargo.lock source of truth keeps local crate after flutter_rust_bridge',
    () {
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
    },
  );

  test(
    'resolveBuildWebPackage uses replacement package for flutter package example',
    () {
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
    },
  );

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

  test('quickstart web smoke enables browser shared buffers', () {
    expect(
      quickstartSmokeFlutterRunArgsForTesting(
        target: QuickstartSmokeTarget.web,
        deviceId: 'chrome',
      ),
      containsAllInOrder([
        'run',
        '-d',
        'chrome',
        '--web-header=Cross-Origin-Opener-Policy=same-origin',
        '--web-header=Cross-Origin-Embedder-Policy=require-corp',
        '--web-browser-flag=--enable-features=SharedArrayBuffer',
      ]),
    );
  });

  test(
    'quickstart smoke gives cold desktop and iOS builds more readiness time',
    () {
      expect(
        quickstartSmokeFlutterRunReadyTimeoutForTesting(
          QuickstartSmokeTarget.desktop,
        ),
        const Duration(minutes: 10),
      );
      expect(
        quickstartSmokeFlutterRunReadyTimeoutForTesting(
          QuickstartSmokeTarget.ios,
        ),
        const Duration(minutes: 10),
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

  test('quickstart smoke allows web worker pool fallback diagnostics', () {
    expect(
      quickstartSmokeOutputFailurePatternForTesting('''
Failed to initialize web worker pool: JsValue(DataCloneError: Failed to execute 'postMessage' on 'Worker': #<Memory> could not be cloned.
DataCloneError: Failed to execute 'postMessage' on 'Worker': #<Memory> could not be cloned.
'''),
      isNull,
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
      expect(statuses.map((status) => status.isReleased), everyElement(true));
    });

    test(
      'uses local Dart manifest version as pub.dev target version',
      () async {
        final rustVersion = getWorkspaceRustVersion();
        final dartVersion = getFrbDartVersion();

        final statuses = await fetchReleasePackageStatuses(
          fetcher: (uri) async {
            if (uri.host == 'crates.io') {
              return {
                'crate': {'max_version': rustVersion},
              };
            }
            return {
              'latest': {'version': '2.12.0'},
              'versions': [
                {'version': dartVersion},
              ],
            };
          },
        );

        final pubDevStatus = statuses.singleWhere(
          (status) => status.registry == 'pub.dev',
        );
        expect(pubDevStatus.releasedVersion, dartVersion);
        expect(pubDevStatus.isReleased, true);
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
      ]);

      expect(config.codegenInstallMode, CodegenInstallMode.cargoInstall);
      expect(config.releaseChannel, ReleaseChannel.unstable);
    });
  });

  group('test checkValgrindOutput', () {
    test('good', () {
      checkValgrindOutput('''
00:00 +1: All tests passed!

==3667== LEAK SUMMARY:
==3667==    definitely lost: 0 bytes in 0 blocks
==3667==    indirectly lost: 0 bytes in 0 blocks
==3667==      possibly lost: 1,216 bytes in 4 blocks
==3667==    still reachable: 16,530 bytes in 202 blocks
==3667==         suppressed: 0 bytes in 0 blocks
==3667== Reachable blocks (those to which a pointer was found) are not shown.
==3667== To see them, rerun with: --leak-check=full --show-leak-kinds=all
    ''');
    });

    test('some dart tests failed', () {
      // no "All tests passed!" line
      expect(
        () => checkValgrindOutput('''
==3667== LEAK SUMMARY:
==3667==    definitely lost: 0 bytes in 0 blocks
==3667==    indirectly lost: 0 bytes in 0 blocks
==3667==      possibly lost: 1,216 bytes in 4 blocks
==3667==    still reachable: 16,530 bytes in 202 blocks
==3667==         suppressed: 0 bytes in 0 blocks
==3667== Reachable blocks (those to which a pointer was found) are not shown.
==3667== To see them, rerun with: --leak-check=full --show-leak-kinds=all
    '''),
        throwsA(isA<Exception>()),
      );
    });

    test('has definitely lost bytes', () {
      expect(
        () => checkValgrindOutput('''
00:00 +1: All tests passed!

==3667== LEAK SUMMARY:
==3667==    definitely lost: 4 bytes in 0 blocks
==3667==    indirectly lost: 0 bytes in 0 blocks
==3667==      possibly lost: 1,216 bytes in 4 blocks
==3667==    still reachable: 16,530 bytes in 202 blocks
==3667==         suppressed: 0 bytes in 0 blocks
==3667== Reachable blocks (those to which a pointer was found) are not shown.
==3667== To see them, rerun with: --leak-check=full --show-leak-kinds=all
    '''),
        throwsA(isA<Exception>()),
      );
    });

    test('has indirectly lost bytes', () {
      expect(
        () => checkValgrindOutput('''
00:00 +1: All tests passed!

==3667== LEAK SUMMARY:
==3667==    definitely lost: 0 bytes in 0 blocks
==3667==    indirectly lost: 4 bytes in 0 blocks
==3667==      possibly lost: 1,216 bytes in 4 blocks
==3667==    still reachable: 16,530 bytes in 202 blocks
==3667==         suppressed: 0 bytes in 0 blocks
==3667== Reachable blocks (those to which a pointer was found) are not shown.
==3667== To see them, rerun with: --leak-check=full --show-leak-kinds=all
    '''),
        throwsA(isA<Exception>()),
      );
    });
  });
}
