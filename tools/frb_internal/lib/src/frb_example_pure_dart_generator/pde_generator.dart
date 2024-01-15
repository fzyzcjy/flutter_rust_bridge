// ignore_for_file: avoid_print

import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart';
import 'package:path/path.dart';

Future<void> generatePureDartPde(
    {required Uri dirPureDart, required Uri dirPureDartPde}) async {
  copyRecursive(Directory(dirPureDart.toFilePath()),
      Directory(dirPureDartPde.toFilePath()), filter: (entity) {
    final relativePath = relative(entity.path, from: dirPureDart.toFilePath());
    return !const [
          '.DS_Store',
        ].contains(basename(relativePath)) &&
        !const [
          // gitignore them
          '.dart_tool',
          '.idea',
          'benchmark',
          'build',
          'coverage',
          'rust/target',

          // will generate separately
          'lib/src/rust',
          'test/api/pseudo_manual',
          'rust/src/api/pseudo_manual',
          'rust/src/frb_generated.rs',
          'rust/src/frb_generated.io.rs',
          'rust/src/frb_generated.web.rs',
        ].contains(relativePath);
  }, map: (file, text) {
    final relativePath = relative(file.path, from: dirPureDart.toFilePath());

    switch (relativePath) {
      case 'pubspec.yaml':
        return simpleReplaceString(text, 'name: frb_example_pure_dart',
            'name: frb_example_pure_dart_pde');

      case 'rust/Cargo.toml':
        return simpleReplaceString(text, 'name = "frb_example_pure_dart"',
            'name = "frb_example_pure_dart_pde"');
      case 'rust/Cargo.lock':
        return simpleReplaceString(
            text, '"frb_example_pure_dart"', '"frb_example_pure_dart_pde"');

      case 'flutter_rust_bridge.yaml':
        return simpleReplaceString(text, '\nfull_dep: true', '');

      default:
        final prelude = switch (extension(file.path)) {
          '.rs' ||
          '.dart' =>
            '// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT\n\n',
          _ => '',
        };
        return prelude + text;
    }
  });
}

// copied and modified fromhttps://stackoverflow.com/questions/27204728
void copyRecursive(
  Directory src,
  Directory dst, {
  required bool Function(FileSystemEntity) filter,
  required String Function(File, String) map,
}) {
  if (!dst.existsSync()) {
    dst.createSync(recursive: true);
  }

  for (final entity in src.listSync(recursive: false)) {
    if (!filter(entity)) continue;

    final newPath = join(dst.path, basename(entity.path));
    if (entity is File) {
      print('Copy $entity -> $newPath');
      File(newPath).writeAsStringSync(map(entity, entity.readAsStringSync()));
    } else if (entity is Directory) {
      copyRecursive(entity, Directory(newPath), filter: filter, map: map);
    }
  }
}
