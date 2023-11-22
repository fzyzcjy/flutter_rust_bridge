import 'dart:io';

import 'package:meta/meta.dart';
import 'package:path/path.dart' as path;

abstract class BaseGenerator {
  final Uri baseDir;

  BaseGenerator({required this.baseDir});

  Future<void> generate() async {
    _writeCodeFiles(generateDirectSources());
    _generateDuplicates();
    await executeFormat();
  }

  void _generateDuplicates() {
    for (final file in Directory(baseDir.toFilePath()).listSync()) {
      final fileName = path.basename(file.path);
      if (duplicatorBlacklistNames.contains(fileName)) continue;

      TODO;
    }
  }

  @protected
  Set<String> get duplicatorBlacklistNames;

  @protected
  Map<String, String> generateDirectSources();

  @protected
  Future<void> executeFormat();

  void _writeCodeFiles(Map<String, String> textOfPathMap) {
    for (final entry in textOfPathMap.entries) {
      File(baseDir.resolve(entry.key).toFilePath()).writeAsStringSync(entry.value);
    }
  }
}
