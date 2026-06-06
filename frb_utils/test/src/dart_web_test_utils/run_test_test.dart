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
}
