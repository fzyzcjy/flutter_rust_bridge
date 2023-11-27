import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'lint.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('lint', lint, _$populateLintConfigParser, _$parseLintConfigResult),
    SimpleConfigCommand('lint-rust', lintRust, _$populateLintConfigParser, _$parseLintConfigResult),
    SimpleConfigCommand('lint-dart', lintDart, _$populateLintConfigParser, _$parseLintConfigResult),
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
  for (final package in kRustPackages) {
    await lintRustMain(config, package);
  }
  await lintRustWasm(config);
}

Future<void> lintRustMain(LintConfig config, String package) async {
  await exec('cargo fmt ${config.fix ? "" : "--check"}', relativePwd: package);
  await exec('cargo clippy ${config.fix ? "--fix" : ""} -- -D warnings', relativePwd: package);
}

Future<void> lintRustWasm(LintConfig config) async {
  await exec('rustup target add wasm32-unknown-unknown');
  await exec('cargo clippy --target wasm32-unknown-unknown ${config.fix ? "--fix" : ""} -- -D warnings',
      relativePwd: 'frb_rust');
}

Future<void> lintDart(LintConfig config) async {
  // await dartPubGet();
  for (final package in kDartPackages) {
    await lintDartFormat(config, package);
    await lintDartAnalyze(config, package);
  }
  await lintDartPana(config);
}

Future<void> lintDartFormat(LintConfig config, String package) async {
  final lineLength = package == 'frb_dart' ? 80 : 120;
  await exec('dart format --line-length $lineLength ${config.fix ? "" : "--set-exit-if-changed"} .',
      relativePwd: package);
}

Future<void> lintDartAnalyze(LintConfig config, String package) async {
  await exec('dart analyze --fatal-infos', relativePwd: package);
}

Future<void> lintDartPana(LintConfig config) async {
  final pana = Platform.isWindows ? 'pana.bat' : 'pana';
  await exec('flutter pub global activate pana');
  await exec('$pana --no-warning --line-length 80 --exit-code-threshold 0', relativePwd: 'frb_dart');
}

Future<void> dartPubGet() async {
  for (final package in kDartPackages) {
    // TODO `with_flutter` is `flutter pub get`
    await exec('dart pub get', relativePwd: package);
  }
}
