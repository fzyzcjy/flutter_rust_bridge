// ignore_for_file: avoid_print

import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/lint.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'misc.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('misc-normalize-pubspec', miscNormalizePubspec),
    SimpleConfigCommand('precommit', precommit, _$populatePrecommitConfigParser,
        _$parsePrecommitConfigResult),
    SimpleCommand('precommit-generate', precommitGenerate),
    SimpleCommand('precommit-integrate', precommitIntegrate),
    SimpleCommand('pub-get-all', pubGetAll),
    SimpleCommand('cargo-fetch-all', cargoFetchAll),
  ];
}

Future<void> miscNormalizePubspec() async {
  print('Execute miscNormalizePubspec');
  for (final package in kDartPackages) {
    final file = File('${exec.pwd}$package/pubspec.lock');
    final original = file.readAsStringSync();
    final modified = original.replaceAll('pub.flutter-io.cn', 'pub.dev');
    if (modified == original) continue;
    file.writeAsStringSync(modified);
  }
}

enum PrecommitMode { fast, slow }

@CliOptions()
class PrecommitConfig {
  final PrecommitMode mode;

  const PrecommitConfig({
    required this.mode,
  });
}

Future<void> precommit(PrecommitConfig config) async {
  if (config.mode == PrecommitMode.slow) {
    await precommitGenerate();
    await precommitIntegrate();
  }

  // format after clippy, since cargo fix may remove a import line, but leave
  // the result unformatted
  await Future.wait([
    (() async {
      await lintDartAnalyze(const LintConfig(fix: true));
      await lintDartFormat(const LintConfig(fix: true));
    })(),
    (() async {
      await lintRustClippy(const LintConfig(fix: true));
      await lintRustFormat(const LintConfig(fix: true));
    })(),
  ]);

  if (config.mode == PrecommitMode.slow) {
    await generateInternal(
        const GenerateConfig(setExitIfChanged: false, coverage: false),
        canSkipAllContributor: true);
    await testRust(const TestRustConfig(updateGoldens: true, coverage: false));
    await pubGetAll();
    await cargoFetchAll();
  }

  await miscNormalizePubspec();
}

Future<void> precommitGenerate() async {
  await Future.wait([
    for (final package in kDartExamplePackages)
      generateRunFrbCodegenCommandGenerate(GeneratePackageConfig(
          setExitIfChanged: false, package: package, coverage: false)),
  ]);
}

Future<void> precommitIntegrate() async {
  await Future.wait([
    for (final package in kDartExampleIntegratePackages)
      generateRunFrbCodegenCommandIntegrate(GeneratePackageConfig(
          setExitIfChanged: false, package: package, coverage: false)),
  ]);
}

Future<void> pubGetAll() async {
  for (final package in kDartPackages) {
    await exec('${kDartModeOfPackage[package]!.name} pub get',
        relativePwd: package);
  }
}

Future<void> cargoFetchAll() async {
  for (final package in kRustPackages) {
    await exec('cargo fetch', relativePwd: package);
  }
}

String convertConfigPackage(String raw) {
  final ans = raw.replaceAll('--', '/');
  if (raw != ans) print('convertConfigPackage: $raw -> $ans');
  return ans;
}

final class FeatureConfiguration {
  final bool defaultFeatures;
  final String? features;

  const FeatureConfiguration({
    required this.features,
    this.defaultFeatures = true,
  });

  String get toCargoArgs =>
      "${features != null ? "--features $features" : ""} ${defaultFeatures ? "" : "--no-default-features"}";
}

List<FeatureConfiguration> getRustFeaturesOfPackage(String package) {
  if (package == "frb_example/pure_dart_pde/rust" ||
      package == "frb_example/pure_dart/rust" ||
      package == "frb_example/pure_dart" ||
      package == "frb_example/pure_dart_pde") {
    return const [
      FeatureConfiguration(features: "internal_feature_for_testing")
    ];
  } else if (package == "frb_example/dart_minimal") {
    return const [
      FeatureConfiguration(features: null),
      FeatureConfiguration(features: "rust-async")
    ];
  } else {
    return const [FeatureConfiguration(features: null)];
  }
}
