// ignore_for_file: avoid_print

import 'dart:io';

// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge_utils/src/commands/serve_web_command.dart';
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart';
import 'package:shelf_static/shelf_static.dart';

Future<String> runServer(ServeWebConfig config, {Handler? extraHandler}) async {
  final staticFilesHandler = createStaticHandler(config.webRoot, defaultDocument: 'index.html');

  // final shouldRelaxCoep = config.shouldRelaxCoep;
  final shouldRelaxCoep = true;

  final innerHandler = Cascade();
  if (extraHandler != null) innerHandler.add(extraHandler);
  innerHandler.add(staticFilesHandler);

  final handler = const Pipeline().addMiddleware((handler) {
    return (req) async {
      print('Request: ${req.method} ${req.requestedUri}');
      final res = await handler(req);
      print('Response: code=${res.statusCode} mimeType=${res.mimeType}');
      return res.change(headers: {
        'Cross-Origin-Opener-Policy': 'same-origin',
        'Cross-Origin-Embedder-Policy': shouldRelaxCoep ? 'credentialless' : 'require-corp',
        // TODO rm
        // // Disable CORS since this server (hosting JS/WASM) is different from
        // // the server that `dart test -p chrome` creates.
        // 'Access-Control-Allow-Origin': '*',
      });
    };
  }).addHandler(innerHandler.handler);

  // TODO
  // final portEnv = Platform.environment['PORT'];
  // final port = portEnv == null ? config.port : int.parse(portEnv);

  final port = config.port;
  final addr = 'http://localhost:$port';
  await serve(handler, InternetAddress.anyIPv4, port);
  print('🦀 Server listening on $addr with content from ${config.webRoot} 🎯');

  if (config.open) {
    runCommand(_kOpen, [addr]);
  }

  return addr;
}

final _kOpen = const {
      'linux': 'xdg-open',
      'macos': 'open',
      'windows': 'start',
    }[Platform.operatingSystem] ??
    'open';
