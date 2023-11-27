/// Similar to `makefile`/`justfile`/..., but based on Dart
/// (Why not directly use justfile: Because want more flexible grammar, such as for loops)
library;

import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'entrypoint.g.dart';

const _kRustPackages = <String>[
  'frb_rust',
  'frb_codegen',
  'frb_example/dart_minimal/rust',
  'frb_example/pure_dart/rust',
  // TODO `with_flutter` example
];

const _kDartPackages = <String>[
  'frb_dart',
  'frb_example/dart_minimal',
  'frb_example/pure_dart',
  // TODO `with_flutter` example
  'frb_utils',
  'tools/frb_internal',
];

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
  for (final package in _kRustPackages) {
    await lintRustMain(config, package);
  }
  await lintRustWasm(config);
}

Future<void> lintRustMain(LintConfig config, String package) async {
  await execute('cd $package && cargo fmt ${config.fix ? "" : "--check"}');
  await execute('cd $package && cargo clippy ${config.fix ? "--fix" : ""} -- -D warnings');
}

Future<void> lintRustWasm(LintConfig config) async {
  await execute('rustup target add wasm32-unknown-unknown');
  await execute(
      'cd frb_rust && cargo clippy --target wasm32-unknown-unknown ${config.fix ? "--fix" : ""} -- -D warnings');
}

Future<void> lintDart(LintConfig config) async {
  await dartPubGet();
  for (final package in _kDartPackages) {
    await lintDartMain(config, package);
  }
  await lintDartPana(config);
}

Future<void> lintDartMain(LintConfig config, String package) async {
  final lineLength = package == 'frb_dart' ? 80 : 120;
  execute('cd $package && '
      'dart format --line-length $lineLength ${config.fix ? "--fix" : "--output=none --set-exit-if-changed"}');
  execute('cd $package && dart analyze --fatal-infos');
}

Future<void> lintDartPana(LintConfig config) async {
  final pana = Platform.isWindows ? 'pana.bat' : 'pana';
  await execute('flutter pub global activate pana');
  await execute('cd frb_dart && $pana --no-warning --line-length 80 --exit-code-threshold 0');
}

Future<void> dartPubGet() async {
  for (final package in _kDartPackages) {
    // TODO `with_flutter` is `flutter pub get`
    await execute('cd $package && dart pub get');
  }
}
