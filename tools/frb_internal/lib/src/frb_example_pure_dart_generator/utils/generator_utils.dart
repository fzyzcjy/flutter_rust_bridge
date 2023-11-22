import 'dart:io';

import 'package:meta/meta.dart';
import 'package:path/path.dart' as path;

abstract class BaseGenerator {
  final Uri packageRootDir;
  final Uri interestDir;

  BaseGenerator({required this.packageRootDir, required String interestDir})
      : interestDir = packageRootDir.resolve(interestDir);

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
  String generateDuplicateCode(String inputText, DuplicatorMode mode);

  @protected
  String generateDuplicateFileStem(String inputStem, DuplicatorMode mode);

  @protected
  Future<void> executeFormat();

  void _writeCodeFiles(Map<String, String> textOfPathMap) {
    for (final entry in textOfPathMap.entries) {
      File(interestDir.resolve(entry.key).toFilePath()).writeAsStringSync(entry.value);
    }
  }
}

enum DuplicatorMode {
  sync;
  // TODO rust-async, ...

  String get postfix => '_twin_$name';
}

class _Duplicator {
  final BaseGenerator generator;

  _Duplicator(this.generator);

  void _generate() {
    for (final file in Directory(generator.interestDir.toFilePath()).listSync()) {
      final fileName = path.basename(file.path);
      final fileStem = path.basenameWithoutExtension(file.path);
      if (file is! File || path.extension(file.path) != '.${generator.extension}') continue;
      if (generator.duplicatorBlacklistNames.contains(fileName)) continue;

      for (final mode in DuplicatorMode.values) {
        final outputText = _computePrelude(fileName) + generator.generateDuplicateCode(file.readAsStringSync(), mode);
        final targetPath = file.uri
            .resolve('pseudo_manual')
            .resolve('${generator.generateDuplicateFileStem(fileStem, mode)}.${generator.extension}')
            .toFilePath();
        File(targetPath).writeAsStringSync(outputText);
      }
    }
  }

  static String _computePrelude(String originalFileName) =>
      '// NOTE: This file is auto-generated from `$originalFileName` by frb_internal\n'
      '// Please do not modify manually, but modify the `$originalFileName` and re-run frb_internal generator\n\n';
}
