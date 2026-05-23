import 'package:flutter_rust_bridge_utils/src/dart_web_test_utils/run_test.dart';
import 'package:test/test.dart';

void main() {
  group('browserExecutablePathFromEnvironment', () {
    test('prefers puppeteer executable path over Chrome aliases', () {
      expect(
        browserExecutablePathFromEnvironment({
          'PUPPETEER_EXECUTABLE_PATH': '/tmp/puppeteer-chrome',
          'CHROME_EXECUTABLE': '/tmp/chrome-executable',
          'CHROME_BIN': '/tmp/chrome-bin',
        }),
        '/tmp/puppeteer-chrome',
      );
    });

    test('uses Chrome executable alias when puppeteer path is absent', () {
      expect(
        browserExecutablePathFromEnvironment({
          'CHROME_EXECUTABLE': '/tmp/chrome-executable',
          'CHROME_BIN': '/tmp/chrome-bin',
        }),
        '/tmp/chrome-executable',
      );
    });

    test('uses Chrome bin alias after ignoring blank values', () {
      expect(
        browserExecutablePathFromEnvironment({
          'PUPPETEER_EXECUTABLE_PATH': ' ',
          'CHROME_EXECUTABLE': '',
          'CHROME_BIN': '/tmp/chrome-bin',
        }),
        '/tmp/chrome-bin',
      );
    });
  });
}
