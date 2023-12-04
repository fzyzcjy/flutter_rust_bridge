import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'lint.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand(
        'lint', lint, _$populateLintConfigParser, _$parseLintConfigResult),
    SimpleConfigCommand('lint-rust', lintRust, _$populateLintConfigParser,
        _$parseLintConfigResult),
    SimpleConfigCommand('lint-dart', lintDart, _$populateLintConfigParser,
        _$parseLintConfigResult),
  ];
}

@CliOptions()
class LintConfig {
  @CliOption(defaultsTo: false)
  final bool fix;

  const LintConfig({
    required this.fix,
  });
}

Future<void> lint(LintConfig config) async {
  await lintRust(config);
  await lintDart(config);
}

Future<void> lintRust(LintConfig config) async {
  await lintRustFormat(config);
  await lintRustClippy(config);
}

Future<void> lintRustFormat(LintConfig config) async {
  for (final package in kRustPackages) {
    await exec('cargo +nightly fmt ${config.fix ? "" : "--check"}',
        relativePwd: package);
  }
}

Future<void> lintRustClippy(LintConfig config) async {
  for (final package in kRustPackages) {
    if (config.fix) {
      await exec('cargo fix --allow-dirty', relativePwd: package);
    }
    await exec(
        'cargo clippy ${config.fix ? "--fix --allow-dirty" : ""} -- -D warnings',
        relativePwd: package);
  }

  await exec(
      'cargo clippy --target wasm32-unknown-unknown ${config.fix ? "--fix --allow-dirty" : ""} -- -D warnings',
      relativePwd: 'frb_rust');
}

Future<void> lintDart(LintConfig config) async {
  await lintDartFormat(config);
  await lintDartAnalyze(config);
  await lintDartPana(config);
}

Future<void> lintDartFormat(LintConfig config) async {
  for (final package in kDartPackages) {
    await exec('dart format ${config.fix ? "" : "--set-exit-if-changed"} .',
        relativePwd: package);
  }
}

Future<void> lintDartAnalyze(LintConfig config) async {
  for (final package in kDartPackages) {
    await runPubGetIfNotRunYet(package);
    await exec('dart ${config.fix ? "fix --apply" : "analyze --fatal-infos"}',
        relativePwd: package);
  }
}

Future<void> lintDartPana(LintConfig config) async {
  final pana = Platform.isWindows ? 'pana.bat' : 'pana';
  await exec('flutter pub global activate pana');
  await exec('$pana --no-warning --line-length 80 --exit-code-threshold 0',
      relativePwd: 'frb_dart');
}
