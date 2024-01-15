import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/preludes.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:glob/glob.dart';
import 'package:glob/list_local_fs.dart';
import 'package:path/path.dart' as path;
import 'package:recase/recase.dart';

part 'generator_utils.freezed.dart';

abstract class BaseGenerator {
  final Uri packageRootDir;
  final Uri interestDir;
  final Package package;

  BaseGenerator({
    required this.packageRootDir,
    required String interestDir,
    required this.package,
  }) : interestDir = packageRootDir.resolve(interestDir);

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

enum DuplicatorComponentMode {
  sync,
  rustAsync,
  sse,
  moi,
}

@freezed
class DuplicatorMode with _$DuplicatorMode {
  const factory DuplicatorMode(List<DuplicatorComponentMode> components) =
      _DuplicatorMode;

  const DuplicatorMode._();

  static DuplicatorMode parse(String raw) => DuplicatorMode(
      raw.split(' ').map(DuplicatorComponentMode.values.byName).toList());

  static const defaultValues = [
    DuplicatorMode([DuplicatorComponentMode.sync]),
    DuplicatorMode([DuplicatorComponentMode.rustAsync]),
    DuplicatorMode([DuplicatorComponentMode.sse]),
    DuplicatorMode([DuplicatorComponentMode.sync, DuplicatorComponentMode.sse]),
    DuplicatorMode(
        [DuplicatorComponentMode.rustAsync, DuplicatorComponentMode.sse]),
  ];

  static final allValues = [
    ...defaultValues,
    ...[const DuplicatorMode([]), ...defaultValues].map(
        (e) => DuplicatorMode([...e.components, DuplicatorComponentMode.moi])),
  ];

  String get postfix =>
      '_twin_${components.map((c) => ReCase(c.name).snakeCase).join('_')}';
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
      if (DuplicatorMode.allValues
          .any((mode) => fileStem.contains(mode.postfix))) {
        continue;
      }

      final fileContent = (file as File).readAsStringSync();
      final annotation = _parseAnnotation(fileContent);

      final chosenModes = annotation.enableAll
          ? DuplicatorMode.allValues
          : DuplicatorMode.defaultValues;

      for (final mode in chosenModes) {
        if (annotation.forbiddenDuplicatorModes.contains(mode)) continue;

        var outputText = computeDuplicatorPrelude(' from `$fileName`') +
            (annotation.addCode ?? '') +
            generator.generateDuplicateCode(fileContent, mode);
        for (final x in annotation.removeCode) {
          outputText = outputText.replaceAll(x, '');
        }

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
            .map((x) => DuplicatorMode.parse(x as String))
            .toList(),
    addCode: data['addCode'] as String?,
    removeCode: ((data['removeCode'] as List<dynamic>?) ?? <String>[])
        .map((x) => x as String)
        .toList(),
    enableAll: data['enableAll'] as bool? ?? false,
  );
}

class _Annotation {
  final List<DuplicatorMode> forbiddenDuplicatorModes;
  final String? addCode;
  final List<String> removeCode;
  final bool enableAll;

  const _Annotation({
    this.forbiddenDuplicatorModes = const [],
    this.addCode,
    this.removeCode = const [],
    this.enableAll = false,
  });
}

enum Package {
  pureDart,
  pureDartPde,
}
