@TestOn('vm')
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:test/test.dart';

void main() {
  test('err returns the original message', () {
    expect(err('hello'), 'hello');
  });

  test('findDartPackageDirectory finds nearest pubspec directory', () async {
    final tempDir = await Directory.systemTemp.createTemp();
    addTearDown(() async => await tempDir.delete(recursive: true));

    final packageDir = Directory('${tempDir.path}/outer/package')
      ..createSync(recursive: true);
    File('${packageDir.path}/pubspec.yaml').writeAsStringSync('name: demo\n');
    final nestedDir = Directory('${packageDir.path}/lib/src')
      ..createSync(recursive: true);

    expect(
      await findDartPackageDirectory(nestedDir.path),
      packageDir.absolute.path,
    );
  });

  test('findDartPackageDirectory throws when no pubspec is found', () async {
    final tempDir = await Directory.systemTemp.createTemp();
    addTearDown(() async => await tempDir.delete(recursive: true));

    await expectLater(
      () async => await findDartPackageDirectory(tempDir.path),
      throwsA(isA<ArgumentError>()),
    );
  });
}
