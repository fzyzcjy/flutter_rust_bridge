import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/integrate_apple_scaffold.dart';
import 'package:test/test.dart';

void main() {
  test(
    'integrate apple scaffold source of truth is explicit for flutter_via_create',
    () {
      expect(
        integrateAppleScaffoldSourceOfTruthPathsForTesting(
          'frb_example/flutter_via_create',
        ),
        ['.metadata', 'ios', 'macos/Podfile'],
      );

      expect(
        integrateAppleScaffoldSourceOfTruthAssetPathsForTesting(
          repoRootPath: '/workspace/flutter_rust_bridge/',
          package: 'frb_example/flutter_via_create',
        ),
        [
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_via_create/.metadata',
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_via_create/ios',
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_via_create/macos/Podfile',
        ],
      );
    },
  );

  test(
    'integrate apple scaffold source of truth is explicit for flutter_via_integrate',
    () {
      expect(
        integrateAppleScaffoldSourceOfTruthPathsForTesting(
          'frb_example/flutter_via_integrate',
        ),
        ['.metadata', 'ios', 'macos/Podfile'],
      );

      expect(
        integrateAppleScaffoldSourceOfTruthAssetPathsForTesting(
          repoRootPath: '/workspace/flutter_rust_bridge/',
          package: 'frb_example/flutter_via_integrate',
        ),
        [
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_via_integrate/.metadata',
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_via_integrate/ios',
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_via_integrate/macos/Podfile',
        ],
      );
    },
  );

  test('integrate apple scaffold source of truth is explicit for flutter_package', () {
    expect(
      integrateAppleScaffoldSourceOfTruthPathsForTesting(
        'frb_example/flutter_package',
      ),
      ['.metadata', 'pubspec.yaml', 'example/ios', 'example/macos/Podfile'],
    );

    expect(
      integrateAppleScaffoldSourceOfTruthAssetPathsForTesting(
        repoRootPath: '/workspace/flutter_rust_bridge/',
        package: 'frb_example/flutter_package',
      ),
      [
        '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_package/.metadata',
        '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_package/pubspec.yaml',
        '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_package/example/ios',
        '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/frb_example/flutter_package/example/macos/Podfile',
      ],
    );
  });

  test(
    'integrate apple scaffold source of truth is empty for unrelated package',
    () {
      expect(
        integrateAppleScaffoldSourceOfTruthPathsForTesting(
          'frb_example/gallery',
        ),
        isEmpty,
      );

      expect(
        integrateAppleScaffoldSourceOfTruthAssetPathsForTesting(
          repoRootPath: '/workspace/flutter_rust_bridge/',
          package: 'frb_example/gallery',
        ),
        isEmpty,
      );
    },
  );

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
}
