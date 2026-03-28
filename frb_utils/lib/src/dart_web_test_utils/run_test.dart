// ignore_for_file: avoid_print, implementation_imports

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
const _kLoopbackNoProxyValue = '127.0.0.1,localhost,::1';
const _kLoopbackHosts = {'127.0.0.1', 'localhost', '::1'};

Future<void> executeTestWeb(TestWebConfig config) async {
  final dartRoot = await findDartPackageDirectory(
    path.dirname(config.entrypoint),
  );
  final webRoot = '$dartRoot/web';
  print('executeTestWeb: Pick dartRoot=$dartRoot');

  List<String> cargoArgs = config.rustFeatures
      .expand((x) => ['--features', x])
      .toList();

  print('executeTestWeb: compile');
  await executeBuildWeb(
    BuildWebArgs(
      output: webRoot,
      release: false,
      verbose: false,
      // TODO make these configurable later when it is publicly used
      //      (now it is only used internally)
      rustCrateDir: '$dartRoot/rust',
      cargoBuildArgs: cargoArgs,
      wasmBindgenArgs: [],
      dartCompileJsEntrypoint: config.entrypoint,
      // TODO make this configurable later
      wasmPackRustupToolchain: 'nightly-2025-02-01',
      wasmPackRustflags: null,
    ),
  );

  Browser? browser;

  print('executeTestWeb: runServer');
  final baseAddr = await runServer(
    ServeWebConfig(
      webRoot: webRoot,
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
  browser = await _launchBrowser(baseAddr: baseAddr, headless: config.headless);
}

Handler _createWebSocketHandler({
  required Future<void> Function() closeBrowser,
}) {
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

const _kTestEntrypointHttpName = 'test_entrypoint.html';

Handler _createIndexFileHandler() => (request) {
  if (request.url.path == _kTestEntrypointHttpName) {
    return Response.ok(
      kTestEntrypointHtmlContent,
      headers: {HttpHeaders.contentTypeHeader: 'text/html'},
    );
  }
  return Response.notFound(null);
};

Future<Browser> _launchBrowser({
  required String baseAddr,
  required bool headless,
}) async {
  if (headless) {
    print(
      'Hint: Running browser in headless mode now. '
      'There are more logs on the browser console, so you can disable headless mode to debug.',
    );
  }

  // Check if running in Docker/container environment (no sandbox needed)
  final isInContainer = File('/.dockerenv').existsSync();

  return await HttpOverrides.runZoned(() async {
    final browser = await puppeteer.launch(
      headless: headless,
      timeout: const Duration(minutes: 5),
      args: isInContainer ? ['--no-sandbox', '--disable-setuid-sandbox'] : [],
      environment: _browserEnvironment(),
    );
    final page = await browser.newPage();
    _configurePageLogging(page);
    await page.goto('$baseAddr/$_kTestEntrypointHttpName');
    return browser;
  }, findProxyFromEnvironment: _findProxyFromEnvironment);
}

String _findProxyFromEnvironment(Uri uri, Map<String, String>? environment) {
  if (_kLoopbackHosts.contains(uri.host)) {
    return 'DIRECT';
  }

  return HttpClient.findProxyFromEnvironment(uri, environment);
}

Map<String, String> _browserEnvironment() {
  final result = Map<String, String>.from(Platform.environment);
  final noProxy = _mergeNoProxyValue(result['NO_PROXY'], result['no_proxy']);
  result['NO_PROXY'] = noProxy;
  result['no_proxy'] = noProxy;
  return result;
}

String _mergeNoProxyValue(String? upperValue, String? lowerValue) {
  final values = <String>{
    ..._splitNoProxyValue(upperValue),
    ..._splitNoProxyValue(lowerValue),
    ..._splitNoProxyValue(_kLoopbackNoProxyValue),
  };
  return values.join(',');
}

Iterable<String> _splitNoProxyValue(String? value) sync* {
  if (value == null) {
    return;
  }

  for (final item in value.split(',')) {
    final trimmed = item.trim();
    if (trimmed.isNotEmpty) {
      yield trimmed;
    }
  }
}

void _configurePageLogging(Page page) {
  // https://stackoverflow.com/questions/47539043/how-to-get-all-console-messages-with-puppeteer-including-errors-csp-violations

  page.onConsole.listen(
    (e) => print('[puppeteer console] [${e.typeName}] ${e.text?.trim()}'),
  );

  page.onPageCrashed.listen((_) => print('puppeteer.Page.onPageCrashed'));
  page.onRequestFailed.listen(
    (e) => print('puppeteer.Page.onRequestFailed $e'),
  );
  page.onError.listen((e) => print('puppeteer.Page.onError $e'));
}
