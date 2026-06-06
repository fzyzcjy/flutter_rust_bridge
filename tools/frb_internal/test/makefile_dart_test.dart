import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/ci_plan.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/lint.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:test/test.dart';

void main() {
  test('dart valgrind compile command uses dart build output directory', () {
    expect(
      dartValgrindCompileCommandForTesting(),
      'dart build cli -t test/dart_valgrind_test_entrypoint.dart -o build/valgrind_test_output/',
    );
  });

  test('CI plan full filter enables every job and full matrices', () {
    final plan = buildCiPlan(filter: 'full', automaticCiDisabled: false);

    expect(plan.enabledJobs, containsAll(kCiJobs.map((job) => job.id)));
    expect(plan.matrixByJob['test_dart_web'], {
      'include': [
        {'package': 'frb_dart'},
        {'package': 'frb_example--dart_minimal'},
        {'package': 'frb_example--pure_dart'},
        {'package': 'frb_example--pure_dart_pde'},
      ],
    });
  });

  test('CI plan filter selects one matrix entry', () {
    final plan = buildCiPlan(
      filter: 'test_dart_web[package=frb_example--pure_dart_pde]',
      automaticCiDisabled: false,
    );

    expect(plan.enabledJobs, {'test_dart_web'});
    expect(plan.matrixByJob['test_dart_web'], {
      'include': [
        {'package': 'frb_example--pure_dart_pde'},
      ],
    });
  });

  test('CI plan filter supports nested matrix dimensions and OR values', () {
    final plan = buildCiPlan(
      filter:
          'test_flutter_native_desktop[platform=linux,package=frb_example--gallery|frb_example--flutter_via_integrate]',
      automaticCiDisabled: false,
    );

    expect(plan.enabledJobs, {'test_flutter_native_desktop'});
    expect(plan.matrixByJob['test_flutter_native_desktop'], {
      'include': [
        {
          'info': {
            'image': 'ubuntu-latest',
            'platform': 'linux',
            'package': 'frb_example--flutter_via_integrate',
          },
        },
        {
          'info': {
            'image': 'ubuntu-latest',
            'platform': 'linux',
            'package': 'frb_example--gallery',
          },
        },
      ],
    });
  });

  test('CI plan automatic disable suppresses all jobs', () {
    final plan = buildCiPlan(filter: 'full', automaticCiDisabled: true);

    expect(plan.enabledJobs, isEmpty);
    expect(plan.matrixByJob['test_dart_web'], {'include': []});
  });

  test('CI plan rejects unknown matrix dimensions', () {
    expect(
      () => buildCiPlan(
        filter: 'test_dart_web[image=ubuntu-latest]',
        automaticCiDisabled: false,
      ),
      throwsFormatException,
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
    },
  );

  test('resolveBuildWebPackage keeps package when no replacement exists', () {
    expect(
      resolveBuildWebPackage('frb_example/gallery'),
      'frb_example/gallery',
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
