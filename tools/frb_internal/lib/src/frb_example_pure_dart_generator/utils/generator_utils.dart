import 'dart:io';

import 'package:meta/meta.dart';
import 'package:path/path.dart' as path;

abstract class BaseGenerator {
  final Uri baseDir;

  BaseGenerator({required this.baseDir});

  Future<void> generate() async {
    _writeCodeFiles(generateDirectSources());
    _Duplicator(this)._generate();
    await executeFormat();
  }

  @protected
  Set<String> get duplicatorBlacklistNames;

  @protected
  String get extension;

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

enum DuplicatorMode {
  sync,
  // TODO rust-async, ...
}

class _Duplicator {
  final BaseGenerator generator;

  _Duplicator(this.generator);

  void _generate() {
    for (final file in Directory(generator.baseDir.toFilePath()).listSync()) {
      final fileName = path.basename(file.path);
      final fileStem = path.basenameWithoutExtension(file.path);
      if (file is! File || path.extension(file.path) != '.${generator.extension}') continue;
      if (generator.duplicatorBlacklistNames.contains(fileName)) continue;
      if (DuplicatorMode.values.any((mode) => fileStem.endsWith(_computePostfix(mode)))) continue;

      for (final mode in DuplicatorMode.values) {
        final outputText = _generateOne(file.readAsStringSync(), mode);
        final targetPath = file.uri.resolve('../$fileStem${_computePostfix(mode)}.${generator.extension}').toFilePath();
        File(targetPath).writeAsStringSync(outputText);
      }
    }
  }

  String _generateOne(String inputText, DuplicatorMode mode) {
    return TODO;
  }

  String _computePostfix(DuplicatorMode mode) => '_twin_${mode.name}';
}
