import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/integrate_apple_scaffold.dart';
import 'package:test/test.dart';

void main() {
  test('integrate apple scaffold packages are explicit', () {
    expect(integrateAppleScaffoldSourceOfTruthPackages(), [
      'frb_example/flutter_via_create',
      'frb_example/flutter_via_create_native_assets',
      'frb_example/flutter_via_integrate',
      'frb_example/flutter_via_integrate_native_assets',
      'frb_example/flutter_package',
      'frb_example/flutter_package_native_assets',
    ]);
  });

  test('integrate apple scaffold generation does not compare OHOS', () {
    final config = generateAppleScaffoldPackageConfigForTesting(
      'frb_example/flutter_via_create',
    );

    expect(config.setExitIfChanged, isFalse);
    expect(config.package, 'frb_example/flutter_via_create');
    expect(config.coverage, isFalse);
    expect(config.includeOhos, isFalse);
    expect(config.skipCheckedInAppleScaffold, isTrue);
  });

  test(
    'integrate apple scaffold source of truth is explicit for flutter_via_create',
    () {
      for (final package in [
        'frb_example/flutter_via_create',
        'frb_example/flutter_via_create_native_assets',
      ]) {
        expect(
          integrateAppleScaffoldSourceOfTruthPathsForTesting(package),
          ['.metadata', 'ios', 'macos/Podfile'],
          reason: package,
        );

        expect(
          integrateAppleScaffoldSourceOfTruthAssetPathsForTesting(
            repoRootPath: '/workspace/flutter_rust_bridge/',
            package: package,
          ),
          [
            '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/.metadata',
            '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/ios',
            '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/macos/Podfile',
          ],
          reason: package,
        );
      }
    },
  );

  test(
    'integrate apple scaffold source of truth is explicit for flutter_via_integrate',
    () {
      for (final package in [
        'frb_example/flutter_via_integrate',
        'frb_example/flutter_via_integrate_native_assets',
      ]) {
        expect(
          integrateAppleScaffoldSourceOfTruthPathsForTesting(package),
          ['.metadata', 'ios', 'macos/Podfile'],
          reason: package,
        );

        expect(
          integrateAppleScaffoldSourceOfTruthAssetPathsForTesting(
            repoRootPath: '/workspace/flutter_rust_bridge/',
            package: package,
          ),
          [
            '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/.metadata',
            '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/ios',
            '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/macos/Podfile',
          ],
          reason: package,
        );
      }
    },
  );

  test('integrate apple scaffold source of truth is explicit for flutter_package', () {
    for (final package in [
      'frb_example/flutter_package',
      'frb_example/flutter_package_native_assets',
    ]) {
      expect(
        integrateAppleScaffoldSourceOfTruthPathsForTesting(package),
        ['.metadata', 'pubspec.yaml', 'example/ios', 'example/macos/Podfile'],
        reason: package,
      );

      expect(
        integrateAppleScaffoldSourceOfTruthAssetPathsForTesting(
          repoRootPath: '/workspace/flutter_rust_bridge/',
          package: package,
        ),
        [
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/.metadata',
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/pubspec.yaml',
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/example/ios',
          '/workspace/flutter_rust_bridge/tools/frb_internal/assets/apple_scaffold/$package/example/macos/Podfile',
        ],
        reason: package,
      );
    }
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

  test('preserved OHOS scaffold paths are explicit', () {
    expect(
      preservedOhosScaffoldPathsForTesting('frb_example/flutter_via_create'),
      ['ohos', 'rust_builder/ohos', 'rust_builder/pubspec.yaml'],
    );
    expect(
      preservedOhosScaffoldPathsForTesting(
        'frb_example/flutter_via_create_native_assets',
      ),
      ['ohos'],
    );
    expect(
      preservedOhosScaffoldPathsForTesting('frb_example/flutter_package'),
      isEmpty,
    );
  });

  test(
    'preserveCheckedInOhosScaffold restores files and directories',
    () async {
      final tempDir = Directory.systemTemp.createTempSync('frb-preserve-ohos-');
      addTearDown(() => tempDir.deleteSync(recursive: true));

      final original = Directory('${tempDir.path}/original');
      final generated = Directory('${tempDir.path}/generated');
      Directory('${original.path}/ohos').createSync(recursive: true);
      Directory(
        '${original.path}/rust_builder/ohos',
      ).createSync(recursive: true);
      File('${original.path}/ohos/marker.txt').writeAsStringSync('root-ohos');
      File(
        '${original.path}/rust_builder/ohos/marker.txt',
      ).writeAsStringSync('builder-ohos');
      File(
        '${original.path}/rust_builder/pubspec.yaml',
      ).writeAsStringSync('plugin:\n  platforms:\n    ohos:\n');
      Directory(generated.path).createSync(recursive: true);

      await preserveCheckedInOhosScaffold(
        package: 'frb_example/flutter_via_create',
        originalPackageDir: original.path,
        generatedPackageDir: generated.path,
      );

      expect(
        File('${generated.path}/ohos/marker.txt').readAsStringSync(),
        'root-ohos',
      );
      expect(
        File(
          '${generated.path}/rust_builder/ohos/marker.txt',
        ).readAsStringSync(),
        'builder-ohos',
      );
      expect(
        File('${generated.path}/rust_builder/pubspec.yaml').readAsStringSync(),
        'plugin:\n  platforms:\n    ohos:\n',
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
