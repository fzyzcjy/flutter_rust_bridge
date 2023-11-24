import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/ffi/dart_cobject.dart' as dart_cobject;
import 'package:flutter_rust_bridge_example/bridge_definitions.dart';
import 'package:test/test.dart';

import '../../../../pure_dart/test/utils/test_flutter_memory_leak_utility.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'main.dart';

const isWeb = bool.fromEnvironment('dart.library.html');

String? skipWeb([String reason = 'unspecified']) => isWeb ? 'Skipped on web (reason: $reason)' : null;

void main(List<String> args) async {
  String dylibPath = args[0];
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');
  final api = initializeExternalLibrary(dylibPath);
}
