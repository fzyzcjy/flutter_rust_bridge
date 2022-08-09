import 'dart:convert';
import 'dart:io';

import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart';
import 'package:shelf_static/shelf_static.dart';
import 'package:args/args.dart';
import 'package:path/path.dart' as p;

final which = Platform.isWindows ? 'where.exe' : 'which';
final open = Platform.isWindows ? 'start' : 'open';

/// Wrap text in bold red.
String err(String msg) => stderr.supportsAnsiEscapes ? '\x1b[1;31m$msg\x1b[0m' : msg;

void eprintln([Object? msg = '']) {
  stderr.writeln('${err('error')}: $msg');
}

Future<String> system(
  String command,
  List<String> arguments, {
  String? pwd,
  Map<String, String>? env,
  bool shell = true,
  bool silent = false,
}) async {
  print('> $command ${arguments.join(' ')}');
  final process = await Process.start(
    command,
    arguments,
    runInShell: shell,
    workingDirectory: pwd,
    environment: env,
  );
  final ret = <String>[];
  final err = <String>[];
  process.stdout.transform(utf8.decoder).listen((line) {
    if (!silent) stdout.write(line);
    ret.add(line);
  });
  process.stderr.transform(utf8.decoder).listen((line) {
    if (!silent) stderr.write(line);
    err.add(line);
  });
  final exitCode = await process.exitCode;
  if (exitCode != 0) {
    throw ProcessException(command, arguments, err.join(''), exitCode);
  }
  return ret.join('');
}

void assert_(bool condition, [String? message]) {
  if (!condition) {
    eprintln(message);
    exit(1);
  }
}

void main(List<String> args) async {
  final exec = Platform.script.pathSegments.last;
  final parser = ArgParser()
    ..addSeparator('Develop Rust WASM modules with cross-origin isolation.')
    ..addSeparator('Usage: $exec -w=<WASM> [OPTIONS]')
    ..addOption('port', abbr: 'p', help: 'HTTP port to listen to', defaultsTo: '8080')
    ..addOption('root', abbr: 'r', help: 'Root of the Flutter/Dart output.')
    ..addOption('crate', abbr: 'c', help: 'Directory of the crate', defaultsTo: 'native')
    ..addOption('dart-input', help: 'Run "dart compile" with the specified input instead of "flutter build".')
    ..addOption('wasm-output', help: 'WASM output path.')
    ..addFlag('release', help: 'Compile in release mode')
    ..addFlag('weak-refs', help: 'Enable the weak references proposal\nRequires wasm-bindgen in path.')
    ..addFlag('reference-types', help: 'Enable the reference types proposal\nRequires wasm-bindgen in path.')
    ..addFlag('help', abbr: 'h', help: 'Print this help message');
  final config = parser.parse(args);
  if (config['help']) {
    print(parser.usage);
    return;
  }

  await system(which, ['wasm-pack']).catchError((_) {
    eprintln(
      'wasm-pack is required, but not found in the path.\n'
      'Please install wasm-pack by following the instructions at https://rustwasm.github.io/wasm-pack/\n'
      'or running `cargo install wasm-pack`.',
    );
    exit(1);
  });

  final shouldRunBindgen = config['weak-refs'] || config['reference-types'];

  if (shouldRunBindgen) {
    await system(which, ['wasm-bindgen']).catchError((_) {
      eprintln(
        'wasm-bindgen flags are enabled, but wasm-bindgen could not be found in the path.\n'
        'Please install wasm-bindgen using `cargo install -f wasm-bindgen-cli`.',
      );
      exit(1);
    });
  }

  final String root;
  final String wasmOutput;
  if (config['dart-input'] != null) {
    assert_(config['root'] != null, 'The --root option is required when building plain Dart projects.');
    root = p.canonicalize(config['root']);
    wasmOutput = p.canonicalize(config['wasm-output'] ?? '$root/pkg');
  } else {
    assert_(config['wasm-output'] != null, 'The --wasm-output option is required when building Flutter projects.');
    root = p.canonicalize(config['root'] ?? 'build/web');
    wasmOutput = p.canonicalize(config['wasm-output']);
  }

  final crateDir = config['crate'];
  assert_(
    await File('$crateDir/Cargo.toml').exists(),
    '$crateDir is not a crate directory.\n'
    'Please specify the crate directory using "--crate <CRATE>".',
  );

  // --- Checks end ---

  final manifest = await system(
    'cargo',
    ['read-manifest'],
    pwd: crateDir,
    silent: true,
  );
  final crateName = (json.decode(manifest)['name'] as String).trim();
  assert_(crateName.isNotEmpty, 'Crate name cannot be empty.');
  await system(
    'wasm-pack',
    [
      'build', '-t', 'no-modules', '-d', wasmOutput, '--no-typescript',
      '--out-name', crateName,
      if (!config['release']) '--dev', '.',
      '--', // cargo build args
      '-Z', 'build-std=std,panic_abort'
    ],
    pwd: crateDir,
    env: {
      'FRB_JS': 'pkg/$crateName',
      'RUSTUP_TOOLCHAIN': 'nightly',
      'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
      if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
    },
  );
  if (shouldRunBindgen) {
    await system('wasm-bindgen', [
      '$crateDir/target/wasm32-unknown-unknown/${config['release'] ? 'release' : 'debug'}/$crateName.wasm',
      '--out-dir',
      'web/pkg',
      '--no-typescript',
      '--target',
      'no-modules',
      if (config['weak-refs']) '--weak-refs',
      if (config['reference-types']) '--reference-types',
    ]);
  }
  if (config['dart-input'] != null) {
    final output = p.basename(config['dart-input']);
    await system('dart', [
      'compile',
      'js',
      '-o',
      '$root/$output.js',
      if (config['release']) '-O2',
      if (stdout.supportsAnsiEscapes) '--enable-diagnostic-colors',
      config['dart-input'],
    ]);
  } else {
    await system(
      'flutter',
      ['build', 'web', if (config['release']) '--release'] + config.rest,
    );
  }

  final ip = InternetAddress.anyIPv4;

  final handler = const Pipeline().addMiddleware((handler) {
    return (req) async {
      final res = await handler(req);
      return res.change(headers: const {
        'Cross-Origin-Opener-Policy': 'same-origin',
        'Cross-Origin-Embedder-Policy': 'credentialless',
      });
    };
  }).addHandler(createStaticHandler(
    root,
    defaultDocument: 'index.html',
  ));

  final port = int.parse(Platform.environment['PORT'] ?? config['port']);
  final addr = 'http://localhost:$port';
  await serve(handler, ip, port);
  print('🦀 Server listening on $addr 🎯');
  system(open, [addr]);
}
