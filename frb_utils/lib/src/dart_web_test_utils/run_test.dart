import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:flutter_rust_bridge_utils/src/commands/serve_web_command.dart';
import 'package:flutter_rust_bridge_utils/src/commands/test_web_command.dart';
import 'package:flutter_rust_bridge_utils/src/serve_web/run_server.dart';
import 'package:puppeteer/puppeteer.dart';
import 'package:shelf/src/handler.dart';
import 'package:shelf_web_socket/shelf_web_socket.dart';

const kTestResultKey = '__result__';

Future<void> executeTestWeb(TestWebConfig options) async {
  print('executeTestWeb: compile');
  await executeBuildWeb(BuildWebArgs(
    dartRoot: TODO,
    output: TODO,
    release: true,
    verbose: false,
    rustCrateDir: TODO,
    cargoBuildArgs: [],
    wasmBindgenArgs: [],
    dartCompileJsEntrypoint: TODO,
  ));

  Browser? browser;

  print('executeTestWeb: runServer');
  final addr = await runServer(
    ServeWebConfig(
      webRoot: TODO,
      port: ServeWebConfig.kDefaultPort,
      open: false,
    ),
    extraHandler: _createWebSocketHandler(
      closeBrowser: () async {
        print('executeTestWeb: browser.close');
        await browser?.close();
      },
    ),
  );
 
  browser = await _browserLaunchAndGoto(addr: addr);
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

Future<Browser> _browserLaunchAndGoto({required String addr}) async {
  print('executeTestWeb: puppeteer.launch');
  final browser = await puppeteer.launch(headless: true, timeout: const Duration(minutes: 5));

  print('executeTestWeb: browser.newPage');
  final page = await browser.newPage();

  print('executeTestWeb: page.goto($addr)');
  await page.goto(addr);

  return browser;
}
