// ignore_for_file: avoid_print

import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge/src/cli/serve/build_web.dart';
import 'package:flutter_rust_bridge/src/cli/serve/config.dart';
import 'package:flutter_rust_bridge/src/cli/serve/run_server.dart';
import 'package:path/path.dart' as p;

final _kWhich = Platform.isWindows ? 'where.exe' : 'which';

String err(String msg) {
  // return stderr.supportsAnsiEscapes ? Colorize(msg).red().bold().toString() : msg; // #1262
  return msg;
}

void eprint([Object? msg = 'unspecified']) {
  stderr.writeln('${err('error')}: $msg');
}

Never bail([String? message]) {
  eprint(message);
  exit(1);
}

/// {@macro flutter_rust_bridge.internal}
void runCliServe(List<String> args) async {
  final config = parseConfig();

  await runCommand(_kWhich, ['wasm-pack']).catchError((_) {
    bail(
      'wasm-pack is required, but not found in the path.\n'
      'Please install wasm-pack by following the instructions at https://rustwasm.github.io/wasm-pack/\n'
      'or running `cargo install wasm-pack`.',
    );
  });

  if (config.shouldRunBindgen) {
    await runCommand(_kWhich, ['wasm-bindgen']).catchError((_) {
      bail(
        'wasm-bindgen flags are enabled, but wasm-bindgen could not be found in the path.\n'
        'Please install wasm-bindgen using `cargo install -f wasm-bindgen-cli`.',
      );
    });
  }

  final String root;
  final String wasmOutput;
  if (config.dartInput != null) {
    if (config.root == null) {
      bail('The --root option is required when building plain Dart projects.');
    }
    root = p.canonicalize(config.root!);
    wasmOutput = p.canonicalize(config.wasmOutput ?? '$root/pkg');
  } else {
    root = p.canonicalize(config.root ?? 'build/web');
    wasmOutput = p.canonicalize(config.wasmOutput ?? 'web/pkg');
  }

  final crateDir = config.crate;
  if (!await File('$crateDir/Cargo.toml').exists()) {
    bail(
      '$crateDir is not a crate directory.\n'
      'Please specify the crate directory using "--crate <CRATE>".',
    );
  }

  // --- Checks end ---

  if (config.build) {
    await buildWeb(
      config,
      crateDir: crateDir,
      wasmOutput: wasmOutput,
      root: root,
      args: args,
    );
  }

  await runServer(config, root: root);
}
