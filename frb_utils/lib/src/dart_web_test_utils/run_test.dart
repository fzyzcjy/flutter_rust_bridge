import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge_utils/src/commands/serve_web_command.dart';
import 'package:flutter_rust_bridge_utils/src/commands/test_web_command.dart';
import 'package:flutter_rust_bridge_utils/src/serve_web/run_server.dart';
import 'package:puppeteer/puppeteer.dart';
import 'package:shelf/src/handler.dart';
import 'package:shelf_web_socket/shelf_web_socket.dart';

const kTestResultKey = '__result__';

Future<void> executeTestWeb(TestWebConfig options) async {
  TODO_compile;

  Browser? browser;

  await runServer(
    ServeWebConfig(
      webRoot: webRoot,
      port: ServeWebConfig.kDefaultPort,
      open: false,
    ),
    extraHandler: _createWebSocketHandler(
      closeBrowser: () async => browser?.close(),
    ),
  );

  print('runTests: puppeteer.launch');
  browser = await puppeteer.launch(headless: true, timeout: const Duration(minutes: 5));

  print('runTests: browser.newPage');
  final page = await browser.newPage();

  print('runTests: page.goto($addr)');
  await page.goto(addr);
}

Handler _createWebSocketHandler({required Future<void> Function() closeBrowser}) {
  return webSocketHandler((channel) async {
    await for (final mes in channel.stream) {
      try {
        final data = jsonDecode(mes);
        if (data is Map && data.containsKey(kTestResultKey)) {
          await closeBrowser();
          exit(data[kTestResultKey] ? 0 : 1);
        } else {
          print(data);
        }
      } catch (err, st) {
        print('$err\nStacktrace:\n$st');
      }
    }
  });
}
