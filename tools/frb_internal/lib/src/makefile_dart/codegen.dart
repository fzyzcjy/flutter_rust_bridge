import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart' as generator;
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'codegen.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('codegen', codegen, _$populateCodegenConfigParser, _$parseCodegenConfigResult),
    SimpleConfigCommand('codegen-internal', codegenInternal, _$populateCodegenConfigParser, _$parseCodegenConfigResult),
    SimpleConfigCommand('codegen-main', codegenMain, _$populateCodegenConfigParser, _$parseCodegenConfigResult),
  ];
}

@CliOptions()
class CodegenConfig {
  @CliOption(defaultsTo: false)
  final bool setExitIfChanged;

  const CodegenConfig({
    required this.setExitIfChanged,
  });
}

Future<void> codegen(CodegenConfig config) async {
  await codegenInternal(config);
  await codegenMain(config);
}

Future<void> codegenInternal(CodegenConfig config) async {
  await generator.generate();
  _maybeSetExitIfChanged(config);
}

Future<void> codegenMain(CodegenConfig config) async {
  TODO;
}

void _maybeSetExitIfChanged(CodegenConfig config) {
  TODO;
}
