import 'dart:convert';
import 'dart:io';

import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart';
import 'package:shelf_static/shelf_static.dart';
import 'package:path/path.dart' as p;
import 'package:shelf_web_socket/shelf_web_socket.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:puppeteer/puppeteer.dart';
import 'package:colorize/colorize.dart';
import 'package:yaml/yaml.dart';

part 'serve.g.dart';

final YamlMap? pubspec = () {
  final pubspecPath = Platform.script.resolve('../pubspec.yaml');
  final pubpsec = File(pubspecPath.toFilePath());
  try {
    return loadYaml(pubpsec.readAsStringSync(), sourceUrl: pubspecPath);
  } catch (err) {
    eprint('Failed to read pubspec: $err');
  }
}();

String get version => pubspec?['version'] ?? '';

final which = Platform.isWindows ? 'where.exe' : 'which';
final open = const {
      'linux': 'xdg-open',
      'macos': 'open',
      'windows': 'start',
    }[Platform.operatingSystem] ??
    'open';

String err(String msg) =>
    stderr.supportsAnsiEscapes ? Colorize(msg).red().bold().toString() : msg;

void eprint([Object? msg = 'unspecified']) {
  stderr.writeln('${err('error')}: $msg');
}

final arrow =
    stdout.supportsAnsiEscapes ? Colorize('>').green().bold().toString() : '>';

