import 'dart:io';

void writeFiles(Uri baseDir, Map<String, String> textOfPathMap) {
  for (final entry in textOfPathMap.entries) {
    File(baseDir.resolve(entry.key).toFilePath()).writeAsStringSync(entry.value);
  }
}
