import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge_utils/src/dart_web_test_utils/run_test.dart';
import 'package:flutter_rust_bridge_utils/src/dart_web_test_utils/static_content.dart';
import 'package:puppeteer/puppeteer.dart' hide Response;
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart' as shelf_io;
import 'package:shelf_web_socket/shelf_web_socket.dart';
import 'package:test/test.dart';

void main() {
  group('browserExecutablePathFromEnvironment', () {
    test('prefers puppeteer executable path over Chrome aliases', () {
      expect(
        browserExecutablePathFromEnvironment({
          'PUPPETEER_EXECUTABLE_PATH': '/tmp/puppeteer-chrome',
          'CHROME_EXECUTABLE': '/tmp/chrome-executable',
          'CHROME_BIN': '/tmp/chrome-bin',
        }, isInContainer: false),
        '/tmp/puppeteer-chrome',
      );
    });

    test('uses Chrome executable alias when puppeteer path is absent', () {
      expect(
        browserExecutablePathFromEnvironment({
          'CHROME_EXECUTABLE': '/tmp/chrome-executable',
          'CHROME_BIN': '/tmp/chrome-bin',
        }, isInContainer: false),
        '/tmp/chrome-executable',
      );
    });

    test('ignores Chrome bin alias outside containers', () {
      expect(
        browserExecutablePathFromEnvironment({
          'PUPPETEER_EXECUTABLE_PATH': ' ',
          'CHROME_EXECUTABLE': '',
          'CHROME_BIN': '/tmp/chrome-bin',
        }, isInContainer: false),
        isNull,
      );
    });

    test('uses Chrome bin alias inside containers', () {
      expect(
        browserExecutablePathFromEnvironment({
          'PUPPETEER_EXECUTABLE_PATH': ' ',
          'CHROME_EXECUTABLE': '',
          'CHROME_BIN': '/tmp/chrome-bin',
        }, isInContainer: true),
        '/tmp/chrome-bin',
      );
    });
  });

  group('testEntrypointHtmlContent', () {
    test('uses JavaScript entrypoint for dart2js runs', () {
      final html = testEntrypointHtmlContent(kJsEntrypointScript);

      expect(html, contains('sendTestResult'));
      expect(html, contains('main.dart.js'));
      expect(html, isNot(contains('main.dart.mjs')));
    });

    test('uses module loader for dart2wasm runs', () {
      final html = testEntrypointHtmlContent(kWasmEntrypointScript);

      expect(html, contains('sendTestResult'));
      expect(html, contains('type="module"'));
      expect(html, contains('main.dart.mjs'));
      expect(html, contains('main.dart.wasm'));
      expect(html, contains('catch (error)'));
      expect(html, contains('sendTestResult(false)'));
      expect(html, isNot(contains('main.dart.js')));
    });

    test('reports failure when the wasm module loader is missing', () async {
      final result = Completer<bool>();
      final handler = Cascade().add(
        webSocketHandler((channel) {
          channel.stream.listen((message) {
            final decoded = jsonDecode(message as String);
            if (decoded is Map && decoded.containsKey(kTestResultKey)) {
              result.complete(decoded[kTestResultKey] as bool);
            }
          });
        }),
      ).add((request) {
        if (request.url.path == 'test_entrypoint.html') {
          return Response.ok(
            testEntrypointHtmlContent(kWasmEntrypointScript),
            headers: {HttpHeaders.contentTypeHeader: 'text/html'},
          );
        }
        return Response.notFound(null);
      }).handler;
      final server = await shelf_io.serve(
        handler,
        InternetAddress.loopbackIPv4,
        0,
      );
      Browser? browser;

      try {
        browser = await puppeteer.launch(
          executablePath: browserExecutablePathFromEnvironment(
            Platform.environment,
            isInContainer: File('/.dockerenv').existsSync(),
          ),
          headless: true,
          args: File('/.dockerenv').existsSync()
              ? ['--no-sandbox', '--disable-setuid-sandbox']
              : [],
        );
        final page = await browser.newPage();
        await page.goto(
          'http://${server.address.address}:${server.port}/test_entrypoint.html',
        );

        expect(
          await result.future.timeout(const Duration(seconds: 10)),
          isFalse,
        );
      } finally {
        await browser?.close();
        await server.close(force: true);
      }
    });
  });
}
