// ignore_for_file: avoid_print

import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart';
import 'package:path/path.dart';

Future<void> generatePureDartPde(
    {required Uri dirPureDart, required Uri dirPureDartPde}) async {
  copyRecursive(Directory(dirPureDart.toFilePath()),
      Directory(dirPureDartPde.toFilePath()), filter: (entity) {
    final relativePath = relative(entity.path, from: dirPureDart.toFilePath());

    if (const [
          '.DS_Store',
        ].contains(basename(relativePath)) ||
        const [
          // gitignore them
          '.dart_tool',
          '.idea',
          'benchmark',
          'build',
          'coverage',
          'rust/target',
          'web',

          // will generate separately
          'frb_generated.h',
          'lib/src/rust',
          'test/api/pseudo_manual',
          'rust/src/api/pseudo_manual',
          'rust/src/frb_generated.rs',
          'rust/src/frb_generated.io.rs',
          'rust/src/frb_generated.web.rs',
        ].contains(relativePath)) {
      return false;
    }

    if (entity is File) {
      final annotation = Annotation.parse(entity.readAsStringSync());
      if (annotation.skipPde) return false;
    }

    return true;
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
        return prelude +
            text
                .replaceAll(
                  'package:frb_example_pure_dart',
                  'package:frb_example_pure_dart_pde',
                )
                .replaceAll('RustOpaqueNom<', 'RustOpaqueMoi<')
                // hack (not a problem, since this script merely generates test code for bridge,
                // but not generate anything related to real users)
                .replaceAll('mirror_twin_sync_sse', 'mirror_twin_sync');
    }
  });

  // To refresh Cargo.lock's ordering
  await exec('cargo fetch', relativePwd: 'frb_example/pure_dart_pde/rust');
}

// copied and modified from https://stackoverflow.com/questions/27204728
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
