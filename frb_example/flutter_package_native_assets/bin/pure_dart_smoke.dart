import 'dart:io';

import 'package:flutter_package_native_assets/flutter_package_native_assets.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> main() async {
  final originalCurrentDirectory = Directory.current;
  late final ExternalLibrary externalLibrary;
  try {
    Directory.current = Directory.systemTemp;
    externalLibrary = await loadExternalLibrary(
      RustLib.kDefaultExternalLibraryLoaderConfig,
    );
  } finally {
    Directory.current = originalCurrentDirectory;
  }

  final normalizedDebugInfo = externalLibrary.debugInfo.replaceAll('\\', '/');
  if (!normalizedDebugInfo.contains('/.dart_tool/lib/')) {
    throw StateError('Unexpected library location: $normalizedDebugInfo');
  }

  await RustLib.init(externalLibrary: externalLibrary);

  final result = greet(name: 'Tom');
  if (result != 'Hello, Tom!') {
    throw StateError('Unexpected greeting: $result');
  }
}
