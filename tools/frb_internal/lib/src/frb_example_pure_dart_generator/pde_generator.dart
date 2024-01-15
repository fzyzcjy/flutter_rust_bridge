import 'dart:io';

import 'package:path/path.dart';

Future<void> generatePureDartPde({required Uri dirPureDart}) async {
  final dirPureDartPde = Directory.current.uri.resolve('../pure_dart_pde/');

  TODO;
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
    if (!filter(entity)) return;

    final newPath = join(dst.path, basename(entity.path));
    if (entity is File) {
      entity.copySync(newPath);
    } else if (entity is Directory) {
      copyRecursive(entity, Directory(newPath), filter: filter);
    }
  }
}
