// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'codegen.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

CodegenConfig _$parseCodegenConfigResult(ArgResults result) => CodegenConfig(
      setExitIfChanged: result['set-exit-if-changed'] as bool,
    );

ArgParser _$populateCodegenConfigParser(ArgParser parser) => parser
  ..addFlag(
    'set-exit-if-changed',
  );

final _$parserForCodegenConfig = _$populateCodegenConfigParser(ArgParser());

CodegenConfig parseCodegenConfig(List<String> args) {
  final result = _$parserForCodegenConfig.parse(args);
  return _$parseCodegenConfigResult(result);
}
