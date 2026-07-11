import 'package:flutter_rust_bridge_utils/src/dart_web_test_utils/run_test.dart';
import 'package:flutter_rust_bridge_utils/src/dart_web_test_utils/static_content.dart';
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
      expect(html, isNot(contains('main.dart.js')));
    });
  });
}
