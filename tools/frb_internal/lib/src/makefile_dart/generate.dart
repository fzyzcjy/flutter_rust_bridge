import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart' as generator;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'generate.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('generate', generate, _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-internal', generateInternal, _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-run-frb-codegen', generateRunFrbCodegen, _$populateGenerateConfigParser, _$parseGenerateConfigResult),
  ];
}

@CliOptions()
class GenerateConfig {
  @CliOption(defaultsTo: false)
  final bool setExitIfChanged;

  const GenerateConfig({
    required this.setExitIfChanged,
  });
}

Future<void> generate(GenerateConfig config) async {
  await generateInternal(config);
  await generateRunFrbCodegen(config);
}

Future<void> generateInternal(GenerateConfig config) async {
  await generator.generate();
  await _maybeSetExitIfChanged(config);

  await generateInternalDartSource(config);
  await _maybeSetExitIfChanged(config);
}

Future<void> generateInternalDartSource(GenerateConfig config) async {
  TODO;
  '''
    #!/usr/bin/env bash
    set -euxo pipefail

    git clone --depth 1 --filter=blob:none --sparse --branch stable https://github.com/dart-lang/sdk.git
    cd sdk
    git sparse-checkout set runtime/include
    cd ..
    cp -rf ./sdk/runtime/include/* ./frb_rust/src/dart_api/
    rm -r sdk
    git diff --exit-code
  ''';
}

Future<void> generateRunFrbCodegen(GenerateConfig config) async {
  // TODO should auto do all these
  // TODO('just install_ffigen_dependency');
  // TODO('just dart_pub_get');
  // TODO('just install_expand');

  TODO('just generate_all');
  await _maybeSetExitIfChanged(config);
}

Future<void> _maybeSetExitIfChanged(GenerateConfig config) async {
  if (config.setExitIfChanged) {
    await exec('git diff --exit-code');
  }
}
