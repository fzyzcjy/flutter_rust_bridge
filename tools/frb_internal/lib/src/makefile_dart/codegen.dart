import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart' as generator;
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'codegen.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('codegen-internal', codegenInternal, _$populateCodegenConfigParser, _$parseCodegenConfigResult),
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

Future<void> codegenInternal(CodegenConfig config) async {
  await generator.generate();
  _maybeSetExitIfChanged(config);
}

void _maybeSetExitIfChanged(CodegenConfig config) {
  TODO;
}
