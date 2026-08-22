import 'dart:io';

import 'package:test/test.dart';

void main() {
  test('native assets hook constructs the builder inside the callback', () {
    final template = _readRepoFile(_templatePath);
    final callbackStart = template.indexOf(
      'await build(args, (input, output) async {',
    );
    final callbackEnd = template.indexOf('\n  });', callbackStart);
    final builderConstruction = template.indexOf(
      'const builder = FlutterRustBridgeNativeAssetsBuilder(',
    );

    expect(builderConstruction, greaterThan(callbackStart));
    expect(builderConstruction, lessThan(callbackEnd));
  });

  test('native assets hook template expands to checked-in examples', () {
    final expected = _readRepoFile(
      _templatePath,
    ).replaceAll('REPLACE_ME_RUST_CRATE_DIR', 'rust');

    for (final examplePath in [
      'frb_example/flutter_via_create_native_assets/hook/build.dart',
      'frb_example/flutter_via_integrate_native_assets/hook/build.dart',
      'frb_example/flutter_package_native_assets/hook/build.dart',
    ]) {
      expect(_readRepoFile(examplePath), expected, reason: examplePath);
    }
  });
}

const _templatePath =
    'frb_codegen/assets/integration_template/native_assets/shared/hook/build.dart';

String _readRepoFile(String relativePath) =>
    File('../../$relativePath').readAsStringSync();
