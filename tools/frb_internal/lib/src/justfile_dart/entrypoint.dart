import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';

/// Similar to `justfile`, but based on Dart
/// (Why not directly use justfile: Because want more flexible grammar, such as for loops)

const _kRustPackages = <String>[TODO];
const _kDartPackages = <String>[TODO];

Future<void> lint() async {
  await lintRust();
  await lintDart();
}

Future<void> lintRust() async {
  for (final package in _kRustPackages) {
    await lintRustMain();
  }
  await lintRustWasm();
}

Future<void> lintRustMain() async {
  await execute('cargo fmt'
      '{{ if mode == "fix" { "" } else { "--check" } }}');
  await execute('cargo clippy '
      '{{ if mode == "fix" { "--fix" } else { "" } }} '
      '-- -D warnings');
}

Future<void> lintRustWasm() async {
  await execute('rustup target add wasm32-unknown-unknown');
  await execute('cd frb_rust && '
      'cargo clippy --target wasm32-unknown-unknown '
      '{{ if mode == "fix" { "--fix" } else { "" } }} '
      '-- -D warnings');
}

Future<void> lintDart() async {
  await dartPubGet();
  for (final package in _kDartPackages) {
    await lintDartMain();
  }
  await lintDartPana();
}

Future<void> lintDartMain() async {
  execute('cd $directory && dart format '
      '--line-length $line_length '
      '{{ if mode == "fix" { "--fix" } else { "--output=none --set-exit-if-changed" } }} '
      '.');

  execute('cd {{directory}} && {{executable}} analyze --fatal-infos');
}

Future<void> lintDartPana() async {
  await execute('flutter pub global activate pana');
  await execute('cd frb_dart && $pana --no-warning --line-length 80 --exit-code-threshold 0');
}

Future<void> dartPubGet() async {
  TODO;
}

Future<void> execute(String command) async {
  TODO;
}
