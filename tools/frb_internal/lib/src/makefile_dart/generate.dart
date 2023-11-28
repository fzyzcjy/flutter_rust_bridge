import 'dart:io';
import 'dart:math';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart'
    as frb_example_pure_dart_generator;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'generate.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('generate-internal', generateInternal,
        _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-run-frb-codegen-command-generate',
        generateRunFrbCodegenCommandGenerate,
        _$populateGeneratePackageConfigParser,
        _$parseGeneratePackageConfigResult),
    SimpleConfigCommand(
        'generate-run-frb-codegen-command-integrate',
        generateRunFrbCodegenCommandIntegrate,
        _$populateGeneratePackageConfigParser,
        _$parseGeneratePackageConfigResult),
    // more detailed command, can be used to execute just a portion of the main command
    SimpleConfigCommand(
        'generate-internal-frb-example-pure-dart',
        generateInternalFrbExamplePureDart,
        _$populateGenerateConfigParser,
        _$parseGenerateConfigResult),
    SimpleConfigCommand('generate-internal-rust', generateInternalRust,
        _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-internal-dart-source',
        generateInternalDartSource,
        _$populateGenerateConfigParser,
        _$parseGenerateConfigResult),
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

@CliOptions()
class GeneratePackageConfig implements GenerateConfig {
  @override
  @CliOption(defaultsTo: false)
  final bool setExitIfChanged;
  final String package;

  const GeneratePackageConfig({
    required this.setExitIfChanged,
    required this.package,
  });
}

Future<void> generateInternal(GenerateConfig config) async {
  await generateInternalFrbExamplePureDart(config);
  await generateInternalRust(config);
  await generateInternalBookHelp(config);
  await generateInternalDartSource(config);
  await generateInternalBuildRunner(config);
}

Future<void> generateInternalFrbExamplePureDart(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    await frb_example_pure_dart_generator.generate();
  });
}

Future<void> generateInternalDartSource(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    final path = '${Directory.systemTemp.path}/${Random().nextInt(1000000000)}';
    await exec('''
    #!/usr/bin/env bash
    set -eux
    mkdir -p $path && cd $path

    git clone --depth 1 --filter=blob:none --sparse --branch stable https://github.com/dart-lang/sdk.git
    (cd sdk && git sparse-checkout set runtime/include)
    cp -rf ./sdk/runtime/include/* ${exec.pwd}frb_rust/src/dart_api/
    rm -rf sdk
  ''');
  });
}

Future<void> generateInternalRust(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    for (final package in kDartPackages) {
      await runDartPubGetIfNotRunYet(package);
    }

    await exec('cargo run -- internal-generate', relativePwd: 'frb_codegen');
  });
}

Future<void> generateInternalBookHelp(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    for (final (cmd, extraArgs) in [
      ('', ''),
      ('generate', ''),
      ('create', ''),
      ('integrate', ''),
      ('build-web', '--dart-root ${exec.pwd}frb_example/pure_dart'),
    ]) {
      await exec(
          'cargo run -- $cmd $extraArgs --help > ${exec.pwd}book/src/generated/${cmd.isEmpty ? "main" : cmd}.txt',
          relativePwd: 'frb_codegen');
    }
  });
}

Future<void> generateInternalBuildRunner(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    for (final package in kDartNonExamplePackages) {
      await runDartPubGetIfNotRunYet(package);
      await exec('dart run build_runner build --delete-conflicting-outputs',
          relativePwd: package);
    }
  });
}

Future<void> generateRunFrbCodegenCommandGenerate(
    GeneratePackageConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    await runDartPubGetIfNotRunYet(config.package);
    await exec(
      'cargo run --manifest-path ${exec.pwd}/frb_codegen/Cargo.toml -- generate',
      relativePwd: config.package,
      extraEnv: {'RUST_BACKTRACE': '1'},
    );
  });
}

Future<void> generateRunFrbCodegenCommandIntegrate(
    GeneratePackageConfig config) async {
  TODO;
}

Future<void> _wrapMaybeSetExitIfChanged(
    GenerateConfig config, Future<void> Function() inner) async {
  // Before actually executing anything, check whether git repository is already dirty
  await _maybeSetExitIfChanged(config);
  await inner();
  // The real check
  await _maybeSetExitIfChanged(config);
}

Future<void> _maybeSetExitIfChanged(GenerateConfig config) async {
  if (config.setExitIfChanged) {
    await exec('git diff --exit-code');
  }
}
