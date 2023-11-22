import 'dart:io';

void writeCodeFiles(Uri baseDir, Map<String, String> textOfPathMap) {
  for (final entry in textOfPathMap.entries) {
    File(baseDir.resolve(entry.key).toFilePath()).writeAsStringSync(entry.value);
  }
}
