@TestOn('vm')
import 'package:flutter_rust_bridge/src/cli/build_web/entrypoint.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  test('build-web parser forwards Dart wasm entrypoint', () {
    final config = parseConfig([
      '--dart-root',
      'example',
      '--dart-compile-js-entrypoint',
      'web/main.dart',
      '--dart-compile-wasm-entrypoint',
      'web/main.dart',
    ]);

    final args = parseBuildWebConfigToArgs(config);

    expect(args.output, path.join('example', 'web'));
    expect(args.dartCompileJsEntrypoint, 'web/main.dart');
    expect(args.dartCompileWasmEntrypoint, 'web/main.dart');
  });
}
