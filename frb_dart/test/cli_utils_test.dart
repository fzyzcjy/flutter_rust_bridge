@TestOn('vm')
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:test/test.dart';

void main() {
  test('err returns the provided message', () {
    expect(err('problem'), 'problem');
  });

  test('findDartPackageDirectory finds the closest pubspec ancestor', () async {
    final temporaryDirectory = await Directory.systemTemp.createTemp(
      'frb_cli_utils_test_',
    );
    final packageDirectory = Directory('${temporaryDirectory.path}/package');
    final nestedDirectory = Directory('${packageDirectory.path}/lib/src');

    try {
      await nestedDirectory.create(recursive: true);
      await File(
        '${packageDirectory.path}/pubspec.yaml',
      ).writeAsString('name: test_package');

      expect(
        await findDartPackageDirectory(nestedDirectory.path),
        packageDirectory.absolute.path,
      );
    } finally {
      await temporaryDirectory.delete(recursive: true);
    }
  });

  test('findDartPackageDirectory reports a missing package root', () async {
    final temporaryDirectory = await Directory.systemTemp.createTemp(
      'frb_cli_utils_missing_test_',
    );

    try {
      await expectLater(
        () => findDartPackageDirectory(temporaryDirectory.path),
        throwsA(isA<ArgumentError>()),
      );
    } finally {
      await temporaryDirectory.delete(recursive: true);
    }
  });
}
