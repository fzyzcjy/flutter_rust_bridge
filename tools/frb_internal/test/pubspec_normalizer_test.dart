import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/pubspec_normalizer.dart';
import 'package:test/test.dart';

void main() {
  test('pubspec yaml normalization keeps checked-in Dart minimum', () {
    expect(
      normalizePubspecYamlText('''
environment:
  sdk: ^3.12.0
'''),
      '''
environment:
  sdk: ^3.11.0
''',
    );
  });

  test('pubspec lock normalization keeps Flutter 3.44 app floor stable', () {
    expect(
      normalizePubspecLockText('''
sdks:
  dart: ">=3.12.0 <4.0.0"
  flutter: ">=3.18.0-18.0.pre.54"
'''),
      '''
sdks:
  dart: ">=3.11.0 <4.0.0"
  flutter: ">=3.18.0-18.0.pre.54"
''',
    );
  });

  test('pubspec lock normalization keeps older example floor stable', () {
    expect(
      normalizePubspecLockText('''
sdks:
  dart: ">=3.10.0-0 <4.0.0"
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
  sdk: ^3.12.0
''');
    File('${package.path}/pubspec.lock').writeAsStringSync('''
sdks:
  dart: ">=3.12.0 <4.0.0"
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
}
