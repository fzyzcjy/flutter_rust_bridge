// ignore_for_file: avoid_print

import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/serve_web/config.dart';
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart';
import 'package:shelf_static/shelf_static.dart';

Future<void> runServer(Config config) async {
  final staticFilesHandler = createStaticHandler(config.webRoot, defaultDocument: 'index.html');

  // TODO these may not be needed, since we use `dart test` now?
  // Browser? browser;
  // Test helper.
  // final socketHandler = webSocketHandler((WebSocketChannel channel) async {
  //   await for (final mes in channel.stream) {
  //     try {
  //       final data = jsonDecode(mes);
  //       if (data is Map && data.containsKey('__result__')) {
  //         await browser?.close();
  //         exit(data['__result__'] ? 0 : 1);
  //       } else {
  //         print(data);
  //       }
  //     } catch (err, st) {
  //       print('$err\nStacktrace:\n$st');
  //     }
  //   }
  // });

  // TODO
  // final shouldRelaxCoep = config.shouldRelaxCoep;
  final shouldRelaxCoep = true;

  final handler = const Pipeline().addMiddleware((handler) {
    return (req) async {
      final res = await handler(req);
      return res.change(headers: {
        'Cross-Origin-Opener-Policy': 'same-origin',
        'Cross-Origin-Embedder-Policy': shouldRelaxCoep ? 'credentialless' : 'require-corp',
      });
    };
  }).addHandler(Cascade()
      // .add(socketHandler) // TODO
      .add(staticFilesHandler)
      .handler);

  // TODO
  // final portEnv = Platform.environment['PORT'];
  // final port = portEnv == null ? config.port : int.parse(portEnv);

  final port = config.port;
  final addr = 'http://localhost:$port';
  await serve(handler, InternetAddress.anyIPv4, port);
  print('🦀 Server listening on $addr 🎯');

  // TODO these may not be needed, since we use `dart test` now?
  // if (config.runTests) {
  //   browser = await puppeteer.launch(
  //     headless: true,
  //     timeout: const Duration(minutes: 5),
  //   );
  //   final page = await browser.newPage();
  //   await page.goto(addr);
  // } else if (config.open) {
  //   runCommand(_kOpen, [addr]);
  // }
}

// TODO these may not be needed, since we use `dart test` now?
// final _kOpen = const {
//   'linux': 'xdg-open',
//   'macos': 'open',
//   'windows': 'start',
// }[Platform.operatingSystem] ??
//     'open';
//
