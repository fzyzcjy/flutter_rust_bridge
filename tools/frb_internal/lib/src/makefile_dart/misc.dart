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
  await lintDartAnalyze(const LintConfig(fix: true));
  await lintRustClippy(const LintConfig(fix: true));

  // format after clippy, since cargo fix may remove a import line, but leave
  // the result unformatted
  await lintDartFormat(const LintConfig(fix: true));
  await lintRustFormat(const LintConfig(fix: true));

  await miscNormalizePubspec();

  if (config.mode == PrecommitMode.slow) {
    await pubGetAll();

    await generateInternal(const GenerateConfig(setExitIfChanged: false));
   
    for (final package in kDartExamplePackages) {
      await generateRunFrbCodegenCommandGenerate(
          GeneratePackageConfig(setExitIfChanged: false, package: package));
    }

    await testRust(const TestRustConfig(updateGoldens: true));
  }
}

Future<void> pubGetAll() async {
  for (final package in kDartPackages) {
    await exec('${kDartModeOfPackage[package]!.name} pub get',
        relativePwd: package);
  }
}