Future<String> system(
  String command,
  List<String> arguments, {
  String? pwd,
  Map<String, String>? env,
  bool shell = true,
  bool silent = false,
}) async {
  print('$arrow $command ${arguments.join(' ')}');
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

Never bail([String? message]) {
  eprint(message);
  exit(1);
}

@CliOptions()
class Opts {
  @CliOption(
    abbr: 'p',
    help: 'HTTP port to listen to',
    valueHelp: 'PORT',
    defaultsTo: 8080,
  )
  late int port;
  @CliOption(
    abbr: 'r',
    help: 'Root of the Flutter/Dart output',
    valueHelp: 'ROOT',
  )
  late String? root;
  @CliOption(
    abbr: 'c',
    help: 'Directory of the crate',
    valueHelp: 'CRATE',
    defaultsTo: 'native',
  )
  late String crate;
  @CliOption(
    abbr: 'd',
    help:
        'Run "dart compile" with the specified input instead of "flutter build"',
    valueHelp: 'ENTRY',
  )
  late String? dartInput;
  @CliOption(abbr: 'w', help: 'WASM output path', valueHelp: 'PKG')
  late String? wasmOutput;
  @CliOption(abbr: 'v', help: 'Display more verbose information')
  late bool verbose;
  @CliOption(
    help: 'Set COEP to credentialless\n'
        'Defaults to true for Flutter',
  )
  late bool relaxCoep;
  late bool relaxCoepWasParsed;
  @CliOption(help: 'Open the webpage in a browser', defaultsTo: true)
  late bool open;
  @CliOption(help: 'Run tests in headless Chromium', negatable: false)
  late bool runTests;
  @CliOption(help: 'Compile in release mode', negatable: false)
  late bool release;
  @CliOption(
    help: 'Enable the weak references proposal\n'
        'Requires wasm-bindgen in path',
  )
  late bool weakRefs;
  @CliOption(
    help: 'Enable the reference types proposal\n'
        'Requires wasm-bindgen in path',
  )
  late bool referenceTypes;
  @CliOption(abbr: 'h', help: 'Print this help message', negatable: false)
  late bool help;
  @CliOption(help: 'Whether to build the library.', defaultsTo: true)
  late bool build;

  static List<String> rest(List<String> args) =>
      _$parserForOpts.parse(args).rest;
}

extension on Opts {
  bool get shouldRunBindgen => weakRefs || referenceTypes;

  /// If not set by user, relax COEP on Flutter.
  bool get shouldRelaxCoep =>
      relaxCoep || (!relaxCoepWasParsed && dartInput == null);
}

void main(List<String> args) async {
  const exec = 'flutter_rust_bridge_serve';
  final config = parseOpts(args);
  if (config.help) {
    print("""
$exec $version
Develop Rust WASM modules with cross-origin isolation.

USAGE:
\t$exec [OPTIONS] [..REST]
\t$exec --dart-input <ENTRY> --root <ROOT> [OPTIONS] [..REST]

OPTIONS:""");
    print(_$parserForOpts.usage);
    return;
  }

  await system(which, ['wasm-pack']).catchError((_) {
    bail(
      'wasm-pack is required, but not found in the path.\n'
      'Please install wasm-pack by following the instructions at https://rustwasm.github.io/wasm-pack/\n'
      'or running `cargo install wasm-pack`.',
    );
  });

  if (config.shouldRunBindgen) {
    await system(which, ['wasm-bindgen']).catchError((_) {
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
    await build(config,
        crateDir: crateDir, wasmOutput: wasmOutput, root: root, args: args);
  }
  await runServer(config, root: root);
}

Future<void> build(
  Opts config, {
  required String crateDir,
  required String wasmOutput,
  required String root,
  required List<String> args,
}) async {
  final manifest = jsonDecode(await system(
    'cargo',
    ['read-manifest'],
    pwd: crateDir,
    silent: true,
  ));
  final String crateName = (manifest['targets'] as List).firstWhere(
      (target) => (target['kind'] as List).contains('cdylib'))['name'];
  if (crateName.isEmpty) bail('Crate name cannot be empty.');
  await system('wasm-pack', [
    'build', '-t', 'no-modules', '-d', wasmOutput, '--no-typescript',
    '--out-name', crateName,
    if (!config.release) '--dev', crateDir,
    '--', // cargo build args
    '-Z', 'build-std=std,panic_abort'
  ], env: {
    'RUSTUP_TOOLCHAIN': 'nightly',
    'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
    if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
  });
  if (config.shouldRunBindgen) {
    await system('wasm-bindgen', [
      '$crateDir/target/wasm32-unknown-unknown/${config.release ? 'release' : 'debug'}/$crateName.wasm',
      '--out-dir',
      wasmOutput,
      '--no-typescript',
      '--target',
      'no-modules',
      if (config.weakRefs) '--weak-refs',
      if (config.referenceTypes) '--reference-types',
    ]);
  }
  if (config.dartInput != null) {
    final output = p.basename(config.dartInput!);
    await system('dart', [
      'compile',
      'js',
      '-o',
      '$root/$output.js',
      if (config.release) '-O2',
      if (stdout.supportsAnsiEscapes) '--enable-diagnostic-colors',
      if (config.verbose) '--verbose',
      config.dartInput!,
    ]);
  } else {
    await system(
      'flutter',
      ['build', 'web', if (!config.release) '--profile'] + Opts.rest(args),
    );
  }
}

Future<void> runServer(Opts config, {required String root}) async {
  final ip = InternetAddress.anyIPv4;

  final staticFilesHandler =
      createStaticHandler(root, defaultDocument: 'index.html');
  Browser? browser;

  // Test helper.
  final socketHandler = webSocketHandler((WebSocketChannel channel) async {
    await for (final mes in channel.stream) {
      try {
        final data = jsonDecode(mes);
        if (data is Map && data.containsKey('__result__')) {
          await browser?.close();
          exit(data['__result__'] ? 0 : 1);
        } else {
          print(data);
        }
      } catch (err, st) {
        print('$err\nStacktrace:\n$st');
      }
    }
  });
  final shouldRelaxCoep = config.shouldRelaxCoep;
  final handler = const Pipeline().addMiddleware((handler) {
    return (req) async {
      final res = await handler(req);
      return res.change(headers: {
        'Cross-Origin-Opener-Policy': 'same-origin',
        'Cross-Origin-Embedder-Policy':
            shouldRelaxCoep ? 'credentialless' : 'require-corp',
      });
    };
  }).addHandler(Cascade().add(socketHandler).add(staticFilesHandler).handler);

  final portEnv = Platform.environment['PORT'];
  final port = portEnv == null ? config.port : int.parse(portEnv);
  final addr = 'http://localhost:$port';
  await serve(handler, ip, port);
  print('🦀 Server listening on $addr 🎯');
  if (config.runTests) {
    browser = await puppeteer.launch(
      headless: true,
      timeout: const Duration(minutes: 5),
    );
    final page = await browser.newPage();
    await page.goto(addr);
  } else if (config.open) {
    system(open, [addr]);
  }
}
