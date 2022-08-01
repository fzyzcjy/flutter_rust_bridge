import 'dart:convert';
import 'dart:io';

import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart';
import 'package:shelf_static/shelf_static.dart';
import 'package:args/args.dart';

Future<String> system(
  String command,
  List<String> arguments, {
  String? pwd,
  Map<String, String>? env,
  bool shell = true,
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
    stdout.write(line);
    ret.add(line);
  });
  process.stderr.transform(utf8.decoder).listen((line) {
    stderr.write(line);
    err.add(line);
  });
  final exitCode = await process.exitCode;
  if (exitCode != 0) {
    throw ProcessException(command, arguments, err.join(''), exitCode);
  }
  return ret.join('');
}

void bail(bool condition, [String? message]) {
  if (!condition) {
    if (message != null) print('Error: $message');
    exit(1);
  }
}

void main(List<String> args) async {
  final parser = ArgParser()
    ..addSeparator('frb_cors: A simple static file server.')
    ..addOption('port',
        abbr: 'p', help: 'HTTP port to listen to', defaultsTo: '8080')
    ..addOption('root',
        abbr: 'r', help: 'Root of the Flutter output', defaultsTo: 'build/web')
    ..addOption('crate',
        abbr: 'c', help: 'Directory of the crate', defaultsTo: 'native')
    ..addFlag('release', help: 'Compile in release mode')
    ..addFlag('help', abbr: 'h');
  final config = parser.parse(args);
  if (config['help']) {
    print(parser.usage);
    return;
  }

  final crateDir = config['crate'];
  bail(await File('$crateDir/Cargo.toml').exists(),
      '$crateDir is not a crate dir.');
  final crateName = await system(
    'sh',
    ['-c', "cargo read-manifest | jq -r '.name'"],
    pwd: crateDir,
    shell: false,
  );
  bail(crateName.trim().isNotEmpty, 'Crate name cannot be empty.');
  await system(
    'wasm-pack',
    [
      'build', '-t', 'no-modules', '-d', '../web/pkg', '--no-typescript',
      if (!config['release']) '--dev', '.',
      '--', // cargo build args
      '-Z', 'build-std=std,panic_abort'
    ],
    pwd: crateDir,
    env: {
      'FRB_JS': 'pkg/${crateName.trim()}',
      'RUSTUP_TOOLCHAIN': 'nightly',
      'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
      if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
    },
  );
  await system('flutter',
      ['build', 'web', if (config['release']) '--release'] + config.rest);

  // Use any available host or container IP (usually `0.0.0.0`).
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
    config['root'],
    defaultDocument: 'index.html',
  ));

  // For running in containers, we respect the PORT environment variable.
  final port = int.parse(Platform.environment['PORT'] ?? config['port']);
  final addr = 'http://localhost:$port';
  await serve(handler, ip, port);
  print('🦀🎯 Server listening on $addr');
  if (Platform.isWindows) {
    system('start', [addr]);
  } else {
    system('open', [addr]);
  }
}
