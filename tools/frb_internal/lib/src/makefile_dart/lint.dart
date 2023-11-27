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

final _exec = SimpleExecutor(
  env: {
    'CARGO_TERM_COLOR': 'always',
  },
  // Use project root directory
  pwd: Directory.current.parent.parent.path,
);

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
  await _exec('cd $package && cargo fmt ${config.fix ? "" : "--check"}');
  await _exec('cd $package && cargo clippy ${config.fix ? "--fix" : ""} -- -D warnings');
}

Future<void> lintRustWasm(LintConfig config) async {
  await _exec('rustup target add wasm32-unknown-unknown');
  await _exec(
      'cd frb_rust && cargo clippy --target wasm32-unknown-unknown ${config.fix ? "--fix" : ""} -- -D warnings');
}

Future<void> lintDart(LintConfig config) async {
  // await dartPubGet();
  for (final package in kDartPackages) {
    await lintDartMain(config, package);
  }
  await lintDartPana(config);
}

Future<void> lintDartMain(LintConfig config, String package) async {
  final lineLength = package == 'frb_dart' ? 80 : 120;
  await _exec('cd $package && '
      'dart format --line-length $lineLength ${config.fix ? "--fix" : "--output=none --set-exit-if-changed"} .');
  await _exec('cd $package && dart analyze --fatal-infos');
}

Future<void> lintDartPana(LintConfig config) async {
  final pana = Platform.isWindows ? 'pana.bat' : 'pana';
  await _exec('flutter pub global activate pana');
  await _exec('cd frb_dart && $pana --no-warning --line-length 80 --exit-code-threshold 0');
}

Future<void> dartPubGet() async {
  for (final package in kDartPackages) {
    // TODO `with_flutter` is `flutter pub get`
    await _exec('cd $package && dart pub get');
  }
}
