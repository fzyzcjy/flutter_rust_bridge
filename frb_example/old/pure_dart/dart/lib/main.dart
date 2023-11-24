import 'dart:developer';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/test.dart';
import 'package:uuid/uuid.dart';

import 'bridge_definitions.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';

const isWeb = bool.fromEnvironment('dart.library.html');

String? skipWeb([String reason = 'unspecified']) => isWeb ? 'Skipped on web (reason: $reason)' : null;

void main(List<String> args) async {
  String dylibPath = args[0];
  var releaseMode = true;
  assert(() {
    releaseMode = false;
    return true;
  }());
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');
  final api = initializeExternalLibrary(dylibPath);
  tearDownAll(() => dispose());

  group('extended sync', () {
    test('void', () async {
      syncVoid();
    });
  });

  test("sync return mirror", () {
    final settings = syncReturnMirror();
    testAppSettings(settings);
  });
}

// vim:expandtab:ts=2:sw=2
