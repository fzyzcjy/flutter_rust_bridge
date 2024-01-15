import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/direct_sources.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:recase/recase.dart';

class DartGenerator extends BaseGenerator {
  DartGenerator({
    required super.packageRootDir,
    required super.interestDir,
    required super.package,
  });

  @override
  Future<void> executeFormat() =>
      executeDartFormat(pwd: packageRootDir.toFilePath());

  @override
  Set<String> get duplicatorBlacklistNames => {};

  @override
  Map<String, String> generateDirectSources() =>
      generateDartDirectSources(package);

  @override
  String get extension => 'dart';

  @override
  String generateDuplicateCode(String inputText, DuplicatorMode mode) {
    return inputText
        // imports
        .replaceAllMapped(
            RegExp(r'src/rust/api/(pseudo_manual/)?(\w+)\.dart'),
            (m) =>
                'src/rust/api/pseudo_manual/${m.group(2)}${mode.postfix}.dart')
        .replaceAll("'../test_utils.dart'", "'../../test_utils.dart'")
        // function call, struct name, etc
        .replaceAll('TwinNormal', ReCase(mode.postfix).pascalCase);
  }

  @override
  String generateDuplicateFileStem(String inputStem, DuplicatorMode mode) {
    final re = RegExp(r'_test$');
    if (!re.hasMatch(inputStem)) throw ArgumentError;
    return inputStem.replaceAll(re, '${mode.postfix}_test');
  }
}
