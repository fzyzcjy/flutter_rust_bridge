import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';

/// Similar to `makefile`/`justfile`/..., but based on Dart
/// (Why not directly use justfile: Because want more flexible grammar, such as for loops)

const _kRustPackages = <String>[TODO];
const _kDartPackages = <String>[TODO];

class LintConfig {
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
    await lintRustMain(config);
  }
  await lintRustWasm(config);
}

Future<void> lintRustMain(LintConfig config) async {
  await execute('cargo fmt ${config.fix ? "" : "--check"}');
  await execute('cargo clippy ${config.fix ? "--fix" : ""} -- -D warnings');
}

Future<void> lintRustWasm(LintConfig config) async {
  await execute('rustup target add wasm32-unknown-unknown');
  await execute(
      'cd frb_rust && cargo clippy --target wasm32-unknown-unknown ${config.fix ? "--fix" : ""} -- -D warnings');
}

Future<void> lintDart(LintConfig config) async {
  await dartPubGet();
  for (final package in _kDartPackages) {
    await lintDartMain(config);
  }
  await lintDartPana(config);
}

Future<void> lintDartMain(LintConfig config) async {
  execute('cd $directory && dart format '
      '--line-length $line_length ${config.fix ? "--fix" : "--output=none --set-exit-if-changed"}');
  execute('cd $directory && $executable analyze --fatal-infos');
}

Future<void> lintDartPana(LintConfig config) async {
  final pana = Platform.isWindows ? 'pana.bat' : 'pana';

  await execute('flutter pub global activate pana');
  await execute('cd frb_dart && $pana --no-warning --line-length 80 --exit-code-threshold 0');
}

Future<void> dartPubGet() async {
  TODO;
}

Future<void> execute(String command) async {
  TODO;
}
