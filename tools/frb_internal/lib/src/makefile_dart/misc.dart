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
    SimpleCommand('pub-get-all', pubGetAll),
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
    await Future.wait([
      for (final package in kDartExamplePackages)
        generateRunFrbCodegenCommandGenerate(GeneratePackageConfig(
            setExitIfChanged: false, package: package, coverage: false)),
    ]);
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
        const GenerateConfig(setExitIfChanged: false, coverage: false));
    await testRust(const TestRustConfig(updateGoldens: true, coverage: false));
    await pubGetAll();
  }

  await miscNormalizePubspec();
}

Future<void> pubGetAll() async {
  for (final package in kDartPackages) {
    await exec('${kDartModeOfPackage[package]!.name} pub get',
        relativePwd: package);
  }
}
