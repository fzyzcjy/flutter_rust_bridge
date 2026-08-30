import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/integrate_ohos_scaffold.dart';
import 'package:test/test.dart';

void main() {
  test('preserved OHOS scaffold paths are explicit', () {
    expect(preservedOhosScaffoldPaths('frb_example/flutter_via_create'), [
      'ohos',
      'rust_builder/ohos',
      'rust_builder/pubspec.yaml',
    ]);
    expect(
      preservedOhosScaffoldPaths(
        'frb_example/flutter_via_create_native_assets',
      ),
      ['ohos'],
    );
    expect(preservedOhosScaffoldPaths('frb_example/flutter_via_integrate'), [
      'ohos',
    ]);
    expect(preservedOhosScaffoldPaths('frb_example/flutter_package'), isEmpty);
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

  test('preserveCheckedInOhosScaffold restores integrate OHOS files', () async {
    final tempDir = Directory.systemTemp.createTempSync(
      'frb-preserve-integrate-ohos-',
    );
    addTearDown(() => tempDir.deleteSync(recursive: true));

    final original = Directory('${tempDir.path}/original');
    final generated = Directory('${tempDir.path}/generated');
    Directory('${original.path}/ohos').createSync(recursive: true);
    File(
      '${original.path}/ohos/ohos_device_smoke_main.dart',
    ).writeAsStringSync('void main() {}');
    Directory(generated.path).createSync(recursive: true);

    await preserveCheckedInOhosScaffold(
      package: 'frb_example/flutter_via_integrate',
      originalPackageDir: original.path,
      generatedPackageDir: generated.path,
    );

    expect(
      File(
        '${generated.path}/ohos/ohos_device_smoke_main.dart',
      ).readAsStringSync(),
      'void main() {}',
    );
  });

  test(
    'checked-in OHOS restore rejects source symlink before mutation',
    () async {
      if (Platform.isWindows) return;

      final tempDir = Directory.systemTemp.createTempSync(
        'frb-preserve-ohos-symlink-',
      );
      addTearDown(() => tempDir.deleteSync(recursive: true));

      final original = Directory('${tempDir.path}/original');
      final generated = Directory('${tempDir.path}/generated');
      final linkedOhos = Directory('${tempDir.path}/linked-ohos');
      original.createSync(recursive: true);
      linkedOhos.createSync(recursive: true);
      Link('${original.path}/ohos').createSync(linkedOhos.path);
      File('${generated.path}/ohos/original.txt')
        ..createSync(recursive: true)
        ..writeAsStringSync('original-ohos');

      await expectLater(
        preserveCheckedInOhosScaffold(
          package: 'frb_example/flutter_via_integrate',
          originalPackageDir: original.path,
          generatedPackageDir: generated.path,
        ),
        throwsUnimplementedError,
      );

      expect(
        File('${generated.path}/ohos/original.txt').readAsStringSync(),
        'original-ohos',
      );
    },
  );

  test('OHOS composition keeps generated OHOS files', () async {
    final tempDir = Directory.systemTemp.createTempSync('frb-retain-ohos-');
    addTearDown(() => tempDir.deleteSync(recursive: true));

    final original = Directory('${tempDir.path}/original');
    final generated = Directory('${tempDir.path}/generated');
    File('${original.path}/generic.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('original');
    File('${original.path}/ohos/removed.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('removed');
    File('${original.path}/rust_builder/ohos/marker.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('original-ohos');
    File('${original.path}/rust_builder/pubspec.yaml')
      ..createSync(recursive: true)
      ..writeAsStringSync('original-pubspec');
    File('${generated.path}/generic.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('generated');
    File('${generated.path}/ohos/marker.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('generated-ohos');
    File('${generated.path}/rust_builder/ohos/marker.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('generated-builder-ohos');
    File('${generated.path}/rust_builder/pubspec.yaml')
      ..createSync(recursive: true)
      ..writeAsStringSync('generated-pubspec');

    await restoreOriginalPackageWithGeneratedOhosScaffold(
      package: 'frb_example/flutter_via_integrate',
      originalPackageDir: original.path,
      generatedPackageDir: generated.path,
      temporaryDirectory: tempDir.path,
    );

    expect(
      File('${generated.path}/generic.txt').readAsStringSync(),
      'original',
    );
    expect(
      File('${generated.path}/ohos/marker.txt').readAsStringSync(),
      'generated-ohos',
    );
    expect(File('${generated.path}/ohos/removed.txt').existsSync(), isFalse);
    expect(
      File('${generated.path}/rust_builder/ohos/marker.txt').readAsStringSync(),
      'generated-builder-ohos',
    );
    expect(
      File('${generated.path}/rust_builder/pubspec.yaml').readAsStringSync(),
      'generated-pubspec',
    );
    expect(
      Directory('${tempDir.path}/generated_package').existsSync(),
      isFalse,
    );
    expect(Directory('${tempDir.path}/staged_ohos').existsSync(), isFalse);
    expect(
      Directory('${tempDir.path}/original_ohos_backup').existsSync(),
      isFalse,
    );
  });

  test('OHOS composition drops generated node modules', () async {
    final tempDir = Directory.systemTemp.createTempSync(
      'frb-retain-ohos-node-modules-',
    );
    addTearDown(() => tempDir.deleteSync(recursive: true));

    final original = Directory('${tempDir.path}/original');
    final generated = Directory('${tempDir.path}/generated');
    File('${original.path}/generic.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('original');
    File('${generated.path}/ohos/scaffold.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('generated-ohos');
    File('${generated.path}/ohos/node_modules/flutter-hvigor-plugin/index.js')
      ..createSync(recursive: true)
      ..writeAsStringSync('generated-dependency');

    await restoreOriginalPackageWithGeneratedOhosScaffold(
      package: 'frb_example/flutter_via_create_native_assets',
      originalPackageDir: original.path,
      generatedPackageDir: generated.path,
      temporaryDirectory: tempDir.path,
    );

    expect(
      File('${generated.path}/ohos/scaffold.txt').readAsStringSync(),
      'generated-ohos',
    );
    expect(
      Directory('${generated.path}/ohos/node_modules').existsSync(),
      false,
    );
  });

  test('OHOS composition rejects missing source before package swap', () async {
    final tempDir = Directory.systemTemp.createTempSync(
      'frb-retain-ohos-missing-source-',
    );
    addTearDown(() => tempDir.deleteSync(recursive: true));

    final original = Directory('${tempDir.path}/original');
    final generated = Directory('${tempDir.path}/generated');
    File('${original.path}/generic.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('original');
    File('${original.path}/ohos/original.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('original-ohos');
    File('${original.path}/rust_builder/ohos/original.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('original-builder-ohos');
    File('${original.path}/rust_builder/pubspec.yaml')
      ..createSync(recursive: true)
      ..writeAsStringSync('original-pubspec');
    File('${generated.path}/ohos/generated.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('generated-ohos');

    await expectLater(
      restoreOriginalPackageWithGeneratedOhosScaffold(
        package: 'frb_example/flutter_via_integrate',
        originalPackageDir: original.path,
        generatedPackageDir: generated.path,
        temporaryDirectory: tempDir.path,
      ),
      throwsStateError,
    );

    expect(File('${original.path}/generic.txt').readAsStringSync(), 'original');
    expect(
      File('${generated.path}/ohos/generated.txt').readAsStringSync(),
      'generated-ohos',
    );
    expect(Directory('${tempDir.path}/staged_ohos').existsSync(), isFalse);
  });

  test(
    'restoreOriginalPackageWithGeneratedOhosScaffold rejects nested source symlink',
    () async {
      if (Platform.isWindows) return;

      final tempDir = Directory.systemTemp.createTempSync(
        'frb-retain-ohos-nested-symlink-',
      );
      addTearDown(() => tempDir.deleteSync(recursive: true));

      final original = Directory('${tempDir.path}/original');
      final generated = Directory('${tempDir.path}/generated');
      final linkedDirectory = Directory('${tempDir.path}/linked-directory');
      File('${original.path}/generic.txt')
        ..createSync(recursive: true)
        ..writeAsStringSync('original');
      File('${original.path}/ohos/original.txt')
        ..createSync(recursive: true)
        ..writeAsStringSync('original-ohos');
      Directory('${generated.path}/ohos').createSync(recursive: true);
      linkedDirectory.createSync(recursive: true);
      Link(
        '${generated.path}/ohos/nested-link',
      ).createSync(linkedDirectory.path);

      await expectLater(
        restoreOriginalPackageWithGeneratedOhosScaffold(
          package: 'frb_example/flutter_via_create_native_assets',
          originalPackageDir: original.path,
          generatedPackageDir: generated.path,
          temporaryDirectory: tempDir.path,
        ),
        throwsUnimplementedError,
      );

      expect(
        File('${original.path}/generic.txt').readAsStringSync(),
        'original',
      );
      expect(Link('${generated.path}/ohos/nested-link').existsSync(), isTrue);
      expect(Directory('${tempDir.path}/staged_ohos').existsSync(), isFalse);
    },
  );

  test(
    'restoreOriginalPackageWithGeneratedOhosScaffold rejects invalid destination before overlay',
    () async {
      final tempDir = Directory.systemTemp.createTempSync(
        'frb-retain-ohos-rollback-',
      );
      addTearDown(() => tempDir.deleteSync(recursive: true));

      final original = Directory('${tempDir.path}/original');
      final generated = Directory('${tempDir.path}/generated');
      File('${original.path}/generic.txt')
        ..createSync(recursive: true)
        ..writeAsStringSync('original');
      File('${original.path}/ohos/original.txt')
        ..createSync(recursive: true)
        ..writeAsStringSync('original-ohos');
      File('${original.path}/rust_builder')
        ..createSync(recursive: true)
        ..writeAsStringSync('blocks-overlay');
      File('${generated.path}/ohos/generated.txt')
        ..createSync(recursive: true)
        ..writeAsStringSync('generated-ohos');
      File('${generated.path}/rust_builder/ohos/generated.txt')
        ..createSync(recursive: true)
        ..writeAsStringSync('generated-builder-ohos');
      File('${generated.path}/rust_builder/pubspec.yaml')
        ..createSync(recursive: true)
        ..writeAsStringSync('generated-pubspec');

      await expectLater(
        restoreOriginalPackageWithGeneratedOhosScaffold(
          package: 'frb_example/flutter_via_integrate',
          originalPackageDir: original.path,
          generatedPackageDir: generated.path,
          temporaryDirectory: tempDir.path,
        ),
        throwsStateError,
      );

      expect(
        File('${original.path}/generic.txt').readAsStringSync(),
        'original',
      );
      expect(
        File('${generated.path}/ohos/generated.txt').readAsStringSync(),
        'generated-ohos',
      );
      expect(
        File('${original.path}/rust_builder').readAsStringSync(),
        'blocks-overlay',
      );
      expect(Directory('${tempDir.path}/staged_ohos').existsSync(), isFalse);
    },
  );

  test('failed OHOS generation restores the original package', () async {
    final tempDir = Directory.systemTemp.createTempSync(
      'frb-restore-failed-ohos-generation-',
    );
    addTearDown(() => tempDir.deleteSync(recursive: true));

    final original = Directory('${tempDir.path}/original');
    final generated = Directory('${tempDir.path}/generated');
    File('${original.path}/marker.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('original');
    File('${generated.path}/marker.txt')
      ..createSync(recursive: true)
      ..writeAsStringSync('failed-generated');

    await restoreOriginalPackageAfterFailedOhosGeneration(
      originalPackageDir: original.path,
      generatedPackageDir: generated.path,
      temporaryDirectory: tempDir.path,
    );

    expect(File('${generated.path}/marker.txt').readAsStringSync(), 'original');
    expect(original.existsSync(), isFalse);
    expect(
      Directory('${tempDir.path}/failed_generated_package').existsSync(),
      isFalse,
    );
  });

  test('OHOS composition rejects unconfigured packages', () async {
    await expectLater(
      restoreOriginalPackageWithGeneratedOhosScaffold(
        package: 'frb_example/flutter_package',
        originalPackageDir: '/unused/original',
        generatedPackageDir: '/unused/generated',
        temporaryDirectory: '/unused/temporary',
      ),
      throwsStateError,
    );
  });
}
