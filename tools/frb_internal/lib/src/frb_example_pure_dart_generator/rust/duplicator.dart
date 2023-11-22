import 'dart:io';

import 'package:path/path.dart' as path;

void generateRustDuplicates({required Uri rustRoot}) {
  for (final file in Directory(rustRoot.resolve('src/api/').toFilePath()).listSync()) {
    final fileName = path.basename(file.path);
    if (fileName == 'mod.rs') continue;

    TODO;
  }
}
