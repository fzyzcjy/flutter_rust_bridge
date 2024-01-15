// ignore_for_file: avoid_print

import 'dart:io';

import 'package:path/path.dart';

Future<void> generatePureDartPde({required Uri dirPureDart}) async {
  final dirPureDartPde = dirPureDart.resolve('../pure_dart_pde/');

  copyRecursive(
    Directory(dirPureDart.toFilePath()),
    Directory(dirPureDartPde.toFilePath()),
    filter: (entity) {
      final relativePath =
          relative(entity.path, from: dirPureDart.toFilePath());
      return !const [
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
    },
  );
}

// copied and modified fromhttps://stackoverflow.com/questions/27204728
void copyRecursive(
  Directory src,
  Directory dst, {
  required bool Function(FileSystemEntity) filter,
}) {
  if (!dst.existsSync()) {
    dst.createSync(recursive: true);
  }

  for (final entity in src.listSync(recursive: false)) {
    if (!filter(entity)) continue;

    final newPath = join(dst.path, basename(entity.path));
    if (entity is File) {
      print('Copy $entity -> $newPath');
      entity.copySync(newPath);
    } else if (entity is Directory) {
      copyRecursive(entity, Directory(newPath), filter: filter);
    }
  }
}
