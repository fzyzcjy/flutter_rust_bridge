import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:flutter_rust_bridge_utils/src/commands/serve_web_command.dart';
import 'package:flutter_rust_bridge_utils/src/commands/test_web_command.dart';
import 'package:flutter_rust_bridge_utils/src/dart_web_test_utils/static_content.dart';
import 'package:flutter_rust_bridge_utils/src/serve_web/run_server.dart';
import 'package:path/path.dart' as path;
import 'package:puppeteer/puppeteer.dart' hide Response;
import 'package:shelf/shelf.dart';
import 'package:shelf_web_socket/shelf_web_socket.dart';

const kTestResultKey = '__result__';

Future<void> executeTestWeb(TestWebConfig config) async {
  final dartRoot = await findDartPackageDirectory(path.dirname(config.entrypoint));
  print('executeTestWeb: Pick dartRoot=$dartRoot');

  print('executeTestWeb: compile');
  await executeBuildWeb(BuildWebArgs(
    output: '$dartRoot/web/pkg',
    release: true,
    verbose: false,
    // TODO make these configurable later when it is publicly used
    //      (now it is only used internally)
    rustCrateDir: '$dartRoot/rust',
    cargoBuildArgs: [],
    wasmBindgenArgs: [],
    dartCompileJsEntrypoint: config.entrypoint,
  ));

  Browser? browser;

  print('executeTestWeb: runServer');
  final addr = await runServer(
    ServeWebConfig(
      webRoot: '$dartRoot/web',
      port: ServeWebConfig.kDefaultPort,
      open: false,
    ),
    extraHandlers: [
      _createWebSocketHandler(
        closeBrowser: () async {
          print('executeTestWeb: close browser');
          await browser?.close();
        },
      ),
      _createIndexFileHandler(),
    ],
  );

  print('executeTestWeb: launchBrowser');
  browser = await _launchBrowser(addr: addr, headless: config.headless);
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

Handler _createIndexFileHandler() =>
    (request) => Response.ok(kIndexHtmlContent, headers: {HttpHeaders.contentTypeHeader: 'text/html'});

Future<Browser> _launchBrowser({
  required String addr,
  required bool headless,
}) async {
  final browser = await puppeteer.launch(
    headless: headless,
    timeout: const Duration(minutes: 5),
  );
  final page = await browser.newPage();
  _configurePageLogging(page);
  await page.goto(addr);
  return browser;
}

void _configurePageLogging(Page page) {
  // https://stackoverflow.com/questions/47539043/how-to-get-all-console-messages-with-puppeteer-including-errors-csp-violations
  page.onConsole.listen((e) => print('puppeteer.Page.onConsole $e'));
  page.onPageCrashed.listen((_) => print('puppeteer.Page.onPageCrashed'));
  page.onRequestFailed.listen((e) => print('puppeteer.Page.onRequestFailed $e'));
  page.onError.listen((e) => print('puppeteer.Page.onError $e'));
}
