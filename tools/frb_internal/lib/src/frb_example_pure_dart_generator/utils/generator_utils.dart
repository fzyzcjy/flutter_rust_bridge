import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/preludes.dart';
import 'package:glob/glob.dart';
import 'package:glob/list_local_fs.dart';
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
      File(interestDir.resolve(entry.key).toFilePath())
          .writeAsStringSync(entry.value);
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
    for (final file in Glob(
            '${generator.interestDir.toFilePath()}/**.${generator.extension}')
        .listSync()) {
      final fileName = path.basename(file.path);
      final fileStem = path.basenameWithoutExtension(file.path);
      if (file is! File ||
          path.extension(file.path) != '.${generator.extension}') continue;
      if (generator.duplicatorBlacklistNames.contains(fileName)) continue;
      if (DuplicatorMode.values
          .any((mode) => fileStem.contains(mode.postfix))) {
        continue;
      }

      final fileContent = (file as File).readAsStringSync();
      final annotation = _parseAnnotation(fileContent);

      for (final mode in DuplicatorMode.values) {
        if (annotation.forbiddenDuplicatorModes.contains(mode)) continue;

        final outputText = computeDuplicatorPrelude(' from `$fileName`') +
            (annotation.extraCode ?? '') +
            generator.generateDuplicateCode(fileContent, mode);
        final targetPath = generator.interestDir
            .resolve('pseudo_manual/')
            .resolve(
                '${generator.generateDuplicateFileStem(fileStem, mode)}.${generator.extension}')
            .toFilePath();
        File(targetPath).writeAsStringSync(outputText);
      }
    }
  }
}

_Annotation _parseAnnotation(String fileContent) {
  const kPrefix = '// FRB_INTERNAL_GENERATOR:';
  if (!fileContent.startsWith(kPrefix)) return const _Annotation();

  final data = jsonDecode(
          fileContent.substring(kPrefix.length, fileContent.indexOf('\n')))
      as Map<String, Object?>;
  return _Annotation(
    forbiddenDuplicatorModes:
        ((data['forbiddenDuplicatorModes'] as List<dynamic>?) ?? [])
            .map((x) => DuplicatorMode.values.byName(x))
            .toList(),
    extraCode: data['extraCode'] as String?,
  );
}

class _Annotation {
  final List<DuplicatorMode> forbiddenDuplicatorModes;
  final String? extraCode;

  const _Annotation({
    this.forbiddenDuplicatorModes = const [],
    this.extraCode,
  });
}
