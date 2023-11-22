import 'dart:io';

import 'package:meta/meta.dart';

abstract class BaseGenerator {
  final Uri baseDir;

  BaseGenerator({required this.baseDir});

  Future<void> generate() async {
    _writeCodeFiles(baseDir, generateDartDirectSources());
    _generateDuplicates();
    await executeDartFormat(workingDirectory: dartRoot.toFilePath());
  }

  @protected
  Future<void> executeFormat();

  void _writeCodeFiles(Map<String, String> textOfPathMap) {
    for (final entry in textOfPathMap.entries) {
      File(baseDir.resolve(entry.key).toFilePath()).writeAsStringSync(entry.value);
    }
  }
}
