import 'dart:io';

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

  test('integrate preserve policy is explicit for flutter_via_create', () {
    expect(
      integratePreservedRelativePathsForTesting(
        'frb_example/flutter_via_create',
      ),
      ['ios', 'macos/Podfile'],
    );
  });

  test('integrate preserve policy is explicit for flutter_via_integrate', () {
    expect(
      integratePreservedRelativePathsForTesting(
        'frb_example/flutter_via_integrate',
      ),
      ['ios', 'macos/Podfile'],
    );
  });

  test('integrate preserve policy is explicit for flutter_package', () {
    expect(
      integratePreservedRelativePathsForTesting('frb_example/flutter_package'),
      ['.metadata', 'example/ios', 'example/macos/Podfile'],
    );
  });

  test('integrate preserve policy is empty for unrelated package', () {
    expect(
      integratePreservedRelativePathsForTesting('frb_example/gallery'),
      isEmpty,
    );
  });

  test('integrate extra args are explicit for flutter_via_create', () {
    expect(
      integrateSetExitIfChangedExtraArgsForTesting(
        'frb_example/flutter_via_create',
      ),
      "':(exclude)frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig' "
      "':(exclude)frb_example/flutter_via_create/rust/Cargo.lock'",
    );
  });

  test('integrate extra args are explicit for flutter_via_integrate', () {
    expect(
      integrateSetExitIfChangedExtraArgsForTesting(
        'frb_example/flutter_via_integrate',
      ),
      "':(exclude)frb_example/flutter_via_integrate/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_via_integrate/macos/Flutter/Flutter-Release.xcconfig' "
      "':(exclude)frb_example/flutter_via_integrate/rust/Cargo.lock'",
    );
  });

  test('integrate extra args are explicit for flutter_package', () {
    expect(
      integrateSetExitIfChangedExtraArgsForTesting(
        'frb_example/flutter_package',
      ),
      "':(exclude)frb_example/flutter_package/example/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_package/example/macos/Flutter/Flutter-Release.xcconfig' "
      "':(exclude)frb_example/flutter_package/rust/Cargo.lock'",
    );
  });

  test('integrate extra args are empty for unrelated package', () {
    expect(
      integrateSetExitIfChangedExtraArgsForTesting('frb_example/gallery'),
      isEmpty,
    );
  });

  test('copyDirectoryRecursive preserves dotfiles and nested workspace files', () {
    final tempDir = Directory.systemTemp.createTempSync('frb-copy-recursive-');
    addTearDown(() => tempDir.deleteSync(recursive: true));

    final source = Directory('${tempDir.path}/source');
    final destination = Directory('${tempDir.path}/destination');
    Directory(
      '${source.path}/.xcodeproj/project.xcworkspace/xcshareddata',
    ).createSync(recursive: true);
    File('${source.path}/.gitignore').writeAsStringSync('DerivedData\n');
    File(
      '${source.path}/.xcodeproj/project.xcworkspace/xcshareddata/WorkspaceSettings.xcsettings',
    ).writeAsStringSync('<plist/>');

    copyDirectoryRecursiveForTesting(source: source, destination: destination);

    expect(
      File('${destination.path}/.gitignore').readAsStringSync(),
      'DerivedData\n',
    );
    expect(
      File(
        '${destination.path}/.xcodeproj/project.xcworkspace/xcshareddata/WorkspaceSettings.xcsettings',
      ).readAsStringSync(),
      '<plist/>',
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
