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
}
