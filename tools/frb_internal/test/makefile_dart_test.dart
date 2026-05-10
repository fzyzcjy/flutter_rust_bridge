import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart';
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

  group('git clean check', () {
    test('classifies git diff exit codes', () {
      expect(classifyGitDiffExitCodeForTesting(0), 'clean');
      expect(classifyGitDiffExitCodeForTesting(1), 'dirty');
      expect(classifyGitDiffExitCodeForTesting(128), 'unavailable');
    });

    test('continues when git metadata is unavailable outside CI', () async {
      var innerDidRun = false;
      var checkCount = 0;

      await wrapMaybeSetExitIfChangedRaw(
        true,
        () async {
          innerDidRun = true;
        },
        gitDiffExecutor: (_) async {
          checkCount++;
          return const RunCommandOutput(stdout: '', stderr: '', exitCode: 128);
        },
        isCi: false,
      );

      expect(innerDidRun, true);
      expect(checkCount, 2);
    });

    test('fails when git metadata is unavailable in CI', () async {
      var innerDidRun = false;

      await expectLater(
        wrapMaybeSetExitIfChangedRaw(
          true,
          () async {
            innerDidRun = true;
          },
          gitDiffExecutor: (_) async {
            return const RunCommandOutput(
              stdout: '',
              stderr: '',
              exitCode: 128,
            );
          },
          isCi: true,
        ),
        throwsA(isA<ProcessException>()),
      );

      expect(innerDidRun, false);
    });

    test(
      'warns on pre-existing dirty tree but still fails after dirty result',
      () async {
        var innerDidRun = false;
        var checkCount = 0;

        await expectLater(
          wrapMaybeSetExitIfChangedRaw(
            true,
            () async {
              innerDidRun = true;
            },
            gitDiffExecutor: (_) async {
              checkCount++;
              return const RunCommandOutput(
                stdout: '',
                stderr: '',
                exitCode: 1,
              );
            },
            isCi: false,
          ),
          throwsA(isA<ProcessException>()),
        );

        expect(innerDidRun, true);
        expect(checkCount, 2);
      },
    );
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
