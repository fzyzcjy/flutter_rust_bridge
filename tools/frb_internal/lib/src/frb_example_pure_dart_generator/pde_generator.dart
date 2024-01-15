// ignore_for_file: avoid_print

import 'dart:io';

import 'package:path/path.dart';

Future<void> generatePureDartPde({required Uri dirPureDart}) async {
  final dirPureDartPde = Directory.current.uri.resolve('../pure_dart_pde/');

  copyRecursive(
    Directory(dirPureDart.toFilePath()),
    Directory(dirPureDartPde.toFilePath()),
    filter: (entity) {
      final relativePath =
          relative(entity.path, from: dirPureDart.toFilePath());
      return !const [
        '.dart_tool',
        '.idea',
        'benchmark',
        'build',
        'coverage',
        'rust/target',
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
