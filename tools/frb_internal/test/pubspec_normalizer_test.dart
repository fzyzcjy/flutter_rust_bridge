import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/pubspec_normalizer.dart';
import 'package:test/test.dart';

void main() {
  for (final version in ['3.12.0', '3.13.0', '3.14.0']) {
    test(
      'pubspec yaml normalization keeps Dart $version template floor stable',
      () {
        expect(
          normalizePubspecYamlText('''
environment:
  sdk: ^$version
'''),
          '''
environment:
  sdk: ^3.11.0
''',
        );
      },
    );

    test('pubspec lock normalization keeps Dart $version app floor stable', () {
      expect(
        normalizePubspecLockText('''
sdks:
  dart: ">=$version <4.0.0"
  flutter: ">=3.18.0-18.0.pre.54"
'''),
        '''
sdks:
  dart: ">=3.11.0 <4.0.0"
  flutter: ">=3.18.0-18.0.pre.54"
''',
      );
    });
  }

  for (final package in ['material_ui', 'cupertino_ui']) {
    test('standalone $package keeps its required SDK floor', () {
      expect(
        normalizePubspecYamlText('''
environment:
  sdk: ^3.14.0
dependencies:
  $package: ^1.1.0
'''),
        '''
environment:
  sdk: ^3.13.0
dependencies:
  $package: ^1.1.0
''',
      );
      expect(
        normalizePubspecLockText('''
packages:
  $package:
    dependency: "direct main"
sdks:
  dart: ">=3.14.0 <4.0.0"
'''),
        '''
packages:
  $package:
    dependency: "direct main"
sdks:
  dart: ">=3.13.0 <4.0.0"
''',
      );
    });
  }

  test('pubspec lock normalization keeps older example floor stable', () {
    expect(
      normalizePubspecLockText('''
sdks:
  dart: ">=3.11.0-0 <4.0.0"
  flutter: ">=3.18.0-18.0.pre.54"
'''),
      '''
sdks:
  dart: ">=3.9.2 <4.0.0"
  flutter: ">=3.18.0-18.0.pre.54"
''',
    );
  });

  test('pubspec lock normalization canonicalizes pub host', () {
    expect(
      normalizePubspecLockText('''
description:
  url: "https://pub.flutter-io.cn"
'''),
      '''
description:
  url: "https://pub.dev"
''',
    );
  });

  test('pubspec normalization handles nested package paths', () {
    final temp = Directory.systemTemp.createTempSync(
      'frb_pubspec_normalizer_test_',
    );
    addTearDown(() => temp.deleteSync(recursive: true));

    final package = Directory('${temp.path}/outer/example')
      ..createSync(recursive: true);
    File('${package.path}/pubspec.yaml').writeAsStringSync('''
environment:
  sdk: ^3.13.0
''');
    File('${package.path}/pubspec.lock').writeAsStringSync('''
sdks:
  dart: ">=3.13.0 <4.0.0"
''');

    normalizePubspecs(
      repoRootPath: temp.path,
      packages: ['outer/example', 'missing_package'],
    );

    expect(File('${package.path}/pubspec.yaml').readAsStringSync(), '''
environment:
  sdk: ^3.11.0
''');
    expect(File('${package.path}/pubspec.lock').readAsStringSync(), '''
sdks:
  dart: ">=3.11.0 <4.0.0"
''');
  });

  test('pubspec normalization keeps internal tooling on the primary SDK', () {
    final temp = Directory.systemTemp.createTempSync(
      'frb_pubspec_normalizer_test_',
    );
    addTearDown(() => temp.deleteSync(recursive: true));

    final package = Directory('${temp.path}/tools/frb_internal')
      ..createSync(recursive: true);
    File('${package.path}/pubspec.yaml').writeAsStringSync('''
environment:
  sdk: ^3.13.0
''');
    File('${package.path}/pubspec.lock').writeAsStringSync('''
description:
  url: "https://pub.flutter-io.cn"
sdks:
  dart: ">=3.13.0 <4.0.0"
''');

    normalizePubspecs(
      repoRootPath: temp.path,
      packages: ['tools/frb_internal'],
    );

    expect(File('${package.path}/pubspec.yaml').readAsStringSync(), '''
environment:
  sdk: ^3.13.0
''');
    expect(File('${package.path}/pubspec.lock').readAsStringSync(), '''
description:
  url: "https://pub.dev"
sdks:
  dart: ">=3.13.0 <4.0.0"
''');
  });
}
